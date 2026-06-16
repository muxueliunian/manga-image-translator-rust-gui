# Development Guide

本仓库当前以 Rust 版 `simple-runtime` 为主要开发目标。便携桌面 GUI 使用 Windows WebView2 + `wry`，不是 Python WebUI。`python/source` 是上游 Python 项目子模块/参考实现，除非任务明确要求 Python WebUI，否则 GUI 相关开发优先落在 Rust WebView 主线。

## 技术选型

- Rust workspace：edition 2021，resolver 3，根配置在 `Cargo.toml`。
- 推理运行时：`ort` / ONNX Runtime，Windows 可走 CUDA 或 DirectML，失败后落回 CPU。
- 图像处理：OpenCV、`image`、`ndarray`、`imageproc`、自定义 `interface-image`。
- 异步与服务：`tokio`、`actix-web`、`actix-files`、`actix-multipart`。
- 桌面 UI：WebView2 + `wry` / `tao`，备用界面是 `egui`。
- 配置与 schema：`serde`、`schemars`、`config`。
- 文本排版：PNG 渲染使用 `cosmic-text`、`tiny-skia`、`ab_glyph` 等。
- Python 侧：FastAPI + React/Vite 前端是上游/参考线，Rust runtime 不依赖 Python 后端。

## 架构分层

- `crates/interface/*`：模块契约层，定义 detector、OCR、inpainter、upscaler、translator、image、model 等 trait 和数据结构。
- `crates/modules/*`：具体模型和功能实现层，包含检测、OCR、修补、放大、渲染、水印等模块。
- `crates/runtimes/simple-runtime`：组装层，负责 CLI、API、WebView GUI、配置、模型注册和执行流水线。
- `crates/textline-merge`：将 OCR 文本行合并成 `TextBlock`。
- `crates/mask-refinement`：根据检测 mask 与文本块生成更适合修补的 mask。
- `crates/modules/renderer/export`：定义中间导出结构 `Export`。
- `python/source`：上游 Python 项目子模块，可作为参数和交互参考，但配置 schema 与 Rust `Settings` 不同。

## 主要入口

- CLI/API/GUI 命令入口：`crates/runtimes/simple-runtime/src/main.rs`
- 命令定义：`crates/runtimes/simple-runtime/src/cli.rs`
- WebView GUI：`crates/runtimes/simple-runtime/src/webview_ui.rs`
- WebView 静态资源：`crates/runtimes/simple-runtime/webview/`
- 原生 egui 备用界面：`crates/runtimes/simple-runtime/src/ui/`
- Actix API：`crates/runtimes/simple-runtime/src/api.rs`
- 执行流水线：`crates/runtimes/simple-runtime/src/execute/`
- 配置结构：`crates/runtimes/simple-runtime/src/settings/`
- 模型注册：`crates/runtimes/simple-runtime/src/setup/`
- PNG 嵌字渲染：`crates/modules/renderer/png/src/lib.rs`
- Windows 便携打包：`scripts/package-windows-portable.ps1`

## 执行流水线

`Models::execute_with_progress` 位于 `crates/runtimes/simple-runtime/src/execute/mod.rs`。单张图片的主链路如下：

1. 图片预处理：`DynamicImage` 转 `RawImage`，保留 alpha。
2. 可选图像放大：`run_upscaler`。
3. 文本检测：`run_detector` 输出四边形文本区域和 raw mask。
4. OCR 识别：`run_ocr` 输出文本、颜色、位置和置信度。
5. 文本行合并：`run_textline_merge` 生成 `TextBlock`。
6. 字典前处理：`run_pre_dict`。
7. 文本翻译：`run_translators`，支持本地模型和 OpenAI-compatible HTTP 请求。
8. 字典后处理：`run_post_dict`。
9. mask refinement：`run_mask_refinement`。
10. 图像修补：`run_inpainter`。
11. 生成 `Export`，再按 `settings.render.renderer` 渲染为 PNG、HTML 或 Raw。

默认配置由 `Settings::default()` 提供：DBNet、Ocr48px、Sugoi、LamaAot、PNG，upscaler 默认不启用。

## WebView GUI 约定

