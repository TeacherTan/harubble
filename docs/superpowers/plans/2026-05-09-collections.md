# 合集功能（Collections）实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 为歌曲引入合集功能，支持官方预置合集（JSON 只读）和用户自定义合集（SQLite 读写），统一通过 CollectionService 对外暴露。

**Architecture:** 混合存储——官方合集作为 `data/official_collections.json` 随 app 发布，启动时加载到内存；用户合集存入现有 `siren_local.db`，通过 `CollectionService` 统一抽象两个数据源。前端通过 Tauri commands bridge 调用。

**Tech Stack:** Rust (rusqlite 0.34, serde, uuid), Tauri 2, TypeScript, Svelte 5

---

## 文件结构

| 操作 | 路径 | 职责 |
|------|------|------|
| Create | `data/official_collections.json` | 官方合集预置数据 |
| Create | `src-tauri/src/collection.rs` | CollectionService 实现 |
| Create | `src-tauri/src/commands/collection.rs` | Tauri command handlers |
| Modify | `src-tauri/src/commands/mod.rs` | 注册 collection 模块 |
| Modify | `src-tauri/src/app_state.rs` | 注册 CollectionService |
| Modify | `src-tauri/src/main.rs` | 注册 collection commands |
| Modify | `src-tauri/Cargo.toml` | 添加 uuid 依赖 |
| Create | `src/lib/collectionApi.ts` | 前端 bridge |
| Modify | `src/lib/types.ts` | 前端类型定义 |
| Create | `src-tauri/tests/collection_test.rs` | 集成测试 |

---

## Task 1: 添加 uuid 依赖 & 创建官方合集数据文件

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Create: `data/official_collections.json`

- [ ] **Step 1: 添加 uuid 依赖到 Cargo.toml**

在 `[dependencies]` 中添加：

```toml
uuid = { version = "1", features = ["v4"] }
```

- [ ] **Step 2: 创建官方合集数据文件**

```json
{
  "schemaVersion": 1,
  "collections": []
}
```

文件路径：`data/official_collections.json`，初始为空数组，后续由维护者手动添加合集条目。

- [ ] **Step 3: 验证 cargo check 通过**

Run: `cargo check --workspace`
Expected: 编译通过，无错误

- [ ] **Step 4: Commit**

```bash
git add src-tauri/Cargo.toml data/official_collections.json
git commit -m "chore: 添加 uuid 依赖与官方合集数据文件骨架"
```

---

## Task 2: 实现 CollectionService 核心结构与 Schema 初始化

**Files:**
- Create: `src-tauri/src/collection.rs`
- Modify: `src-tauri/src/lib.rs`（添加 `pub mod collection;`）

- [ ] **Step 1: 创建 collection.rs 并定义数据结构**

```rust
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const OFFICIAL_PREFIX: &str = "official:";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cover: Option<String>,
    pub song_count: usize,
    pub is_official: bool,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cover: Option<String>,
    pub song_ids: Vec<String>,
    pub is_official: bool,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

pub type LocalizedValue = HashMap<String, String>;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfficialCollectionEntry {
    pub id: String,
    pub name: LocalizedValue,
    pub description: LocalizedValue,
    pub cover: Option<String>,
    pub song_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfficialCollectionsFile {
    pub schema_version: u32,
    pub collections: Vec<OfficialCollectionEntry>,
}

#[derive(Clone)]
pub(crate) struct CollectionService {
    official: Arc<Vec<OfficialCollectionEntry>>,
    conn: Arc<Mutex<Connection>>,
}
```

- [ ] **Step 2: 实现 new() 和 initialize_schema()**

