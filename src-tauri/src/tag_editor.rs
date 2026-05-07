use crate::tag_registry::{
    LocalizedValue, TagDimension, TagRegistry, TagSet, CURRENT_SCHEMA_VERSION,
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use tempfile::NamedTempFile;

const REMOTE_FILE_NAME: &str = "tag_registry_remote.json";
const LOCAL_FILE_NAME: &str = "tag_registry_local.json";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum EntityType {
    Album,
    Song,
}

#[derive(Clone)]
pub(crate) struct TagEditorService {
    remote: Arc<RwLock<TagRegistry>>,
    local: Arc<RwLock<TagRegistry>>,
    remote_path: PathBuf,
    local_path: PathBuf,
}

impl TagEditorService {
    pub(crate) fn new(app_data_dir: &Path) -> Self {
        let remote_path = app_data_dir.join(REMOTE_FILE_NAME);
        let local_path = app_data_dir.join(LOCAL_FILE_NAME);
        let remote = load_registry(&remote_path).unwrap_or_else(empty_registry);
        let local = load_registry(&local_path).unwrap_or_else(empty_registry);
        Self {
            remote: Arc::new(RwLock::new(remote)),
            local: Arc::new(RwLock::new(local)),
            remote_path,
            local_path,
        }
    }

    pub(crate) fn remote_registry(&self) -> std::sync::RwLockReadGuard<'_, TagRegistry> {
        self.remote
            .read()
            .expect("tag_editor remote RwLock poisoned")
    }

    pub(crate) fn local_registry(&self) -> std::sync::RwLockReadGuard<'_, TagRegistry> {
        self.local.read().expect("tag_editor local RwLock poisoned")
    }
    pub(crate) fn set_entity_tag(
        &self,
        entity_type: EntityType,
        cid: &str,
        dimension_key: &str,
        values: Vec<LocalizedValue>,
    ) -> Result<()> {
        {
            let mut local = self
                .local
                .write()
                .expect("tag_editor local RwLock poisoned");
            let map = match entity_type {
                EntityType::Album => &mut local.albums,
                EntityType::Song => &mut local.songs,
            };
            let tag_set = map.entry(cid.to_string()).or_default();
            tag_set.tags.insert(dimension_key.to_string(), values);
        }
        self.persist_local()
    }

    pub(crate) fn remove_entity_tag(
        &self,
        entity_type: EntityType,
        cid: &str,
        dimension_key: &str,
    ) -> Result<()> {
        {
            let mut local = self
                .local
                .write()
                .expect("tag_editor local RwLock poisoned");
            let map = match entity_type {
                EntityType::Album => &mut local.albums,
                EntityType::Song => &mut local.songs,
            };
            if let Some(tag_set) = map.get_mut(cid) {
                tag_set.tags.remove(dimension_key);
                if tag_set.tags.is_empty() {
                    map.remove(cid);
                }
            }
        }
        self.persist_local()
    }

    pub(crate) fn add_local_dimension(
        &self,
        key: &str,
        label_zh: &str,
        label_en: &str,
    ) -> Result<()> {
        {
            let remote = self.remote.read().expect("poisoned");
            if remote.tag_dimensions.iter().any(|d| d.key == key) {
                anyhow::bail!("维度 key '{key}' 已存在于远端注册表中");
            }
            let mut local = self.local.write().expect("poisoned");
            if local.tag_dimensions.iter().any(|d| d.key == key) {
                anyhow::bail!("维度 key '{key}' 已存在于本地注册表中");
            }
            local.tag_dimensions.push(TagDimension {
                key: key.to_string(),
                label: HashMap::from([
                    ("zh-CN".to_string(), label_zh.to_string()),
                    ("en-US".to_string(), label_en.to_string()),
                ]),
            });
        }
        self.persist_local()
    }
    pub(crate) fn remove_local_dimension(&self, key: &str) -> Result<()> {
        {
            let mut local = self.local.write().expect("poisoned");
            local.tag_dimensions.retain(|d| d.key != key);
            for tag_set in local.albums.values_mut() {
                tag_set.tags.remove(key);
            }
            for tag_set in local.songs.values_mut() {
                tag_set.tags.remove(key);
            }
            local.albums.retain(|_, ts| !ts.tags.is_empty());
            local.songs.retain(|_, ts| !ts.tags.is_empty());
        }
        self.persist_local()
    }

    pub(crate) fn compute_merged(&self) -> TagRegistry {
        let remote = self.remote.read().expect("poisoned");
        let local = self.local.read().expect("poisoned");

        let dimensions = merge_dimensions(&remote.tag_dimensions, &local.tag_dimensions);
        let albums = merge_entity_maps(&remote.albums, &local.albums);
        let songs = merge_entity_maps(&remote.songs, &local.songs);

        TagRegistry {
            schema_version: remote.schema_version.max(local.schema_version).max(1),
            updated_at: remote.updated_at.clone(),
            tag_dimensions: dimensions,
            albums,
            songs,
        }
    }

    fn persist_local(&self) -> Result<()> {
        let local = self.local.read().expect("tag_editor local RwLock poisoned");
        persist_registry(&self.local_path, &local)
    }

    #[allow(dead_code)]
    fn persist_remote(&self) -> Result<()> {
        let remote = self
            .remote
            .read()
            .expect("tag_editor remote RwLock poisoned");
        persist_registry(&self.remote_path, &remote)
    }
}
fn empty_registry() -> TagRegistry {
    TagRegistry {
        schema_version: CURRENT_SCHEMA_VERSION,
        ..Default::default()
    }
}

