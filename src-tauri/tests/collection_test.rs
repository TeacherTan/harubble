use tempfile::TempDir;

fn create_service() -> (harubble::collection::CollectionService, TempDir) {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("test.db");
    let official_json = r#"{"schemaVersion":1,"collections":[{"id":"official:test","name":{"zh-CN":"测试合集","en-US":"Test Collection"},"description":{"zh-CN":"描述","en-US":"Description"},"cover":null,"songIds":["song-1","song-2"]}]}"#;
    let service =
        harubble::collection::CollectionService::new(&db_path, official_json.as_bytes()).unwrap();
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

    service
        .add_songs(&col.id, &["a".into(), "b".into(), "c".into()])
        .unwrap();
    let fetched = service.get(&col.id, "zh-CN").unwrap();
    assert_eq!(fetched.song_ids, vec!["a", "b", "c"]);

    service
        .reorder_songs(&col.id, &["c".into(), "a".into(), "b".into()])
        .unwrap();
    let reordered = service.get(&col.id, "zh-CN").unwrap();
    assert_eq!(reordered.song_ids, vec!["c", "a", "b"]);
}

#[test]
fn test_remove_songs() {
    let (service, _tmp) = create_service();
    let col = service.create("移除测试", "", None).unwrap();
    service
        .add_songs(&col.id, &["x".into(), "y".into(), "z".into()])
        .unwrap();

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
    assert!(service
        .update("official:test", Some("new name"), None, None)
        .is_err());
}

#[test]
fn test_export_and_import() {
    let (service, _tmp) = create_service();
    let col = service.create("导出测试", "描述", None).unwrap();
    service
        .add_songs(&col.id, &["s1".into(), "s2".into()])
        .unwrap();

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
    service
        .add_songs(&col.id, &["a".into(), "b".into()])
        .unwrap();
    service
        .add_songs(&col.id, &["b".into(), "c".into()])
        .unwrap();

    let fetched = service.get(&col.id, "zh-CN").unwrap();
    assert_eq!(fetched.song_ids.len(), 3);
}