```rust
impl CollectionService {
    pub fn new(db_path: &Path, official_json: &[u8]) -> Result<Self, String> {
        let official_file: OfficialCollectionsFile = serde_json::from_slice(official_json)
            .map_err(|e| format!("解析官方合集数据失败: {e}"))?;

        let conn = Connection::open(db_path)
            .map_err(|e| format!("打开合集数据库失败: {e}"))?;

        let service = Self {
            official: Arc::new(official_file.collections),
            conn: Arc::new(Mutex::new(conn)),
        };

        service.initialize_schema()?;
        Ok(service)
    }

    fn initialize_schema(&self) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("获取数据库锁失败: {e}"))?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS collections (
                id          TEXT PRIMARY KEY,
                name        TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                cover_path  TEXT,
                created_at  INTEGER NOT NULL,
                updated_at  INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS collection_songs (
                collection_id TEXT NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
                song_id       TEXT NOT NULL,
                position      INTEGER NOT NULL,
                added_at      INTEGER NOT NULL,
                PRIMARY KEY (collection_id, song_id)
            );
            CREATE INDEX IF NOT EXISTS idx_collection_songs_pos
                ON collection_songs(collection_id, position);
            PRAGMA foreign_keys = ON;"
        ).map_err(|e| format!("初始化合集表失败: {e}"))?;
        Ok(())
    }
}
```

- [ ] **Step 3: 在 lib.rs 中声明模块**

在 `src-tauri/src/lib.rs` 的模块声明区域添加：

```rust
pub mod collection;
```

- [ ] **Step 4: 验证编译通过**

Run: `cargo check --workspace`
Expected: 编译通过

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/collection.rs src-tauri/src/lib.rs
git commit -m "feat(collection): 定义数据结构与 schema 初始化"
```

---

## Task 3: 实现 CollectionService 查询方法

**Files:**
- Modify: `src-tauri/src/collection.rs`

- [ ] **Step 1: 实现 resolve_locale 辅助方法**

```rust
impl CollectionService {
    fn resolve_locale(value: &LocalizedValue, locale: &str) -> String {
        value.get(locale)
            .or_else(|| value.get("zh-CN"))
            .or_else(|| value.get("en-US"))
            .or_else(|| value.values().next())
            .cloned()
            .unwrap_or_default()
    }
}
```

- [ ] **Step 2: 实现 list_all()**

```rust
impl CollectionService {
    pub fn list_all(&self, locale: &str) -> Result<Vec<CollectionSummary>, String> {
        let mut results: Vec<CollectionSummary> = self.official.iter().map(|entry| {
            CollectionSummary {
                id: entry.id.clone(),
                name: Self::resolve_locale(&entry.name, locale),
                description: Self::resolve_locale(&entry.description, locale),
                cover: entry.cover.clone(),
                song_count: entry.song_ids.len(),
                is_official: true,
                created_at: None,
                updated_at: None,
            }
        }).collect();

        let conn = self.conn.lock().map_err(|e| format!("获取数据库锁失败: {e}"))?;
        let mut stmt = conn.prepare(
            "SELECT c.id, c.name, c.description, c.cover_path, c.created_at, c.updated_at,
                    COUNT(cs.song_id) as song_count
             FROM collections c
             LEFT JOIN collection_songs cs ON cs.collection_id = c.id
             GROUP BY c.id
             ORDER BY c.updated_at DESC"
        ).map_err(|e| format!("查询合集列表失败: {e}"))?;

        let user_collections = stmt.query_map([], |row| {
            Ok(CollectionSummary {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                cover: row.get(3)?,
                created_at: Some(row.get(4)?),
                updated_at: Some(row.get(5)?),
                song_count: row.get::<_, i64>(6)? as usize,
                is_official: false,
            })
        }).map_err(|e| format!("查询合集列表失败: {e}"))?;

        for collection in user_collections {
            results.push(collection.map_err(|e| format!("读取合集记录失败: {e}"))?);
        }

        Ok(results)
    }
}
```

- [ ] **Step 3: 实现 get()**

```rust
impl CollectionService {
    pub fn get(&self, id: &str, locale: &str) -> Result<Collection, String> {
        if id.starts_with(OFFICIAL_PREFIX) {
            let entry = self.official.iter()
                .find(|e| e.id == id)
                .ok_or_else(|| format!("合集不存在: {id}"))?;
            return Ok(Collection {
                id: entry.id.clone(),
                name: Self::resolve_locale(&entry.name, locale),
                description: Self::resolve_locale(&entry.description, locale),
                cover: entry.cover.clone(),
                song_ids: entry.song_ids.clone(),
                is_official: true,
                created_at: None,
                updated_at: None,
            });
        }

        let conn = self.conn.lock().map_err(|e| format!("获取数据库锁失败: {e}"))?;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, cover_path, created_at, updated_at
             FROM collections WHERE id = ?1"
        ).map_err(|e| format!("查询合集失败: {e}"))?;