fn load_registry(path: &Path) -> Option<TagRegistry> {
    if !path.exists() {
        return None;
    }
    let content = std::fs::read_to_string(path).ok()?;
    let registry: TagRegistry = serde_json::from_str(&content).ok()?;
    if registry.schema_version != CURRENT_SCHEMA_VERSION {
        return None;
    }
    Some(registry)
}

fn persist_registry(path: &Path, registry: &TagRegistry) -> Result<()> {
    let parent = path.parent().context("缓存文件路径无父目录")?;
    std::fs::create_dir_all(parent).context("创建缓存目录失败")?;
    let content = serde_json::to_vec_pretty(registry).context("序列化失败")?;
    let mut temp = NamedTempFile::new_in(parent).context("创建临时文件失败")?;
    temp.write_all(&content).context("写入临时文件失败")?;
    temp.as_file().sync_all().context("sync 失败")?;
    temp.persist(path)
        .map_err(|e| e.error)
        .context("原子重命名失败")?;
    Ok(())
}

fn merge_dimensions(remote: &[TagDimension], local: &[TagDimension]) -> Vec<TagDimension> {
    let mut result = remote.to_vec();
    let remote_keys: HashSet<&str> = remote.iter().map(|d| d.key.as_str()).collect();
    for dim in local {
        if !remote_keys.contains(dim.key.as_str()) {
            result.push(dim.clone());
        }
    }
    result
}

fn merge_entity_maps(
    remote: &HashMap<String, TagSet>,
    local: &HashMap<String, TagSet>,
) -> HashMap<String, TagSet> {
    let mut result: HashMap<String, TagSet> = HashMap::new();
    let all_cids: HashSet<&str> = remote
        .keys()
        .chain(local.keys())
        .map(|s| s.as_str())
        .collect();

    for cid in all_cids {
        let remote_set = remote.get(cid);
        let local_set = local.get(cid);
        let merged_set = merge_tag_sets(remote_set, local_set);
        if !merged_set.tags.is_empty() {
            result.insert(cid.to_string(), merged_set);
        }
    }
    result
}
fn merge_tag_sets(remote: Option<&TagSet>, local: Option<&TagSet>) -> TagSet {
    match (remote, local) {
        (None, None) => TagSet::default(),
        (Some(r), None) => r.clone(),
        (None, Some(l)) => l.clone(),
        (Some(r), Some(l)) => {
            let mut merged_tags: HashMap<String, Vec<LocalizedValue>> = HashMap::new();
            let all_keys: HashSet<&str> = r
                .tags
                .keys()
                .chain(l.tags.keys())
                .map(|s| s.as_str())
                .collect();
            for key in all_keys {
                let r_vals = r.tags.get(key);
                let l_vals = l.tags.get(key);
                let merged_vals = match (r_vals, l_vals) {
                    (None, None) => continue,
                    (Some(rv), None) => rv.clone(),
                    (None, Some(lv)) => lv.clone(),
                    (Some(rv), Some(lv)) => union_localized_values(rv, lv),
                };
                merged_tags.insert(key.to_string(), merged_vals);
            }
            TagSet { tags: merged_tags }
        }
    }
}

