# mdpeek

极轻量的 Windows Markdown 单文件阅读器。

- **单个 exe**：约 10 MB，无运行时依赖
- **资源占用极小**：内存约 30-40 MB
- **纯原生渲染**：Rust + [egui](https://github.com/emilk/egui)，不依赖任何 WebView / 浏览器内核 / 框架
- **一键安装**：双击 `install.bat` 后，右键 `.md` 文件 → 打开方式 → mdpeek

## 支持语法

标题、粗体/斜体、行内代码、代码块、引用、无序/有序列表、任务列表、表格、分隔线、链接、图片（相对路径基于文件所在目录）、Github 风格引用块。

## 构建

```bash
cargo build --release
# 产物：target/release/reader.exe
```

## 安装

```bash
# 把编译产物复制到 dist/ 并命名为 mdpeek.exe 后：
dist\install.bat
```

卸载：`dist\uninstall.bat`（只移除当前用户的注册表项，不影响系统）。

## 使用

- 资源管理器右键任意 `.md` → 打开方式 → mdpeek
- 或命令行：`mdpeek.exe 文档.md`
- 或把 `.md` 文件拖到窗口上

## 技术说明

- 编译为 `x86_64-pc-windows-gnu`，使用 `rust-lld` 自包含链接，无需安装 mingw gcc
- 中文显示：运行时自动从 `C:\Windows\Fonts` 加载微软雅黑
- 仅写入 `HKCU` 注册表，无需管理员权限，不改动 `.md` 默认关联