        let collection = stmt.query_row([id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, i64>(5)?,
            ))
        }).map_err(|_| format!("合集不存在: {id}"))?;

        let mut song_stmt = conn.prepare(
            "SELECT song_id FROM collection_songs
             WHERE collection_id = ?1 ORDER BY position ASC"
        ).map_err(|e| format!("查询合集歌曲失败: {e}"))?;

        let song_ids: Vec<String> = song_stmt.query_map([id], |row| {
            row.get(0)
        }).map_err(|e| format!("查询合集歌曲失败: {e}"))?
        .filter_map(|r| r.ok())
        .collect();

        Ok(Collection {
            id: collection.0,
            name: collection.1,
            description: collection.2,
            cover: collection.3,
            song_ids,
            is_official: false,
            created_at: Some(collection.4),
            updated_at: Some(collection.5),
        })
    }
}
```

- [ ] **Step 4: 验证编译通过**

Run: `cargo check --workspace`
Expected: 编译通过

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/collection.rs
git commit -m "feat(collection): 实现 list_all 与 get 查询方法"
```

---

## Task 4: 实现 CollectionService 写入方法（CRUD）

**Files:**
- Modify: `src-tauri/src/collection.rs`

- [ ] **Step 1: 实现 guard_not_official 辅助方法**

```rust
impl CollectionService {
    fn guard_not_official(id: &str) -> Result<(), String> {
        if id.starts_with(OFFICIAL_PREFIX) {
            return Err("官方合集不可修改".to_string());
        }
        Ok(())
    }
}
```

- [ ] **Step 2: 实现 create()**

```rust
impl CollectionService {
    pub fn create(&self, name: &str, description: &str, cover_path: Option<&str>) -> Result<Collection, String> {
        let id = Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("获取时间戳失败: {e}"))?
            .as_millis() as i64;

        let conn = self.conn.lock().map_err(|e| format!("获取数据库锁失败: {e}"))?;
        conn.execute(
            "INSERT INTO collections (id, name, description, cover_path, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![id, name, description, cover_path, now, now],
        ).map_err(|e| format!("创建合集失败: {e}"))?;

        Ok(Collection {
            id,
            name: name.to_string(),
            description: description.to_string(),
            cover: cover_path.map(|s| s.to_string()),
            song_ids: vec![],
            is_official: false,
            created_at: Some(now),
            updated_at: Some(now),
        })
    }
}
```

- [ ] **Step 3: 实现 update()**

```rust
impl CollectionService {
    pub fn update(
        &self,
        id: &str,
        name: Option<&str>,
        description: Option<&str>,
        cover_path: Option<Option<&str>>,
    ) -> Result<Collection, String> {
        Self::guard_not_official(id)?;

        let conn = self.conn.lock().map_err(|e| format!("获取数据库锁失败: {e}"))?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("获取时间戳失败: {e}"))?
            .as_millis() as i64;

        if let Some(name) = name {
            conn.execute(
                "UPDATE collections SET name = ?1, updated_at = ?2 WHERE id = ?3",
                rusqlite::params![name, now, id],
            ).map_err(|e| format!("更新合集名称失败: {e}"))?;
        }
        if let Some(desc) = description {
            conn.execute(
                "UPDATE collections SET description = ?1, updated_at = ?2 WHERE id = ?3",
                rusqlite::params![desc, now, id],
            ).map_err(|e| format!("更新合集描述失败: {e}"))?;
        }
        if let Some(cover) = cover_path {
            conn.execute(
                "UPDATE collections SET cover_path = ?1, updated_at = ?2 WHERE id = ?3",
                rusqlite::params![cover, now, id],
            ).map_err(|e| format!("更新合集封面失败: {e}"))?;
        }

        drop(conn);
        self.get(id, "zh-CN")
    }
}
```

