# 合集功能设计（Collections）

## 概述

为歌曲引入「合集」概念，分为官方合集（随 app 发布的预置定义）和自定义合集（用户创建的可编辑集合）。

## 存储方案：混合架构

| 数据类型 | 存储位置 | 访问模式 |
|---------|---------|---------|
| 官方合集 | `data/official_collections.json`（bundle 内） | 只读，启动时加载到内存 |
| 用户合集 | `siren_local.db` SQLite 表 | 读写，ACID 事务 |

选择混合方案的理由：
- 官方合集是静态配置，与 `tag_registry.json` 同质，随版本发布更新
- 用户合集需要 CRUD + 排序 + 元数据编辑，SQLite 事务性保证数据一致性
- 导出/导入时两者共享同一 JSON schema，格式统一

## 数据模型

### 官方合集 JSON 格式

文件路径：`data/official_collections.json`

```json
{
  "schema_version": 1,
  "collections": [
    {
      "id": "official:arknights-main-theme",
      "name": { "zh-CN": "明日方舟主题曲", "en-US": "Arknights Main Themes" },
      "description": { "zh-CN": "历代主题曲合集", "en-US": "Main theme songs collection" },
      "cover": null,
      "song_ids": ["song-cid-1", "song-cid-2", "song-cid-3"]
    }
  ]
}
```

约束：
- ID 统一使用 `official:` 前缀，防止与用户合集冲突
- `name` / `description` 支持多语言 map（与 tag_registry 一致）
- `song_ids` 数组顺序即播放顺序
- `cover` 可为 null（使用默认封面）或相对路径指向 bundle 内资源

### 用户合集 SQLite Schema

复用现有 `siren_local.db`，新增两张表：

```sql
CREATE TABLE IF NOT EXISTS collections (
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
```

约束：
- 用户合集 ID 使用 UUID v4（无前缀），与官方合集 `official:` 前缀互斥
- `position` 列为排序依据，支持事务性批量更新
- `ON DELETE CASCADE` 确保删除合集时自动清理关联歌曲
- `cover_path` 指向用户选择的本地图片路径，可为 null

## 服务层架构

```
CollectionService (统一入口)
├── 官方合集数据  ← Arc<Vec<Collection>>，启动时从 JSON 加载，不可变
└── 用户合集数据  ← Arc<Mutex<Connection>>，复用 siren_local.db
```

### Rust 结构

```rust
#[derive(Clone)]
pub(crate) struct CollectionService {
    official: Arc<Vec<Collection>>,
    conn: Arc<Mutex<Connection>>,
}
```

### 统一数据结构

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub name: String,           // 展示时已解析为当前 locale
    pub description: String,
    pub cover: Option<String>,
    pub song_ids: Vec<String>,
    pub is_official: bool,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}
```

### 核心方法

| 方法 | 说明 | 官方合集 | 用户合集 |
|------|------|---------|---------|
| `list_all()` | 返回所有合集摘要 | 合并返回 | 合并返回 |
| `get(id)` | 获取单个合集详情（含歌曲列表） | 支持 | 支持 |
| `create(name, desc, cover)` | 创建空合集 | 拒绝 | 支持 |
| `update(id, name, desc, cover)` | 更新元数据 | 拒绝 | 支持 |
| `delete(id)` | 删除合集 | 拒绝 | 支持 |
| `add_songs(id, song_ids)` | 添加歌曲到合集 | 拒绝 | 支持 |
| `remove_songs(id, song_ids)` | 从合集移除歌曲 | 拒绝 | 支持 |
| `reorder_songs(id, song_ids)` | 重排歌曲顺序 | 拒绝 | 支持 |
| `export(id)` | 导出为 JSON | 支持 | 支持 |
| `import(json)` | 从 JSON 导入为用户合集 | — | 支持 |

官方合集的写操作统一返回错误：`"官方合集不可修改"`。

## Tauri Commands

```rust
#[tauri::command]
async fn list_collections(state: State<'_, AppState>) -> Result<Vec<CollectionSummary>, String>;

#[tauri::command]
async fn get_collection(state: State<'_, AppState>, id: String) -> Result<Collection, String>;

#[tauri::command]
async fn create_collection(state: State<'_, AppState>, name: String, description: String, cover_path: Option<String>) -> Result<Collection, String>;

#[tauri::command]
async fn update_collection(state: State<'_, AppState>, id: String, name: Option<String>, description: Option<String>, cover_path: Option<Option<String>>) -> Result<Collection, String>;

#[tauri::command]
async fn delete_collection(state: State<'_, AppState>, id: String) -> Result<(), String>;

#[tauri::command]
async fn add_songs_to_collection(state: State<'_, AppState>, id: String, song_ids: Vec<String>) -> Result<(), String>;

#[tauri::command]
async fn remove_songs_from_collection(state: State<'_, AppState>, id: String, song_ids: Vec<String>) -> Result<(), String>;

#[tauri::command]
async fn reorder_collection_songs(state: State<'_, AppState>, id: String, song_ids: Vec<String>) -> Result<(), String>;

#[tauri::command]
async fn export_collection(state: State<'_, AppState>, id: String) -> Result<String, String>;

#[tauri::command]
async fn import_collection(state: State<'_, AppState>, json: String) -> Result<Collection, String>;
```

## 导出/导入格式

导出的 JSON 与官方合集格式兼容：

```json
{
  "schema_version": 1,
  "collection": {
    "id": "exported-uuid",
    "name": { "zh-CN": "我的合集", "en-US": "My Collection" },
    "description": { "zh-CN": "", "en-US": "" },
    "cover": null,
    "song_ids": ["cid-1", "cid-2"]
  }
}
```

导入时：
- 校验 `schema_version` 兼容性
- 忽略原始 `id`，分配新 UUID
- 如果 `name` 是 string 而非 locale map，包装为 `{ "zh-CN": name }`
- 歌曲 ID 不做存在性校验（允许导入尚未缓存的歌曲引用）

## 前端类型定义

```typescript
interface CollectionSummary {
  id: string;
  name: string;
  description: string;
  cover: string | null;
  songCount: number;
  isOfficial: boolean;
  createdAt: number | null;
  updatedAt: number | null;
}

interface Collection extends CollectionSummary {
  songIds: string[];
}
```

## 错误处理

| 场景 | 错误信息 |
|------|---------|
| 修改官方合集 | `"官方合集不可修改"` |
| 合集不存在 | `"合集不存在: {id}"` |
| 重复歌曲 | 静默忽略已存在的 song_id |
| 导入格式错误 | `"导入格式无效: {detail}"` |
| 数据库锁获取失败 | `"获取数据库锁失败: {detail}"` |

## 不在本次范围内

- 合集封面图片的裁剪/压缩处理（直接存储原始路径）
- 合集间歌曲拖拽移动
- 智能推荐合集
- 合集分享到社交平台
- 合集与播放队列的联动（后续迭代）
