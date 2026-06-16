# AGENT.md

## 基本协作规则

- 对话使用中文。
- 默认系统环境是 Windows 11，命令示例优先使用 PowerShell。
- 如果用户指令存在明显问题，先指出问题和风险，再给出可执行方案。
- 对用户命令保持审视态度，主动发现可能的冲突、遗漏和副作用。
- 任何代码编写或文件修改前，必须先读取本文件；如本文件引用后续规则文档，也必须继续读取对应文档。

## 项目定位

- 当前主线是 Rust 版 `simple-runtime`。
- 便携桌面 GUI 使用 Windows WebView2 + `wry`，入口是 `crates/runtimes/simple-runtime/src/webview_ui.rs`。
- WebView 静态资源位于 `crates/runtimes/simple-runtime/webview/`。
- `python/source` 是上游 Python 项目子模块/参考实现，不是当前 Rust 便携 GUI 的主目标。
- 除非用户明确要求 Python WebUI，否则 GUI 优化优先改 Rust WebView。

## 关键目录

- Rust workspace：`Cargo.toml`
- 主运行时：`crates/runtimes/simple-runtime/`
- 配置结构：`crates/runtimes/simple-runtime/src/settings/`
- 执行流水线：`crates/runtimes/simple-runtime/src/execute/`
- 模型注册：`crates/runtimes/simple-runtime/src/setup/`
- 模块接口：`crates/interface/`
- 模块实现：`crates/modules/`
- 开发文档：`DEVELOPMENT_GUIDE.md`、`docs/src/`
- Windows 便携打包：`scripts/package-windows-portable.ps1`

## GUI 开发规则

- WebView 前端通过 `window.ipc.postMessage` 与 Rust 后端通信。
- 新增 GUI 行为时同步检查：
  - `crates/runtimes/simple-runtime/webview/index.html`
  - `crates/runtimes/simple-runtime/webview/styles.css`
  - `crates/runtimes/simple-runtime/webview/app.js`
  - `crates/runtimes/simple-runtime/src/webview_ui.rs`
- 普通控件应映射到 Rust `Settings`，高级 JSON 继续作为完整配置兜底。
- 显式参数优先覆盖常用设置：detector、OCR、mask refinement、inpainter、upscaler、translator、render。
- UI 风格保持安静、克制、可扫描；避免花哨装饰，优先清晰分组和稳定布局。
- 不要把 Python React 前端的配置结构直接套到 Rust WebView；两者 schema 不同。

## 性能分析规则

- 图片翻译速度分析先区分冷启动与热启动。
- 单图主链路是：预处理 -> 可选 upscaler -> detector -> OCR -> textline merge -> translator -> mask refinement -> inpainter -> render。
- 常见瓶颈包括模型加载、`detector.options.detect_size`、OCR batch、翻译网络/本地模型、`inpainter.inpainting_size`、PNG 渲染、多图串行。
- 用户要求“只做技术分析探讨”时，不要实现加速代码；可以提出配置优化、埋点方案和后续改造建议。

## 构建与测试

- Windows 构建前通常需要 OpenCV 和 LLVM 环境变量，参考 `DEVELOPMENT_GUIDE.md`。
- 常用命令：
  - `cargo fmt`
  - `cargo test -p simple-runtime`
  - `cargo build -p simple-runtime`
  - `cargo build -p simple-runtime --release --features cuda`
- GUI 改动后应尽量用 Computer Use 回归 Windows WebView UI，至少验证启动、非空白渲染、配置保存、参数联动和基础按钮流程。

## 文件修改约束

- 不要回滚用户未要求回滚的改动。
- 工作树有未知改动时，先识别是否与当前任务相关；无关则忽略，相关则兼容处理。
- 代码改动保持聚焦，不做无关重构。
- 文档应记录实际当前状态，不要把未实现功能写成已实现。