- [ ] **Step 4: 实现 delete()**

```rust
impl CollectionService {
    pub fn delete(&self, id: &str) -> Result<(), String> {
        Self::guard_not_official(id)?;

        let conn = self.conn.lock().map_err(|e| format!("获取数据库锁失败: {e}"))?;
        let affected = conn.execute(
            "DELETE FROM collections WHERE id = ?1",
            rusqlite::params![id],
        ).map_err(|e| format!("删除合集失败: {e}"))?;

        if affected == 0 {
            return Err(format!("合集不存在: {id}"));
        }
        Ok(())
    }
}
```

- [ ] **Step 5: 验证编译通过**

Run: `cargo check --workspace`
Expected: 编译通过

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/collection.rs
git commit -m "feat(collection): 实现 create/update/delete 写入方法"
```

---

## Task 5: 实现歌曲管理方法（添加、移除、排序）

**Files:**
- Modify: `src-tauri/src/collection.rs`

- [ ] **Step 1: 实现 add_songs()**

```rust
impl CollectionService {
    pub fn add_songs(&self, id: &str, song_ids: &[String]) -> Result<(), String> {
        Self::guard_not_official(id)?;

        let conn = self.conn.lock().map_err(|e| format!("获取数据库锁失败: {e}"))?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("获取时间戳失败: {e}"))?
            .as_millis() as i64;

        let max_pos: i64 = conn.query_row(
            "SELECT COALESCE(MAX(position), -1) FROM collection_songs WHERE collection_id = ?1",
            rusqlite::params![id],
            |row| row.get(0),
        ).map_err(|e| format!("查询最大位置失败: {e}"))?;

        let mut position = max_pos + 1;
        for song_id in song_ids {
            conn.execute(
                "INSERT OR IGNORE INTO collection_songs (collection_id, song_id, position, added_at)
                 VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![id, song_id, position, now],
            ).map_err(|e| format!("添加歌曲失败: {e}"))?;
            position += 1;
        }

        conn.execute(
            "UPDATE collections SET updated_at = ?1 WHERE id = ?2",
            rusqlite::params![now, id],
        ).map_err(|e| format!("更新合集时间戳失败: {e}"))?;

        Ok(())
    }
}
```

- [ ] **Step 2: 实现 remove_songs()**

```rust
impl CollectionService {
    pub fn remove_songs(&self, id: &str, song_ids: &[String]) -> Result<(), String> {
        Self::guard_not_official(id)?;

        let conn = self.conn.lock().map_err(|e| format!("获取数据库锁失败: {e}"))?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("获取时间戳失败: {e}"))?
            .as_millis() as i64;

        for song_id in song_ids {
            conn.execute(
                "DELETE FROM collection_songs WHERE collection_id = ?1 AND song_id = ?2",
                rusqlite::params![id, song_id],
            ).map_err(|e| format!("移除歌曲失败: {e}"))?;
        }

        conn.execute(
            "UPDATE collections SET updated_at = ?1 WHERE id = ?2",
            rusqlite::params![now, id],
        ).map_err(|e| format!("更新合集时间戳失败: {e}"))?;

        Ok(())
    }
}
```

- [ ] **Step 3: 实现 reorder_songs()**

```rust
impl CollectionService {
    pub fn reorder_songs(&self, id: &str, song_ids: &[String]) -> Result<(), String> {
        Self::guard_not_official(id)?;

        let conn = self.conn.lock().map_err(|e| format!("获取数据库锁失败: {e}"))?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("获取时间戳失败: {e}"))?
            .as_millis() as i64;

        let tx = conn.unchecked_transaction()
            .map_err(|e| format!("开启事务失败: {e}"))?;

