# 首页模块设计文档

## 概述

为 siren-music-download 应用新增首页视图，作为应用启动后的默认展示页面。首页分为两个区域：上半部分为内容发现（最新专辑、系列分组、最近收听历史），下半部分为状态仪表盘（下载进度、正在播放、库存统计、平台概览）。

## 设计决策

| 维度       | 选择                                | 理由                                                                             |
| ---------- | ----------------------------------- | -------------------------------------------------------------------------------- |
| 后端数据层 | SQLite + 细粒度 Commands + 后台预热 | 收听历史需要分页/排序能力；belong 持久化后冷启动零成本；为未来本地数据层扩展奠基 |
| 前端架构   | App.svelte 分层重构 + AppRuntime    | 根治 App.svelte 919 行膨胀问题；appRuntime 可单元测试；首页只是重构的受益者      |
| 导航方式   | 侧栏内切换                          | 保持现有布局一致性，侧栏始终可见，主内容区域根据 `currentView` 切换              |
| 数据来源   | 后端新接口                          | 提供最新专辑、系列分组、收听历史三个维度的专用数据                               |

## 架构设计

### 视图切换机制

在 `shellStore` 中新增视图状态：

```typescript
type AppView = 'home' | 'library';
let currentView = $state<AppView>('home');
```

- 应用启动时默认 `currentView = 'home'`
- 侧栏顶部新增"首页"入口按钮，点击设置 `currentView = 'home'`
- 用户在侧栏选择专辑时自动切换 `currentView = 'library'`
- 主内容区域根据 `currentView` 条件渲染 `HomeView` 或 `AlbumWorkspace`

### App.svelte 分层重构

将 App.svelte 的 `<script>` 块逻辑提取到 `appRuntime.svelte.ts`：

```
src/lib/features/shell/
  appRuntime.svelte.ts    # ~600-700 行纯逻辑
```

`createAppRuntime()` 工厂函数封装：

- 所有 controller 的创建和依赖注入（library、player、download、settings、albumStageMotion、home）
- `bootstrapApp()` 启动流程
- `subscribeToTauriEvents()` 事件订阅
- settings dirty tracking 的 `$effect` 链
- 主题提取和 artwork 加载逻辑
- 所有事件处理函数（handlePlay、handleRefresh、handleSelectAlbum 等）

重构后 App.svelte 缩减为约 200-300 行的纯模板编排层：

```svelte
<script lang="ts">
  import { createAppRuntime } from '$lib/features/shell/appRuntime.svelte';
  const runtime = createAppRuntime();
</script>

<!-- 模板通过 runtime.xxx 访问所有状态和回调 -->
{#if runtime.currentView === 'home'}
  <HomeView {runtime} />
{:else}
  <AlbumWorkspace ...>
    <AlbumWorkspaceContent ... />
  </AlbumWorkspace>
{/if}
```

## 后端设计

### SQLite 数据库

位置：`app_data_dir/siren_local.db`
依赖：`rusqlite` crate

#### Schema

```sql
CREATE TABLE listening_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    song_cid TEXT NOT NULL,
    song_name TEXT NOT NULL,
    album_cid TEXT NOT NULL,
    album_name TEXT NOT NULL,
    cover_url TEXT,
    artists TEXT NOT NULL,  -- JSON array
    played_at TEXT NOT NULL  -- ISO 8601
);
CREATE INDEX idx_listening_played_at ON listening_history(played_at DESC);

CREATE TABLE album_metadata_cache (
    album_cid TEXT PRIMARY KEY,
    belong TEXT NOT NULL,
    updated_at TEXT NOT NULL  -- ISO 8601
);
```

#### 线程安全

`rusqlite::Connection` 不是 `Send`。使用专用线程池（`spawn_blocking`）或单线程 actor 模式处理所有 SQLite 操作，通过 channel 与 Tauri async runtime 通信。

### 新增 Tauri Commands

