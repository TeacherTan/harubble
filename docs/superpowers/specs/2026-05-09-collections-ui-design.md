# 合集 UI 界面设计

## 概述

为合集功能构建完整的前端界面，包括侧栏合集分区、合集详情页、添加到合集菜单、拖拽排序以及导入/导出操作。

## 架构决策

- **入口方式**：Sidebar 独立分区（在专辑列表上方），直接列出所有合集，点击后主区域展示合集详情
- **详情布局**：类专辑详情页风格——顶部合集封面 + 元数据 + 操作按钮，下方歌曲列表
- **状态管理**：新增 `collectionController`，遵循现有 Controller 模式，在 `appRuntime` 中与 `libraryController` 平级注册

## 组件清单

| 组件 | 位置 | 职责 |
|------|------|------|
| `CollectionSidebarSection.svelte` | `src/lib/components/app/` | Sidebar 合集分区：官方+用户合集列表、新建按钮、导入按钮 |
| `CollectionDetailPanel.svelte` | `src/lib/components/app/` | 合集详情页：封面+元数据+操作按钮+歌曲列表容器 |
| `CollectionSongRow.svelte` | `src/lib/components/app/` | 单行歌曲展示，左侧拖拽手柄，显示歌名/艺术家/所属专辑 |
| `AddToCollectionMenu.svelte` | `src/lib/components/app/` | 「添加到合集」弹出菜单，从歌曲行的 ⋮ 菜单触发 |
| `CollectionFormDialog.svelte` | `src/lib/components/app/` | 新建/编辑合集的模态对话框（名称、描述、封面路径） |

## 新增文件

```
src/lib/features/collection/
└── controller.svelte.ts            # 合集状态与操作方法

src/lib/components/app/
├── CollectionSidebarSection.svelte  # Sidebar 合集分区
├── CollectionDetailPanel.svelte     # 合集详情页
├── CollectionSongRow.svelte         # 可拖拽歌曲行
├── AddToCollectionMenu.svelte       # 添加到合集弹出菜单
└── CollectionFormDialog.svelte      # 新建/编辑合集对话框
```

## 需修改的文件

| 文件 | 变更内容 |
|------|---------|
| `src/lib/features/shell/appRuntime.svelte.ts` | 注册 `collectionController` |
| `src/App.svelte` | 主区域增加 `collection` 视图分支 |
| `src/lib/components/app/AlbumSidebar.svelte` | 嵌入 `CollectionSidebarSection` |
| `src/lib/components/app/AlbumDetailPanel.svelte` | 歌曲行增加「添加到合集」菜单入口 |

## 状态管理

### collectionController

```typescript
// src/lib/features/collection/controller.svelte.ts

interface CollectionControllerState {
  collections: CollectionSummary[];       // $state - 全部合集列表
  selectedCollectionId: string | null;    // $state - 当前选中的合集 ID
  selectedCollection: Collection | null;  // $derived - 选中合集详情（含 songIds）
  isLoading: boolean;                     // $state - 加载状态
}

interface CollectionController extends CollectionControllerState {
  loadCollections(): Promise<void>;
  selectCollection(id: string): Promise<void>;
  deselectCollection(): void;
  createCollection(name: string, description: string, coverPath?: string): Promise<void>;
  updateCollection(id: string, name?: string, description?: string, coverPath?: string | null): Promise<void>;
  deleteCollection(id: string): Promise<void>;
  addSongs(collectionId: string, songIds: string[]): Promise<void>;
  removeSongs(collectionId: string, songIds: string[]): Promise<void>;
  reorderSongs(collectionId: string, songIds: string[]): Promise<void>;
  exportCollection(id: string): Promise<void>;
  importCollection(): Promise<void>;
}
```

- `loadCollections()` 在 appRuntime 初始化时调用
- `selectCollection(id)` 同时触发 `currentView = 'collection'`
- 增删改操作完成后自动刷新 `collections` 列表和 `selectedCollection`

### 视图切换

`appRuntime.currentView` 新增 `'collection'` 枚举值：

```typescript
type AppView = 'home' | 'library' | 'tagEditor' | 'collection';
```

主区域渲染逻辑：

```svelte
{#if runtime.currentView === 'home'}
  <HomeView />
{:else if runtime.currentView === 'collection'}
  <CollectionDetailPanel />
{:else}
  <AlbumWorkspace />
{/if}
```

## 交互流程

### 浏览合集

1. 应用启动 → `collectionController.loadCollections()` 加载合集列表
2. Sidebar 合集分区渲染：官方合集（带 ⭐ 标记）在上，用户合集在下
3. 点击合集项 → `selectCollection(id)` → `currentView = 'collection'`
4. 主区域渲染 `CollectionDetailPanel`，展示封面、元数据、歌曲列表

### 新建合集

1. 侧栏底部「+ 新建合集」按钮
2. 弹出 `CollectionFormDialog`（模态）
3. 填写名称（必填）、描述（选填）
4. 点击「创建」→ `createCollection()` → 刷新列表 → 自动选中新合集

### 从专辑添加歌曲

1. 专辑详情页 `AlbumDetailPanel` 歌曲行右侧 `⋮` 菜单
2. 菜单中新增「添加到合集」选项
3. 点击后弹出 `AddToCollectionMenu`，列出所有用户合集（官方合集不显示）
4. 点击目标合集 → `addSongs(collectionId, [songCid])` → toast 提示成功

### 拖拽排序

1. 合集详情页歌曲行左侧显示拖拽手柄 `⠿`（仅用户合集显示）
2. 拖拽释放后收集新顺序的 `songIds` 数组
3. 调用 `reorderSongs(collectionId, newOrder)` → 刷新详情
4. 使用 HTML5 Drag and Drop API 实现

### 导出

1. 合集详情页操作按钮区「导出」按钮
2. 调用 `exportCollection(id)` 获取 JSON 字符串
3. 通过 Tauri `save` 对话框让用户选择保存路径
4. 写入文件

### 导入

1. 侧栏合集分区「导入」按钮（或操作栏）
2. 通过 Tauri `open` 对话框选择 JSON 文件
3. 读取文件内容 → `importCollection(json)` → 刷新列表 → 自动选中导入的合集

## 样式规范

- 遵循现有 CSS 变量体系：`--surface-*`、`--accent-*`、`--motion-*`
- 合集分区背景与 Sidebar 一致（`--surface-sidebar`）
- 详情页复用 `AlbumDetailPanel` 的渐变背景风格
- 官方合集使用 `--accent-*` 色标记
- 拖拽手柄使用 `cursor: grab` / `cursor: grabbing`
- 所有动画使用 CSS transitions（`--motion-base`）和 Svelte 内置 transition

## 错误处理

| 场景 | 行为 |
|------|------|
| 加载合集列表失败 | 侧栏显示错误提示，允许重试 |
| 添加歌曲失败 | toast 提示错误信息 |
| 删除合集 | 先确认对话框，再执行 |
| 导入格式错误 | toast 提示「导入格式无效」 |
| 官方合集写操作 | UI 层直接禁用操作按钮，不发送请求 |

## 不在本次范围内

- 合集封面图片上传/裁剪（仅支持路径输入）
- 合集内搜索
- 合集与播放队列联动
- 合集分享到社交平台
- 侧栏合集的拖拽排序（合集间顺序调整）