        for (position, song_id) in song_ids.iter().enumerate() {
            tx.execute(
                "UPDATE collection_songs SET position = ?1
                 WHERE collection_id = ?2 AND song_id = ?3",
                rusqlite::params![position as i64, id, song_id],
            ).map_err(|e| format!("更新歌曲位置失败: {e}"))?;
        }

        tx.execute(
            "UPDATE collections SET updated_at = ?1 WHERE id = ?2",
            rusqlite::params![now, id],
        ).map_err(|e| format!("更新合集时间戳失败: {e}"))?;

        tx.commit().map_err(|e| format!("提交事务失败: {e}"))?;
        Ok(())
    }
}
```

- [ ] **Step 4: 验证编译通过**

Run: `cargo check --workspace`
Expected: 编译通过

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/collection.rs
git commit -m "feat(collection): 实现歌曲添加、移除与排序方法"
```

---

## Task 6: 实现导出/导入方法

**Files:**
- Modify: `src-tauri/src/collection.rs`

- [ ] **Step 1: 定义导出/导入 JSON 结构**

```rust
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportedCollection {
    pub schema_version: u32,
    pub collection: ExportedCollectionData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportedCollectionData {
    pub id: String,
    pub name: LocalizedValue,
    pub description: LocalizedValue,
    pub cover: Option<String>,
    pub song_ids: Vec<String>,
}
```

- [ ] **Step 2: 实现 export()**

```rust
impl CollectionService {
    pub fn export(&self, id: &str, locale: &str) -> Result<String, String> {
        let collection = self.get(id, locale)?;

        let mut name_map = HashMap::new();
        name_map.insert("zh-CN".to_string(), collection.name);

        let mut desc_map = HashMap::new();
        desc_map.insert("zh-CN".to_string(), collection.description);

        let exported = ExportedCollection {
            schema_version: 1,
            collection: ExportedCollectionData {
                id: collection.id,
                name: name_map,
                description: desc_map,
                cover: collection.cover,
                song_ids: collection.song_ids,
            },
        };

        serde_json::to_string_pretty(&exported)
            .map_err(|e| format!("序列化合集失败: {e}"))
    }
}
```

- [ ] **Step 3: 实现 import()**

```rust
impl CollectionService {
    pub fn import(&self, json: &str) -> Result<Collection, String> {
        let exported: ExportedCollection = serde_json::from_str(json)
            .map_err(|e| format!("导入格式无效: {e}"))?;

        if exported.schema_version > 1 {
            return Err(format!("不支持的合集版本: {}", exported.schema_version));
        }

        let name = Self::resolve_locale(&exported.collection.name, "zh-CN");
        let description = Self::resolve_locale(&exported.collection.description, "zh-CN");

        let collection = self.create(&name, &description, exported.collection.cover.as_deref())?;

        if !exported.collection.song_ids.is_empty() {
            self.add_songs(&collection.id, &exported.collection.song_ids)?;
        }

        self.get(&collection.id, "zh-CN")
    }
}
```

- [ ] **Step 4: 验证编译通过**

Run: `cargo check --workspace`
Expected: 编译通过

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/collection.rs
git commit -m "feat(collection): 实现导出与导入方法"
```

---

## Task 7: 注册 CollectionService 到 AppState

**Files:**
- Modify: `src-tauri/src/app_state.rs`

- [ ] **Step 1: 在 AppState 结构体中添加字段**

在 `AppState` 的字段列表中添加：

```rust
pub(crate) collection: CollectionService,
```

并在文件顶部添加 import：

```rust
use crate::collection::CollectionService;
```

- [ ] **Step 2: 在 AppState::new() 中初始化 CollectionService**

在其他服务初始化之后（如 `album_metadata_cache` 之后）添加：

```rust
let official_collections_bytes = include_bytes!("../../data/official_collections.json");
let collection = CollectionService::new(&db_path, official_collections_bytes)
    .map_err(|e| format!("初始化合集服务失败: {e}"))?;
