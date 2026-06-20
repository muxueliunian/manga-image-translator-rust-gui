# Manga Image Translator (Rust) · 便携版 v1.0.0

Windows 便携版漫画翻译器，自带 WebView2 桌面 GUI。解压即用，无需安装。
这是本 GUI 分支的首个正式发布。**v1.0.0 为 CPU 版**；GPU 加速见下方「路线」。

## 下载

| 包 | 适用 | 体积 |
|---|---|---|
| `manga-image-translator-rust-portable-cpu.zip` | 所有 Windows 用户 | ~69MB |

> 本版为 CPU 版，无显卡要求，任何机器都能跑。

## 系统要求

- Windows 10 / 11 x64
- Microsoft Edge WebView2 Runtime（Win11 通常自带；缺则装微软官方 Evergreen 安装器）

## ⚠ 首次使用（模型不随包，必读）

1. 解压，双击 **`run-ui.bat`** 启动。
2. 工具栏 **「模型」** → 选一个**外部文件夹**作模型目录（别放进解压目录，更新会被清）。
3. 在面板里下载 detector / OCR / inpainter 模型（或勾「启动自动下载」）。
4. 左侧 **导入** 文件/文件夹 → 勾选要翻译的项 → **「开始翻译」** → 画布预览 → **「导出选中」**。

> 没设模型目录直接翻译会明确报错提示去设置，属预期行为，不是 bug。

## 已知限制

- **放大器（upscaler）暂不建议使用**：当前整页推理、极慢且放大收益易被后续流程丢弃，保持关闭。
  彻底重做见「路线」。
- **本地翻译器模型**（Sugoi / NLLB 等）当前仍会重下，**推荐用 API 翻译器**（DeepSeek / OpenAI 等）。
- 无气泡背景文字 OCR 效果有限；detector `auto_rotate` 开关当前无实际效果，保持关闭。
- 默认配置即为推荐配置。

## 路线 / Roadmap

- **v1.0.1 — GPU 加速（NVIDIA）**：合并 CPU/CUDA 为**单个包**，在应用内一键下载 GPU 运行时，
  同一个包即可在 CPU / GPU 间切换，无需单独的 CUDA 版。
- **后续 — 放大器彻底重做**：让 upscaler（waifu2x / ESRGAN / anime4k）正确、高效地工作。

## 许可证

GPL-3.0（见仓库 `LICENSE`）。本项目是 fork，源自 `zyddnys/manga-image-translator`（GPL-3.0）
与 Rust 端口 `frederik-uni/manga-image-translator-rust`；详见仓库 `NOTICE`。
随包的 OpenCV / ONNX Runtime / WebView2 各遵循其自身许可证（CPU 版不含任何 NVIDIA 组件）。

---
*发布时把上面正文复制到 GitHub Release 描述即可。*