| Command                   | 签名                                | 说明                                                 |
| ------------------------- | ----------------------------------- | ---------------------------------------------------- |
| `get_latest_albums`       | `(limit: u32) -> Vec<Album>`        | 从 `get_albums()` 取前 N 条，附带库存增强            |
| `get_albums_by_series`    | `() -> Vec<SeriesGroup>`            | 从 SQLite 读 belong 映射，与 albums 做内存 join 分组 |
| `get_recent_history`      | `(limit: u32) -> Vec<HistoryEntry>` | SQLite 查询，按 played_at DESC                       |
| `record_listening_event`  | `(event: ListeningEvent) -> ()`     | 在 `play_song` 内部自动调用                          |
| `clear_listening_history` | `() -> u32`                         | 返回删除条数                                         |
| `get_homepage_status`     | `() -> HomepageStatus`              | 聚合下载进度、库存统计、平台概览                     |

### 数据结构

```rust
struct SeriesGroup {
    series: String,
    albums: Vec<Album>,
}

struct HistoryEntry {
    id: i64,
    song_cid: String,
    song_name: String,
    album_cid: String,
    album_name: String,
    cover_url: Option<String>,
    artists: Vec<String>,
    played_at: String,
}

struct HomepageStatus {
    platform_album_count: u32,       // get_albums().len()
    platform_song_count: u32,        // 累加各 AlbumDetail.songs.len()，从 belong 预热时顺便统计并缓存
    local_downloaded_count: u32,     // LocalInventorySnapshot.downloaded_count
    local_storage_bytes: u64,        // 遍历 inventory 目录计算，或从 inventory snapshot 获取
    active_download_count: u32,      // DownloadManager 当前活跃任务数
    completed_download_count: u32,   // DownloadManager 已完成任务数
}

struct ListeningEvent {
    song_cid: String,
    song_name: String,
    album_cid: String,
    album_name: String,
    cover_url: Option<String>,
    artists: Vec<String>,
}
```

### belong 预热策略

应用启动时 spawn 后台任务 `spawn_belong_warmup`：

1. 调用 `get_albums()` 获取全量专辑列表
2. 查询 SQLite 中已有的 `album_metadata_cache` 记录
3. 对缺失 belong 的专辑，以 `tokio::sync::Semaphore(5)` 限制并发请求 `get_album_detail`
4. 将 belong 写入 SQLite
5. 完成后通过 Tauri 事件 `homepage-belong-ready` 通知前端
6. 后续每次调用 `get_album_detail` command 时顺便更新 SQLite 中的 belong 记录

### 收听历史记录策略

- 触发时机：在后端 `play_song` command 内部自动调用 `record_listening_event`
- 去重：检查最近一条记录的 `song_cid`，连续播放同一首歌只记录一次
- 上限：SQLite 表保留最近 500 条，超出时删除最旧记录
- 清除：用户可通过首页"清除历史"按钮调用 `clear_listening_history`

## 前端设计

### 新增 feature 模块

```
src/lib/features/home/
  store.svelte.ts        # 首页数据状态
  controller.svelte.ts   # 数据获取、缓存、刷新逻辑
```

### homeStore 状态

```typescript
interface HomeState {
  latestAlbums: Album[];
  seriesGroups: SeriesGroup[];
  recentHistory: HistoryEntry[];
  status: HomepageStatus | null;
  loading: boolean;
  belongReady: boolean;
  lastLoadedAt: number | null;
}
```

### homeController 职责

- `loadHomepageData()` — 并行调用 `get_latest_albums`、`get_albums_by_series`、`get_recent_history`、`get_homepage_status`
- 缓存策略：数据缓存在 store 中，切换回首页时如果距上次加载 < 5 分钟则直接展示，否则后台静默刷新
- 监听 `homepage-belong-ready` 事件，收到后刷新系列分组数据
- 提供 `refreshHomepage()` 强制刷新方法

### 组件结构

```
src/lib/components/app/
  HomeView.svelte              # 首页根组件，垂直滚动容器
  HomeLatestAlbums.svelte      # 最新专辑横向卡片列表
  HomeSeriesGroups.svelte      # 系列分类标签/网格
  HomeRecentHistory.svelte     # 最近收听列表
  HomeStatusDashboard.svelte   # 状态仪表盘（4 个卡片）
```

### 组件交互