```

并在 `AppState` 构造表达式中添加 `collection` 字段。

- [ ] **Step 3: 验证编译通过**

Run: `cargo check --workspace`
Expected: 编译通过

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/app_state.rs
git commit -m "feat(collection): 注册 CollectionService 到 AppState"
```

---

## Task 8: 实现 Tauri Command Handlers

**Files:**
- Create: `src-tauri/src/commands/collection.rs`
- Modify: `src-tauri/src/commands/mod.rs`

- [ ] **Step 1: 创建 commands/collection.rs**

```rust
use tauri::State;

use crate::app_state::AppState;
use crate::collection::{Collection, CollectionSummary};

#[tauri::command]
pub fn list_collections(state: State<'_, AppState>) -> Result<Vec<CollectionSummary>, String> {
    let locale = state.preferences().locale.clone();
    state.collection.list_all(&locale)
}

#[tauri::command]
pub fn get_collection(state: State<'_, AppState>, id: String) -> Result<Collection, String> {
    let locale = state.preferences().locale.clone();
    state.collection.get(&id, &locale)
}

#[tauri::command]
pub fn create_collection(
    state: State<'_, AppState>,
    name: String,
    description: String,
    cover_path: Option<String>,
) -> Result<Collection, String> {
    state.collection.create(&name, &description, cover_path.as_deref())
}

#[tauri::command]
pub fn update_collection(
    state: State<'_, AppState>,
    id: String,
    name: Option<String>,
    description: Option<String>,
    cover_path: Option<Option<String>>,
) -> Result<Collection, String> {
    state.collection.update(
        &id,
        name.as_deref(),
        description.as_deref(),
        cover_path.as_ref().map(|o| o.as_deref()),
    )
}

#[tauri::command]
pub fn delete_collection(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state.collection.delete(&id)
}

#[tauri::command]
pub fn add_songs_to_collection(
    state: State<'_, AppState>,
    id: String,
    song_ids: Vec<String>,
) -> Result<(), String> {
    state.collection.add_songs(&id, &song_ids)
}

#[tauri::command]
pub fn remove_songs_from_collection(
    state: State<'_, AppState>,
    id: String,
    song_ids: Vec<String>,
) -> Result<(), String> {
    state.collection.remove_songs(&id, &song_ids)
}

#[tauri::command]
pub fn reorder_collection_songs(
    state: State<'_, AppState>,
    id: String,
    song_ids: Vec<String>,
) -> Result<(), String> {
    state.collection.reorder_songs(&id, &song_ids)
}

#[tauri::command]
pub fn export_collection(state: State<'_, AppState>, id: String) -> Result<String, String> {
    let locale = state.preferences().locale.clone();
    state.collection.export(&id, &locale)
}

#[tauri::command]
pub fn import_collection(state: State<'_, AppState>, json: String) -> Result<Collection, String> {
    state.collection.import(&json)
}
```

- [ ] **Step 2: 在 commands/mod.rs 中注册模块**

添加：

```rust
pub mod collection;
```

- [ ] **Step 3: 验证编译通过**

Run: `cargo check --workspace`
Expected: 编译通过

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/commands/collection.rs src-tauri/src/commands/mod.rs
git commit -m "feat(collection): 实现 Tauri command handlers"
```

---

## Task 9: 注册 Commands 到 Tauri invoke_handler

**Files:**
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 1: 在 generate_handler! 宏中添加 collection commands**

在 `tauri::generate_handler!` 列表中添加：

```rust
commands::collection::list_collections,
commands::collection::get_collection,
commands::collection::create_collection,
commands::collection::update_collection,
commands::collection::delete_collection,
commands::collection::add_songs_to_collection,
commands::collection::remove_songs_from_collection,
commands::collection::reorder_collection_songs,
commands::collection::export_collection,
commands::collection::import_collection,
```

- [ ] **Step 2: 验证编译通过**

Run: `cargo check --workspace`
Expected: 编译通过

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/main.rs
git commit -m "feat(collection): 注册 commands 到 Tauri invoke handler"
```

---

## Task 10: 前端类型定义与 API Bridge