- 前端通过 `window.ipc.postMessage` 向 `webview_ui.rs` 发送 IPC。
- IPC 类型定义在 `IpcKind`，新增按钮或异步行为时需要同步修改：
  - `crates/runtimes/simple-runtime/webview/index.html`
  - `crates/runtimes/simple-runtime/webview/styles.css`
  - `crates/runtimes/simple-runtime/webview/app.js`
  - `crates/runtimes/simple-runtime/src/webview_ui.rs`
- 翻译任务在后台线程运行，阶段进度通过 `UserEvent::Progress` 推送给前端。
- 翻译完成后结果先写入 `results/webview/job_*`，前端可预览并按复选框多选导出到用户选择的目录。
- API Key、OpenAI-compatible Base URL、模型名、prompt 等配置保存到 `config/app.json`。
- 普通控件应映射到 Rust `Settings`；高级 JSON 保留为完整配置兜底。
- 不要直接复用 Python React 前端的配置字段名，Rust WebView 使用的是 `Settings` schema。

## 参数开放优先级

GUI 中优先显式展示这些 Rust `Settings` 字段：

- `detector.detector`
- `detector.options.detect_size`
- `detector.options.unclip_ratio`
- `detector.options.text_threshold`
- `detector.options.box_threshold`
- `detector.preprocessor.invert`
- `detector.preprocessor.gamma_correct`
- `detector.preprocessor.rotate`
- `detector.preprocessor.auto_rotate`
- `ocr.ocr`
- `ocr.min_text_length`
- `ocr.prob`
- `ocr.filter_text`
- `mask_refinement.method`
- `mask_refinement.ignore_bubble`
- `mask_refinement.dilation_offset`
- `mask_refinement.kernel_size`
- `mask_refinement.furigana`
- `inpainter.inpainter`
- `inpainter.inpainting_size`
- `inpainter.mask`
- `inpainter.inpaint_color`
- `upscaler.upscaler`
- `upscaler.patch_size`
- `upscaler.padding`
- `translator.openai_compatible.timeout_secs`
- 输出格式由 WebView 任务 payload 传给后端，对应 PNG、HTML、Raw。
- `render.text_direction`

## 渲染与排版

- GUI 的 `文字方向` 对应 `settings.render.text_direction`：
  - `Auto`
  - `Horizontal`
  - `Vertical`
- PNG 渲染器会根据原文特征和检测框比例做自动方向判断。
- PNG 渲染器会对已放置文本做 AABB 碰撞避让，避免多个翻译块互相遮挡。
- 字幕或横排对白被误判为竖排时，优先在 GUI 中切换 `Horizontal` 验证。

## 性能分析

用户要求性能优化“只做技术分析探讨”时，不要直接实现加速代码。若用户明确要求实施性能计划，优先做可观测性、CUDA 校验和安全的生命周期优化；先区分冷启动和热启动，再分阶段定位瓶颈。

常见耗时来源：

- 首次模型加载/下载：模型缓存通常在 `models/<kind>/<name>`。
- Detector：默认 DBNet `detect_size=2048`，高分辨率图可能触发更重的检测成本。
- OCR：默认 Ocr48px 会加载多个 ONNX session，耗时随文本框数量和 batch size 增长。
- Translator：本地模型走推理；OpenAI-compatible 是一次批量 HTTP 请求，主要受网络和模型响应影响。
- Inpainter：默认 LamaAot，`inpainting_size=2048`，当前修补通常按整页尺度推理。
- PNG render：文本块多时，字体测量、glyph shaping、描边和合成都可能变慢。
- 并发：WebView 当前逐张处理，多图基本串行。

无需改代码即可试的配置或运行方式：

- 使用 CUDA 构建或便携包：`--features cuda` / `-Cuda`。
- 如追求质量，不要通过降低 `detector.options.detect_size` 或 `inpainter.inpainting_size` 提速。
- 保持 `upscaler.upscaler = null`，避免后续阶段处理放大后的图。
- 临时输出 Raw 或 HTML，用于区分流水线耗时与 PNG 渲染耗时。
- 提高 `ocr.prob` 或 `ocr.min_text_length`，减少低置信文本进入翻译和渲染。
- 同一进程内批量测试，单独记录第一张冷启动和第二张热启动。
- 设置 `MIT_REQUIRE_CUDA=1` 或在 WebView 勾选“要求 CUDA”，避免 CUDA 不可用时静默回退 CPU。