fn union_localized_values(a: &[LocalizedValue], b: &[LocalizedValue]) -> Vec<LocalizedValue> {
    let mut result = a.to_vec();
    for val in b {
        let duplicate = result.iter().any(|existing| existing.0 == val.0);
        if !duplicate {
            result.push(val.clone());
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_service_with_empty_dir_has_empty_layers() {
        let dir = tempfile::tempdir().unwrap();
        let svc = TagEditorService::new(dir.path());
        assert!(svc.remote_registry().albums.is_empty());
        assert!(svc.local_registry().albums.is_empty());
    }

    #[test]
    fn new_service_loads_existing_remote_cache() {
        let dir = tempfile::tempdir().unwrap();
        let remote = TagRegistry {
            schema_version: 1,
            updated_at: "2026-05-01T00:00:00Z".to_string(),
            tag_dimensions: vec![],
            albums: HashMap::from([("A1".into(), TagSet::default())]),
            songs: HashMap::new(),
        };
        let path = dir.path().join("tag_registry_remote.json");
        std::fs::write(&path, serde_json::to_vec_pretty(&remote).unwrap()).unwrap();
        let svc = TagEditorService::new(dir.path());
        assert!(svc.remote_registry().albums.contains_key("A1"));
    }

    #[test]
    fn new_service_loads_existing_local_overlay() {
        let dir = tempfile::tempdir().unwrap();
        let local = TagRegistry {
            schema_version: 1,
            updated_at: "".to_string(),
            tag_dimensions: vec![],
            albums: HashMap::new(),
            songs: HashMap::from([("S1".into(), TagSet::default())]),
        };
        let path = dir.path().join("tag_registry_local.json");
        std::fs::write(&path, serde_json::to_vec_pretty(&local).unwrap()).unwrap();
        let svc = TagEditorService::new(dir.path());
        assert!(svc.local_registry().songs.contains_key("S1"));
    }
    #[test]
    fn set_entity_tag_writes_to_local_overlay() {
        let dir = tempfile::tempdir().unwrap();
        let svc = TagEditorService::new(dir.path());
        let values = vec![LocalizedValue({
            let mut m = HashMap::new();
            m.insert("zh-CN".to_string(), "罗德岛".to_string());
            m
        })];
        svc.set_entity_tag(EntityType::Album, "A1", "faction", values)
            .unwrap();
        let local = svc.local_registry();
        let tag_set = local.albums.get("A1").unwrap();
        assert_eq!(tag_set.tags.get("faction").unwrap().len(), 1);
    }

    #[test]
    fn remove_entity_tag_deletes_from_local_overlay() {
        let dir = tempfile::tempdir().unwrap();
        let svc = TagEditorService::new(dir.path());
        let values = vec![LocalizedValue({
            let mut m = HashMap::new();
            m.insert("zh-CN".to_string(), "罗德岛".to_string());
            m
        })];
        svc.set_entity_tag(EntityType::Album, "A1", "faction", values)
            .unwrap();
        svc.remove_entity_tag(EntityType::Album, "A1", "faction")
            .unwrap();
        let local = svc.local_registry();
        assert!(local
            .albums
            .get("A1")
            .map_or(true, |ts| !ts.tags.contains_key("faction")));
    }

    #[test]
    fn set_entity_tag_persists_to_disk() {
        let dir = tempfile::tempdir().unwrap();
        let svc = TagEditorService::new(dir.path());
        let values = vec![LocalizedValue({
            let mut m = HashMap::new();
            m.insert("zh-CN".to_string(), "测试".to_string());
            m
        })];
        svc.set_entity_tag(EntityType::Song, "S1", "genre", values)
            .unwrap();
        let svc2 = TagEditorService::new(dir.path());
        let local = svc2.local_registry();
        assert!(local.songs.get("S1").unwrap().tags.contains_key("genre"));
    }
    #[test]
    fn add_local_dimension_appends_to_local_dimensions() {
        let dir = tempfile::tempdir().unwrap();
        let svc = TagEditorService::new(dir.path());
        svc.add_local_dimension("mood", "心情", "Mood").unwrap();
        let local = svc.local_registry();
        let dim = local
            .tag_dimensions
            .iter()
            .find(|d| d.key == "mood")
            .unwrap();
        assert_eq!(dim.label.get("zh-CN").unwrap(), "心情");
        assert_eq!(dim.label.get("en-US").unwrap(), "Mood");
    }

    #[test]
    fn add_local_dimension_rejects_duplicate_key() {
        let dir = tempfile::tempdir().unwrap();
        let svc = TagEditorService::new(dir.path());
        svc.add_local_dimension("mood", "心情", "Mood").unwrap();
        let result = svc.add_local_dimension("mood", "情绪", "Emotion");
        assert!(result.is_err());
    }

    #[test]
    fn remove_local_dimension_cleans_up_references() {
        let dir = tempfile::tempdir().unwrap();
        let svc = TagEditorService::new(dir.path());
        svc.add_local_dimension("mood", "心情", "Mood").unwrap();
        let values = vec![LocalizedValue({
            let mut m = HashMap::new();
            m.insert("zh-CN".to_string(), "欢快".to_string());
            m
        })];
        svc.set_entity_tag(EntityType::Song, "S1", "mood", values)
            .unwrap();
        svc.remove_local_dimension("mood").unwrap();
        let local = svc.local_registry();
        assert!(local.tag_dimensions.iter().all(|d| d.key != "mood"));
        assert!(local
            .songs
            .get("S1")
            .map_or(true, |ts| !ts.tags.contains_key("mood")));
    }
    #[test]
    fn compute_merged_combines_remote_and_local() {
        let dir = tempfile::tempdir().unwrap();
        let remote = TagRegistry {
            schema_version: 1,
            updated_at: "2026-05-01T00:00:00Z".to_string(),
            tag_dimensions: vec![TagDimension {
                key: "faction".to_string(),
                label: HashMap::from([("zh-CN".into(), "阵营".into())]),
            }],
            albums: HashMap::from([(
                "A1".into(),
                TagSet {
                    tags: HashMap::from([(
                        "faction".into(),
                        vec![LocalizedValue(HashMap::from([(
                            "zh-CN".into(),
                            "罗德岛".into(),
                        )]))],
                    )]),
                },
            )]),
            songs: HashMap::new(),
        };
        std::fs::write(
            dir.path().join("tag_registry_remote.json"),
            serde_json::to_vec_pretty(&remote).unwrap(),
        )
        .unwrap();

        let svc = TagEditorService::new(dir.path());
        svc.add_local_dimension("mood", "心情", "Mood").unwrap();
        svc.set_entity_tag(
            EntityType::Album,
            "A1",
            "mood",
            vec![LocalizedValue(HashMap::from([(
                "zh-CN".into(),
                "热血".into(),
            )]))],
        )
        .unwrap();

        let merged = svc.compute_merged();
        assert_eq!(merged.tag_dimensions.len(), 2);
        assert_eq!(merged.tag_dimensions[0].key, "faction");
        assert_eq!(merged.tag_dimensions[1].key, "mood");
        let a1 = merged.albums.get("A1").unwrap();
        assert!(a1.tags.contains_key("faction"));
        assert!(a1.tags.contains_key("mood"));
    }

    #[test]
    fn compute_merged_deduplicates_same_dimension_values() {
        let dir = tempfile::tempdir().unwrap();
        let shared_value = LocalizedValue(HashMap::from([("zh-CN".into(), "罗德岛".into())]));
        let remote = TagRegistry {
            schema_version: 1,
            updated_at: "".to_string(),
            tag_dimensions: vec![TagDimension {
                key: "faction".to_string(),
                label: HashMap::from([("zh-CN".into(), "阵营".into())]),
            }],
            albums: HashMap::from([(
                "A1".into(),
                TagSet {
                    tags: HashMap::from([("faction".into(), vec![shared_value.clone()])]),
                },
            )]),
            songs: HashMap::new(),
        };
        std::fs::write(
            dir.path().join("tag_registry_remote.json"),
            serde_json::to_vec_pretty(&remote).unwrap(),
        )
        .unwrap();

        let svc = TagEditorService::new(dir.path());
        svc.set_entity_tag(EntityType::Album, "A1", "faction", vec![shared_value])
            .unwrap();

        let merged = svc.compute_merged();
        let a1 = merged.albums.get("A1").unwrap();
        assert_eq!(a1.tags.get("faction").unwrap().len(), 1, "重复值应去重");
    }
}
