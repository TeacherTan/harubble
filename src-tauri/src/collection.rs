//! 合集服务——用户自建合集与官方合集的统一管理层。
//!
//! 该模块提供合集的增删改查、歌曲管理（添加、移除、排序）以及导入导出能力。
//! 官方合集从内嵌 JSON 文件加载，用户合集持久化到 SQLite 数据库。
//!
//! # 主要能力
//!
//! - 列出所有合集（官方 + 用户），支持 locale 本地化。
//! - 查询单个合集详情，包含歌曲 ID 列表（按 position 排序）。
//! - 创建、更新、删除用户合集（官方合集只读）。
//! - 向合集添加/移除歌曲，以及对歌曲重新排序。
//! - 导出合集为 JSON 字符串，从 JSON 字符串导入合集。
//!
//! # 设计约束
//!
//! - 官方合集 ID 以 `"official:"` 为前缀，不可写入。
//! - 用户合集 ID 为 UUID v4，存储于 SQLite。
//! - 所有写操作在修改前调用 `guard_not_official` 防止误改官方合集。
//! - `update()` 在调用 `self.get()` 前必须 `drop(conn)` 以避免 Mutex 死锁。

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

// ─── 常量 ─────────────────────────────────────────────────────────────────────

/// 官方合集 ID 前缀，带此前缀的合集不可修改。
const OFFICIAL_PREFIX: &str = "official:";

// ─── 数据结构 ─────────────────────────────────────────────────────────────────

/// 合集摘要，用于列表展示。
///
/// 包含合集基本信息与歌曲数量，不含完整歌曲 ID 列表。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionSummary {
    /// 合集唯一 ID（官方合集以 `"official:"` 为前缀，用户合集为 UUID v4）。
    pub id: String,
    /// 合集名称（已按 locale 解析）。
    pub name: String,
    /// 合集描述（已按 locale 解析）。
    pub description: String,
    /// 封面图路径或 URL，可为空。
    pub cover: Option<String>,
    /// 合集中的歌曲数量。
    pub song_count: i64,
    /// 是否为官方合集。
    pub is_official: bool,
    /// 最后更新时间戳（毫秒，Unix epoch）。
    pub updated_at: i64,
}

/// 合集详情，包含完整歌曲 ID 列表。
///
/// 用于合集详情页展示，歌曲 ID 按 position 升序排列。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    /// 合集唯一 ID。
    pub id: String,
    /// 合集名称（已按 locale 解析）。
    pub name: String,
    /// 合集描述（已按 locale 解析）。
    pub description: String,
    /// 封面图路径或 URL，可为空。
    pub cover: Option<String>,
    /// 合集中的歌曲 ID 列表，按 position 升序排列。
    pub song_ids: Vec<String>,
    /// 是否为官方合集。
    pub is_official: bool,
    /// 最后更新时间戳（毫秒，Unix epoch）。
    pub updated_at: i64,
}

/// 多语种本地化值，key 为 BCP 47 语言标签（如 `"zh-CN"`、`"en-US"`）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedValue(pub HashMap<String, String>);

/// 官方合集 JSON 文件中的单个合集条目。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfficialCollectionEntry {
    /// 合集 ID（不含 `"official:"` 前缀，加载时自动补全）。
    pub id: String,
    /// 合集名称，多语种。
    pub name: LocalizedValue,
    /// 合集描述，多语种。
    pub description: LocalizedValue,
    /// 封面图路径或 URL，可为空。
    pub cover: Option<String>,
    /// 合集中的歌曲 ID 列表，按顺序排列。
    pub song_ids: Vec<String>,
}

/// 官方合集 JSON 文件根结构。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfficialCollectionsFile {
    /// JSON schema 版本，当前为 1。
    pub schema_version: u32,
    /// 官方合集列表。
    pub collections: Vec<OfficialCollectionEntry>,
}

/// 合集服务，管理官方合集（内存）与用户合集（SQLite）。
///
/// 实现 `Clone`，可在 Tauri 状态中共享。
#[derive(Clone)]
pub(crate) struct CollectionService {
    /// 官方合集列表（内存，只读）。
    official: Arc<Vec<OfficialCollectionEntry>>,
    /// SQLite 连接（用户合集持久化）。
    conn: Arc<Mutex<Connection>>,
}

impl CollectionService {
    /// 创建合集服务实例。
    ///
    /// # 参数
    ///
    /// - `db_path`：SQLite 数据库文件路径，不存在时自动创建。
    /// - `official_json`：官方合集 JSON 文件内容（字节切片），通常由 `include_bytes!` 嵌入。
    ///
    /// # 返回值
    ///
    /// 成功返回 `CollectionService`，失败返回中文错误描述。
    ///
    /// # 副作用
    ///
    /// 调用 `initialize_schema()` 确保数据库表结构存在。
    pub(crate) fn new(db_path: &Path, official_json: &[u8]) -> Result<Self, String> {
        let conn = Connection::open(db_path)
            .map_err(|e| format!("打开合集数据库失败: {e}"))?;

        let file: OfficialCollectionsFile = serde_json::from_slice(official_json)
            .map_err(|e| format!("解析官方合集 JSON 失败: {e}"))?;

        let official: Vec<OfficialCollectionEntry> = file
            .collections
            .into_iter()
            .map(|mut entry| {
                if !entry.id.starts_with(OFFICIAL_PREFIX) {
                    entry.id = format!("{}{}", OFFICIAL_PREFIX, entry.id);
                }
                entry
            })
            .collect();

        let service = Self {
            official: Arc::new(official),
            conn: Arc::new(Mutex::new(conn)),
        };
        service.initialize_schema()?;
        Ok(service)
    }

    /// 初始化数据库 schema。
    ///
    /// 创建 `collections` 与 `collection_songs` 表（若不存在），并启用外键约束。
    /// 幂等操作，可安全重复调用。
    fn initialize_schema(&self) -> Result<(), String> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| format!("获取合集数据库锁失败: {e}"))?;

        conn.execute_batch(
            "PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS collections (
                id          TEXT PRIMARY KEY NOT NULL,
                name        TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                cover       TEXT,
                updated_at  INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS collection_songs (
                collection_id TEXT NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
                song_id       TEXT NOT NULL,
                position      INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (collection_id, song_id)
            );

            CREATE INDEX IF NOT EXISTS idx_collection_songs_collection
                ON collection_songs(collection_id, position);",
        )
        .map_err(|e| format!("初始化合集表失败: {e}"))?;

        Ok(())
    }
}