| 用户操作         | 行为                                                             |
| ---------------- | ---------------------------------------------------------------- |
| 点击最新专辑卡片 | 调用 `runtime.handleSelectAlbum(album)`，自动切换到 library 视图 |
| 点击系列标签     | 在侧栏中过滤该系列的专辑（复用 libraryController 搜索能力）      |
| 点击最近收听条目 | 调用 `runtime.handlePlay(song)` 直接播放                         |
| 点击正在播放卡片 | 只读展示，点击可展开 player flyout                               |
| 点击"清除历史"   | 调用 `clear_listening_history`，刷新列表                         |

### 与现有 controller 的集成

homeController 通过只读访问获取实时状态，不引入新的事件订阅：

- `playerController.currentSong` / `isPlaying` → 正在播放卡片
- `downloadController.activeDownloadCount` → 下载进度
- `libraryController.albums` → 平台概览统计

### 新增 API bridge

在 `src/lib/api.ts` 中新增：

```typescript
export function getLatestAlbums(limit: number): Promise<Album[]>;
export function getAlbumsBySeriesGroup(): Promise<SeriesGroup[]>;
export function getRecentHistory(limit: number): Promise<HistoryEntry[]>;
export function clearListeningHistory(): Promise<number>;
export function getHomepageStatus(): Promise<HomepageStatus>;
```

## 错误处理与降级

首页采用"区块独立降级"策略，每个区块的数据加载失败不影响其他区块：

| 区块       | 失败场景                     | 降级行为                                                        |
| ---------- | ---------------------------- | --------------------------------------------------------------- |
| 最新专辑   | `get_latest_albums` 网络失败 | 显示重试按钮，不阻塞其他区块                                    |
| 系列分组   | belong 预热未完成            | 显示"索引构建中"骨架屏，监听 `homepage-belong-ready` 后自动刷新 |
| 最近收听   | SQLite 读取失败              | 显示空状态"暂无收听记录"                                        |
| 状态仪表盘 | 部分子系统不可用             | 该字段显示 "--"，其他字段正常展示                               |

## 测试策略

### 后端

- `listening_history` 模块：单元测试覆盖 CRUD、去重逻辑、上限截断
- `album_metadata_cache` 模块：单元测试覆盖缓存命中/未命中
- `homepage` commands：集成测试验证聚合数据正确性
- `spawn_belong_warmup`：测试并发控制和错误恢复

### 前端

- `homeController`：单元测试覆盖数据加载、缓存过期判断、刷新逻辑
- `HomeView` 及子组件：组件测试验证渲染和交互
- `appRuntime`：单元测试验证 controller 编排和生命周期管理

## 布局参考

首页在主内容区域内垂直滚动，结构如下：

```
┌─────────────────────────────────────────┐
│ TopToolbar (刷新 | 下载 | 设置)          │
├─────────────────────────────────────────┤
│ ┌─────────────────────────────────────┐ │
│ │ 最新专辑 (横向滚动卡片列表)          │ │
│ │ [封面] [封面] [封面] [封面] ...     │ │
│ ├─────────────────────────────────────┤ │
│ │ 按系列浏览 (标签/胶囊按钮)          │ │
│ │ [Erta Saga] [OST] [EP] [...]       │ │
│ ├─────────────────────────────────────┤ │
│ │ 最近收听 (紧凑列表)                 │ │
│ │ [封面] 歌曲名 - 艺术家  3分钟前  ▶  │ │
│ │ [封面] 歌曲名 - 艺术家  1小时前  ▶  │ │
│ ├─────────────────────────────────────┤ │
│ │ 状态概览                            │ │
│ │ ┌──────────┐ ┌──────────┐          │ │
│ │ │ 平台 78  │ │ 库存 312 │          │ │
│ │ │ 专辑·520首│ │ 已下载   │          │ │
│ │ └──────────┘ └──────────┘          │ │
│ │ ┌──────────────────────┐           │ │
│ │ │ 正在播放 歌曲名 ━━━━ │           │ │
│ │ └──────────────────────┘           │ │
│ │ ┌──────────────────────┐           │ │
│ │ │ 下载任务 2进行中 ━━━ │           │ │
│ │ └──────────────────────┘           │ │
│ └─────────────────────────────────────┘ │
├─────────────────────────────────────────┤
│ PlayerFlyoutStack (底部播放器)           │
└─────────────────────────────────────────┘
```
