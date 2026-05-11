# 文档目录

## 本地开发

### 环境要求

- Rust
- Bun 1.3+（唯一 JS 包管理器）

### 常用命令

```bash
# 安装依赖与启动开发
bun install
bun run tauri:dev
```

```bash
# 检查与测试
bun run format:check
bun run lint
bun run check
cargo test --workspace
```

```bash
# 构建
bun run build
bun run tauri:build
```

### 生成 Rust 文档（rustdoc）

项目中的 Rust API 文档统一通过 `cargo doc` 生成，产物默认输出到 `target/doc/`。

```bash
# 生成核心库文档
cargo doc -p harubble_core --no-deps

# 生成桌面应用库文档（包含 private items）
cargo doc -p harubble --lib --no-deps --document-private-items

# 生成桌面应用二进制入口文档（包含 private items）
cargo doc -p harubble --bin harubble --no-deps --document-private-items
```

- `--no-deps` 只生成当前工作区包的文档，避免展开依赖库。
- `--document-private-items` 适合本地排查模块职责与内部状态。
- 生成后打开 `target/doc/index.html` 查看文档首页。

---

## 文档索引

### reference/ — 技术参考

#### [frontend-guide.md](./reference/frontend-guide.md)

前端架构、组件约定、域边界、运行时架构、UI 系统（设计 token、字体方案、动效规则）、国际化、交互模式与内容规范。

#### [resource-update.md](./reference/resource-update.md)

标签注册表（Tag Registry）的更新机制与数据结构说明。

#### [internationalization.md](./reference/internationalization.md)

国际化架构决策、支持语言、品牌标识规范与翻译层技术选型。

### process/ — 项目规定

#### [release-process.md](./process/release-process.md)

CI/CD 流程、发布触发条件、版本号策略与产物构建。

#### [review-rules.md](./process/review-rules.md)

低风险变更（测试整理、结构性重构、文档补充）的评审与审批规则。

#### [agent-hooks.md](./process/agent-hooks.md)

代码格式化与 hooks 配置。使用 pre-commit 框架统一管理提交前格式化，对所有开发者和 AI Agent 生效。开发者可以直接将此文档提供给 Agent，由 Agent 自行完成安装。

### history/ — 历史记录

#### [decisions.md](./history/decisions.md)

关键技术选型的背景、考量与结论。

#### [roadmap.md](./history/roadmap.md)

后端路线图，包含已完成阶段总览与待办阶段方向。
