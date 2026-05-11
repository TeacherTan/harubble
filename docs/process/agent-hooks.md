# 代码格式化与 Hooks 配置

## pre-commit（推荐）

项目使用 [pre-commit](https://pre-commit.com/) 框架在每次提交时自动执行格式化检查，对所有开发者和 AI Agent 统一生效。

### 安装

```bash
pip install pre-commit
pre-commit install
```

安装完成后，每次 `git commit` 都会自动运行以下检查：

- **trailing-whitespace**：移除行尾空白
- **end-of-file-fixer**：确保文件以换行结尾
- **check-merge-conflict**：检测未解决的合并冲突标记
- **prettier**：格式化 JS / TS / Svelte / Markdown 文件
- **cargo fmt**：格式化 Rust 代码

如果格式化修改了文件，提交会被阻止，开发者需要检查 diff 并重新暂存后再次提交。

### 手动运行

```bash
# 对所有文件运行
pre-commit run --all-files

# 对暂存文件运行（等同于提交时的行为）
pre-commit run
```

### 跳过（不推荐）

```bash
git commit --no-verify
```
