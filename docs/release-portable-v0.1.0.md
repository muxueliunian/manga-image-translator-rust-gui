# Manga Image Translator (Rust) · 便携版 portable-v0.1.0

Windows 便携版漫画翻译器，自带 WebView2 桌面 GUI。解压即用，无需安装。
本分支的首个发布，重点是全新的便携 WebView 编辑器界面（工具栏 + 文件树 + 画布预览 + 模型管理）。

## 下载哪个

| 包 | 适用 | 体积(zip) |
|---|---|---|
| `...-portable-cpu.zip` | **所有 Windows 用户**（无显卡要求） | ~200MB |
| `...-portable-cuda.zip` | 有 **NVIDIA 显卡** 且想用 GPU 加速 | ~0.7–1GB |

> 不确定就下 **CPU 版**。CUDA 版需要支持 CUDA 12 的 NVIDIA 驱动。

## 系统要求

- Windows 10 / 11 x64
- Microsoft Edge WebView2 Runtime（Win11 一般已自带；缺失时去微软官网装“Evergreen Standalone Installer”）
- CUDA 版额外需要：NVIDIA 显卡 + 支持 CUDA 12 的驱动

## ⚠ 首次使用（必读 —— 模型不随包）

便携包**不含模型权重**，需首次自行下载到一个**外部目录**（不要放进解压目录，否则更新覆盖时会被清掉）：

1. 解压 zip，双击 **`run-ui.bat`** 启动。
2. 顶部工具栏点 **「模型」** → **选择一个外部文件夹**（如 `D:\mit-models`）作为模型目录。
3. 在模型面板里 **下载** 需要的 detector / OCR / inpainter 模型（或勾「启动自动下载」）。
4. 回主界面：左侧 **导入** 文件/文件夹 → 勾选要翻译的项 → 点 **「开始翻译」**。
5. 翻译完成在中央画布预览；用工具栏 **「导出选中」** 导出到你设的导出目录。

> 没设模型目录就直接翻译会**明确报错**提示你去设置——这是预期行为，不是 bug。

## 已知限制（如实告知）

- **放大器 ESRGAN 暂不可用**：整页 f32 不分块、极慢，默认保持关闭即可。
- **本地翻译器模型**（Sugoi / NLLB / M2M100 等）当前仍会下载到包内旧路径、更新后重下；
  **推荐用 API 翻译器**（DeepSeek / OpenAI 等，无需下模型）。统一模型根的修复在后续版本（M1c）。
- **无气泡的背景文字** OCR 效果有限（模型能力上限）。
- detector 的 `auto_rotate` 预处理开关当前无实际效果，请保持关闭。
- 默认配置即为推荐配置：盲目调低阈值 / 调高 unclip 往往更差。

## 许可证

GPL-3.0（见 `LICENSE`）。本项目是 fork，源自 `zyddnys/manga-image-translator`（GPL-3.0）
与 Rust 端口 `frederik-uni/manga-image-translator-rust`；详见 `NOTICE`。
随包的 OpenCV / ONNX Runtime / WebView2 / (CUDA 版) NVIDIA 运行时各遵循其自身许可证。

---
*本文件为发布说明草稿，发布时复制到 GitHub Release 描述即可。*