**Files:**
- Modify: `src/lib/types.ts`
- Create: `src/lib/collectionApi.ts`

- [ ] **Step 1: 在 types.ts 中添加合集类型**

在文件末尾添加：

```typescript
export interface CollectionSummary {
  id: string;
  name: string;
  description: string;
  cover: string | null;
  songCount: number;
  isOfficial: boolean;
  createdAt: number | null;
  updatedAt: number | null;
}

export interface Collection extends CollectionSummary {
  songIds: string[];
}
```

- [ ] **Step 2: 创建 collectionApi.ts**

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { Collection, CollectionSummary } from './types';

export async function listCollections(): Promise<CollectionSummary[]> {
  return invoke('list_collections');
}

export async function getCollection(id: string): Promise<Collection> {
  return invoke('get_collection', { id });
}

export async function createCollection(
  name: string,
  description: string,
  coverPath?: string | null,
): Promise<Collection> {
  return invoke('create_collection', { name, description, coverPath: coverPath ?? null });
}

export async function updateCollection(
  id: string,
  name?: string | null,
  description?: string | null,
  coverPath?: string | null | undefined,
): Promise<Collection> {
  return invoke('update_collection', {
    id,
    name: name ?? null,
    description: description ?? null,
    coverPath: coverPath === undefined ? null : coverPath,
  });
}

export async function deleteCollection(id: string): Promise<void> {
  return invoke('delete_collection', { id });
}

export async function addSongsToCollection(id: string, songIds: string[]): Promise<void> {
  return invoke('add_songs_to_collection', { id, songIds });
}

export async function removeSongsFromCollection(id: string, songIds: string[]): Promise<void> {
  return invoke('remove_songs_from_collection', { id, songIds });
}

export async function reorderCollectionSongs(id: string, songIds: string[]): Promise<void> {
  return invoke('reorder_collection_songs', { id, songIds });
}

export async function exportCollection(id: string): Promise<string> {
  return invoke('export_collection', { id });
}

export async function importCollection(json: string): Promise<Collection> {
  return invoke('import_collection', { json });
}
```

- [ ] **Step 3: 验证前端类型检查通过**

Run: `bun run check`
Expected: 无类型错误

- [ ] **Step 4: Commit**

```bash
git add src/lib/types.ts src/lib/collectionApi.ts
git commit -m "feat(collection): 添加前端类型定义与 API bridge"
```

---

## Task 11: 集成测试

**Files:**
- Create: `src-tauri/tests/collection_test.rs`

- [ ] **Step 1: 编写集成测试**

```rust
use std::path::PathBuf;
use tempfile::TempDir;

fn create_service() -> (siren_music_download::collection::CollectionService, TempDir) {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("test.db");
    let official_json = br#"{"schemaVersion":1,"collections":[{"id":"official:test","name":{"zh-CN":"测试合集","en-US":"Test Collection"},"description":{"zh-CN":"描述","en-US":"Description"},"cover":null,"songIds":["song-1","song-2"]}]}"#;
    let service = siren_music_download::collection::CollectionService::new(&db_path, official_json).unwrap();
    (service, tmp)
}

#[test]
fn test_list_all_includes_official() {
    let (service, _tmp) = create_service();
    let list = service.list_all("zh-CN").unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].name, "测试合集");
    assert!(list[0].is_official);
    assert_eq!(list[0].song_count, 2);
}

#[test]
fn test_get_official_collection() {
    let (service, _tmp) = create_service();
    let col = service.get("official:test", "en-US").unwrap();
    assert_eq!(col.name, "Test Collection");
    assert_eq!(col.song_ids, vec!["song-1", "song-2"]);
    assert!(col.is_official);
}

#[test]
fn test_create_and_get_user_collection() {
    let (service, _tmp) = create_service();
    let created = service.create("我的合集", "自定义描述", None).unwrap();
    assert!(!created.is_official);
    assert!(!created.id.starts_with("official:"));
    assert_eq!(created.name, "我的合集");

    let fetched = service.get(&created.id, "zh-CN").unwrap();
    assert_eq!(fetched.name, "我的合集");
    assert_eq!(fetched.song_ids.len(), 0);
}