需要代码改造时的方向：

- 给 `execute/mod.rs` 每个阶段加耗时埋点。
- `Models::new` 按配置懒构造，减少不必要模型初始化。
- Inpainting 按 mask bounding box 局部裁剪修补，再贴回整图。
- 拆分 `Models` 锁粒度，允许受控并发。
- 渲染阶段缓存字体测量，减少重复 shaping。
- 将 ONNX Runtime 线程数、provider 偏好和 batch size 配置化。

当前已实现的观测能力：

- CLI 和 WebView 会写入 `logs/job_<timestamp>.log`，记录模型准备、主流水线阶段、渲染写入和总耗时。
- 日志会记录 CUDA feature、CUDA 可用性、是否强制 CUDA、ONNX provider 选择以及可用的 `nvidia-smi` 显存采样。
- WebView 显示推理状态，并暴露 `max_parallel_images` 与 `max_parallel_gpu_jobs` 参数。当前使用受控模型池处理多图；4070 Laptop 8GB 建议默认 `max_parallel_images=2`、`max_parallel_gpu_jobs=1`，确认显存余量后再把 GPU 并发调到 `2`。
- 后续候选优化记录在 `docs/performance-optimization.md`。

## Windows 构建环境

本地构建前通常需要设置 OpenCV 和 LLVM 路径：

```powershell
$env:OPENCV_LINK_LIBS='opencv_world4110'
$env:OPENCV_LINK_PATHS='C:\Users\atlas\Desktop\本子翻译\tools\opencv-4.11.0\opencv\build\x64\vc16\lib'
$env:OPENCV_INCLUDE_PATHS='C:\Users\atlas\Desktop\本子翻译\tools\opencv-4.11.0\opencv\build\include'
$env:OPENCV_DISABLE_PROBES='pkg_config,cmake,vcpkg_cmake,vcpkg'
$env:LIBCLANG_PATH='C:\Users\atlas\Desktop\本子翻译\tools\LLVM-22.1.6\bin'
$env:PATH='C:\Users\atlas\Desktop\本子翻译\tools\opencv-4.11.0\opencv\build\x64\vc16\bin;C:\Users\atlas\Desktop\本子翻译\tools\LLVM-22.1.6\bin;' + $env:PATH
```

常用命令：

```powershell
cargo fmt
cargo test -p simple-runtime
cargo build -p simple-runtime
cargo build -p simple-runtime --release --features cuda
.\scripts\package-windows-portable.ps1 -Cuda -NoZip
```

## 便携包验证

打包输出目录：

```text
dist/manga-image-translator-rust-portable/
```

推荐验证顺序：

1. 运行 `run-ui-debug.bat`，先看控制台日志。
2. 确认 WebView 界面非空白，顶部 backend badge 正常。
3. 选择图片，确认列表中可以删除单个输入。
4. 切换输出格式、文字方向、检测/OCR/修补/放大参数，确认高级 JSON 同步变化。
5. 保存配置，重启后确认配置能恢复。
6. 开始翻译，确认按钮禁用、进度条显示具体阶段。
7. 翻译完成后确认输入列表被清空。
8. 在结果区预览图片，勾选单张或多张，导出到指定目录。
9. 如 CUDA 未启用，检查日志中的 ONNX Runtime provider 和缺失 DLL。

## 编码规范

- 改动 Rust 代码后运行 `cargo fmt`。
- 优先沿用本仓库已有模块边界，不做无关重构。
- 新增模型模块时，通常需要：
  1. 实现对应 `crates/interface/*` trait。
  2. 加入 `settings` enum 和默认配置。
  3. 在 `setup` 中注册模型。
  4. 在 `execute` 中接入调用。
  5. 更新 schema/example、文档和 GUI 控件。
- 文档必须描述当前真实实现，不要把计划项写成已实现。
- 代码里存在性能敏感的 `unsafe` 和图像处理逻辑，改动前先确认数据尺寸、通道数、mask 尺寸和 alpha 处理。

## Git 约定

- 当前用户希望直接在 `master` 上开发并推送到 fork。
- fork remote 为 `muxue`，推送命令通常是：

```powershell
git push muxue master
```