#[test]
fn test_add_and_reorder_songs() {
    let (service, _tmp) = create_service();
    let col = service.create("排序测试", "", None).unwrap();

    service.add_songs(&col.id, &["a".into(), "b".into(), "c".into()]).unwrap();
    let fetched = service.get(&col.id, "zh-CN").unwrap();
    assert_eq!(fetched.song_ids, vec!["a", "b", "c"]);

    service.reorder_songs(&col.id, &["c".into(), "a".into(), "b".into()]).unwrap();
    let reordered = service.get(&col.id, "zh-CN").unwrap();
    assert_eq!(reordered.song_ids, vec!["c", "a", "b"]);
}

#[test]
fn test_remove_songs() {
    let (service, _tmp) = create_service();
    let col = service.create("移除测试", "", None).unwrap();
    service.add_songs(&col.id, &["x".into(), "y".into(), "z".into()]).unwrap();

    service.remove_songs(&col.id, &["y".into()]).unwrap();
    let fetched = service.get(&col.id, "zh-CN").unwrap();
    assert_eq!(fetched.song_ids, vec!["x", "z"]);
}

#[test]
fn test_delete_collection_cascades() {
    let (service, _tmp) = create_service();
    let col = service.create("删除测试", "", None).unwrap();
    service.add_songs(&col.id, &["s1".into()]).unwrap();

    service.delete(&col.id).unwrap();
    assert!(service.get(&col.id, "zh-CN").is_err());
}

#[test]
fn test_official_collection_is_readonly() {
    let (service, _tmp) = create_service();
    assert!(service.add_songs("official:test", &["new".into()]).is_err());
    assert!(service.delete("official:test").is_err());
    assert!(service.update("official:test", Some("new name"), None, None).is_err());
}

#[test]
fn test_export_and_import() {
    let (service, _tmp) = create_service();
    let col = service.create("导出测试", "描述", None).unwrap();
    service.add_songs(&col.id, &["s1".into(), "s2".into()]).unwrap();

    let json = service.export(&col.id, "zh-CN").unwrap();
    let imported = service.import(&json).unwrap();

    assert_ne!(imported.id, col.id);
    assert_eq!(imported.name, "导出测试");
    assert_eq!(imported.song_ids, vec!["s1", "s2"]);
}

#[test]
fn test_duplicate_song_ignored() {
    let (service, _tmp) = create_service();
    let col = service.create("去重测试", "", None).unwrap();
    service.add_songs(&col.id, &["a".into(), "b".into()]).unwrap();
    service.add_songs(&col.id, &["b".into(), "c".into()]).unwrap();

    let fetched = service.get(&col.id, "zh-CN").unwrap();
    assert_eq!(fetched.song_ids.len(), 3);
}
```

- [ ] **Step 2: 确保 collection 模块对测试可见**

在 `src-tauri/src/lib.rs` 中确认 `pub mod collection;` 已存在（Task 2 已添加）。

- [ ] **Step 3: 运行测试**

Run: `cargo test --package siren-music-download --test collection_test`
Expected: 所有测试通过

- [ ] **Step 4: Commit**

```bash
git add src-tauri/tests/collection_test.rs
git commit -m "test(collection): 添加集成测试覆盖核心功能"
```

---

## Task 12: 格式化与最终验证

**Files:**
- All modified files

- [ ] **Step 1: Rust 格式化**

Run: `cargo fmt --all`

- [ ] **Step 2: Clippy 检查**

Run: `cargo clippy --workspace --all-targets`
Expected: 无 warning（或仅已有 warning）

- [ ] **Step 3: 前端格式化与检查**

Run: `bun run format && bun run check`
Expected: 通过

- [ ] **Step 4: 全量测试**

Run: `cargo test --workspace`
Expected: 所有测试通过

- [ ] **Step 5: Commit（如有格式化变更）**

```bash
git add -A
git commit -m "chore: 格式化与 lint 修复"
```
