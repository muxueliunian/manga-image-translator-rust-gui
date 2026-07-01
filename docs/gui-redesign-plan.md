# 便携 GUI 重构方案(类 Photoshop 编辑器)

> 范围:仅 Rust `simple-runtime` 便携 WebView GUI(WebView2 + wry)。
> 入口 `crates/runtimes/simple-runtime/src/webview_ui.rs`,静态资源
> `crates/runtimes/simple-runtime/webview/{index.html,styles.css,app.js}`。
> 目标:把上下两栏的表单堆叠改成 **工具栏 + 中央画布 + 可折叠侧面板 + 状态栏**。
> 状态:P0/P1/P2 已完成并打包目检通过(暗色主题、五区布局、手风琴均正常)。
> 2026-06-18 用户目检后提出 4 条修订意见(见末尾「用户反馈与修订」),已纳入计划、未来实现;
> 其中**配色方向改为 Claude 经典配色 + 深/浅双模式**,取代原「石墨 + 朱红」P0 token。

## 关键现状结论(决定可行性)

1. **本地图片可直接显示,无需新 IPC / base64**:现有结果缩略图已用 `file:///` URL
   在 `<img>` 渲染(`app.js: fileUrl()` + `.result-thumb`),证明 WebView2 在 `mit://`
   页面下能直接加载本地文件。画布显示输入/输出图复用此机制即可。
2. **检测框/译文叠加数据当前未暴露**:`process_one` 拿到 `Export`(含文本块四边形坐标、
   原文、译文)渲染成文件后即丢弃,前端只拿到输出文件路径。叠加层是唯一需要新增后端
   数据通道的功能,故划二期。
3. **IPC 形态**:id 配对的请求/响应 + log/progress 单向事件,10 个 `IpcKind`
   (AppReady/PickImages/PickFolder/PickOutputDir/Defaults/LoadConfig/SaveConfig/
   StartTranslation/PreviewResult/ExportResults)。扩展容易,一期不改动。

## 设计方向(克制优先,符合 AGENT.md GUI 规则)

"暗房工作台":中性石墨壳后退,让漫画页成为屏幕上唯一明亮饱和的元素(同 PS/Lightroom)。

| 维度 | 选择 |
|---|---|
| 色板 | 石墨壳 `#1E2024 / #2A2D32 / #34383F`;文字 `#E6E7E9` / 弱化 `#8A8F98`;分隔线 `#3A3E45` |
| 单一强调色 | 朱/印泥红 `#C8453B`,只用于激活工具、译文高亮、进度 |
| 字体 | UI 系统无衬线(Inter/Segoe UI);数值 HUD(缩放%、坐标、并发、耗时)用等宽 |
| 圆角/密度 | 4px 小圆角、紧凑行高、hairline 分隔,工具感不卡片化 |

## 目标布局

```
┌──────────────────────────────────────────────────────────────────────┐
│ TOOLBAR  [品牌] │ +图片 +文件夹 ✕清空 │ ▶开始 ⏹ │ ⤓导出 │ 默认 加载 保存 │ ⓘCUDA 中/EN │
├────────┬───────────────────────────────────────────────┬─────────────┤
│ FILMSTRIP │            CANVAS(视觉重心)                │  INSPECTOR  │
│ 缩略图竖列 │   <img> 基底 + (二期)SVG 叠加层           │ 手风琴分组  │
│ 选中/删除 │   原图/译图切换 · 叠加开关 · 缩放% · 平移   │ 翻译/检测·OCR│
│ 拖放区    │                                             │ /修补Mask/  │
│          │                                             │ 放大渲染/输出│
│          │                                             │ /运行 + JSON │
├────────┴───────────────────────────────────────────────┴─────────────┤
│ STATUS  ●就绪  阶段文案 ▓▓▓▓▓░░ 62% · 5/8        ▴日志(可上拉)        │
└──────────────────────────────────────────────────────────────────────┘
```

- TOOLBAR:主动作 + 配置簇 + 后端 badge/语言切换。
- FILMSTRIP(左):输入队列改缩略图竖列,选中当前页、删除单项、底部拖放区,
  翻译后叠完成/失败/跳过角标。
- CANVAS(中):`<img>` 基底(原图/译图切换)+ 缩放平移;二期加 SVG 叠加层。
- INSPECTOR(右):全部配置收进可折叠手风琴分组 + 底部高级 JSON 折叠兜底;窄屏可整体收起。
- STATUS(底):状态点 + 进度条 + 阶段文案 + 可上拉日志面板。

## 旧→新映射表(零功能删除)

| 旧控件/区块 | 现 ID | 新位置 |
|---|---|---|
| 标题/副标题 | `.topbar-brand` | TOOLBAR 左(副标题可删) |
| 后端 badge / 语言切换 | `backendBadge` / `langToggle` | TOOLBAR 右 |
| 添加图片/文件夹/清空 | `pickImages/pickFolder/clearInputs` | TOOLBAR 主动作 + FILMSTRIP 顶 |
| 输入队列列表 | `inputList` | FILMSTRIP 缩略图竖列(`<img>`) |
| 拖放区 | (现无显式 DOM) | FILMSTRIP 底部 |
| 输出目录/格式 | `outputDir/pickOutputDir/outputFormat` | INSPECTOR「输出」组 |
| CUDA 状态/详情 | `providerStatus/cudaError*` | TOOLBAR ⓘ popover + STATUS 点 |
| 强制 CUDA/调试输出 | `requireCuda/debugMode` | INSPECTOR「运行」组 |
| 图片/GPU 并发 | `maxParallelImages/maxParallelGpuJobs` | INSPECTOR「运行」组 |
| 配置 tab(4 个) | `.config-tab*` | INSPECTOR 手风琴分组 |
| 翻译/检测·OCR/修补·Mask/放大渲染 全字段 | `translator…textDirection` | INSPECTOR 对应分组(逻辑全保留) |
| 默认/加载/保存 | `reloadDefaults/loadConfig/saveConfig` | TOOLBAR 配置簇 |
| 配置保存路径提示 | (statusText) | STATUS BAR |
| 高级 JSON | `settingsJson` | INSPECTOR 底部折叠 |
| 开始翻译 + spinner | `startTranslation` | TOOLBAR 主动作 ▶ |
| 进度条/标签 | `progressBar/progressLabel` | STATUS BAR |
| 预览导出结果网格 | `results/result-grid` | 并入 CANVAS(译图)+ FILMSTRIP(角标) |
| 全选/导出选中 | `selectAllResults/exportSelected` | TOOLBAR ⤓导出 + FILMSTRIP 多选 |
| 预览(开 explorer) | `data-preview-index` | CANVAS 右键/双击 或 INSPECTOR 按钮(保留 `PreviewResult`) |
| 运行记录/复制/清空 | `logList/clearLog` | STATUS BAR 上拉面板 |

## IPC 与数据流

- 显示图片(画布+缩略图):**无需新 IPC**。输入用 `state.inputPaths` 的 `file://`;
  输出用 `TranslationOutput.output` 的 `file://`(PNG 直显,HTML/Raw 回退原图+提示)。
  需把"仅 png 出缩略图"放宽到 jpg/jpeg/webp。
- 叠加层(二期):复用结果通道,给 `TranslationOutput` 加
  `overlay: Option<ImageOverlay>`,在 `process_one` 渲染前从 `exp` 提取轻量 DTO:
  `RegionOverlay { quad:[[x,y];4], angle, src, dst, fg, bg }`、
  `ImageOverlay { width, height, regions }`。坐标用渲染基准图尺寸,前端按
  `naturalWidth/Height` 比例缩放定位。**不回传整个 `Export`**。
- 可选性能兜底 IPC:`MakeThumbnail { path, max_side }`,仅在 filmstrip/画布卡顿时再加。
- 现有 10 个 `IpcKind` 一期全部保留,语义不变。

## 增量迁移步骤(每步单独 commit)

| 阶段 | 内容 | 验证 |
|---|---|---|
| P0 ✅ | 抽 CSS 设计 token(色/字/距),不改结构 | 已完成(将被 P0R 改配色) |
| P1 ✅ | HTML 五区骨架,复用全部现有 id | 已完成,目检通过 |
| P2 ✅ | 右面板 tab→手风琴;进度+日志归状态栏 | 已完成,目检通过 |
| **P0R** ✅ | 配色改 Claude 经典配色,`[data-theme=light/dark]` 双模式 + 工具栏切换并 localStorage 持久化;全局隐藏滚动条 | 已完成,目检通过(反馈 ③④) |
| **P2.5** ✅ | 运行记录面板可拖拽边框调高,高度存 localStorage 下次沿用 | 已完成,目检通过(反馈 ②) |
| **P2.6** ✅ | 原生窗口尺寸/位置/最大化记忆:退出时存 `config/window.json`、启动时恢复(唯一改 `webview_ui.rs` 的小后端改动) | 已完成,目检通过(追加需求) |
| P3(修订) | 左栏 = VSCode 式**递归文件树**(图标+名)+ 复选框选择驱动翻译流程;新增 IPC `ListDir`;详见下方「P3 需求细化」 | 见 P3 细化 |
| P4(修订) | 点击左栏任一项(输入图或译图)→ 中央画布 `<img>` 预览 + 缩放/平移;取代结果卡片网格为主视图 | 反馈 ①;缩放平移顺滑 |
| P5(二期) | 后端 overlay DTO + SVG 叠加层 + 开关 | 检测框/译文对齐、放大场景坐标正确 |

每个 GUI 阶段收尾:`cargo fmt` + `cargo check` +
`scripts/package-windows-portable.ps1 -Cuda -NoZip` + `run-ui-debug.bat` 回归。

## 风险点

1. **画布坐标系**(二期最大):upscaler / 预处理 rotate 改变文本块坐标基准,叠加易错位。
2. **大图内存**:多张高分辨率页同时 `file://` 进 filmstrip;用 `loading=lazy` + CSS 降采样,必要时上缩略图 IPC。
3. **wry 注入**:`evaluate_script` 单向注入大 payload 有体积/转义成本;叠加 DTO 保持精简。
4. **`file://` 跨源**:画布只用 `<img>`,不 `fetch` 读像素(受 CORS 限制)。
5. **inline 构建**:资源经 `include_str!` + 字符串替换内联,**不能引入需打包的 JS 库**。
6. **窄屏(最小 960)**:右面板必须可整体收起,否则画布被挤没。

## 画布技术选型(已定)

`<img>` 基底层 + CSS `transform` 缩放/平移 + (二期)SVG 叠加层,手写约 50 行 pan/zoom,
**不引第三方库**。理由:`file://` 原生加载超大图、GPU 合成 transform 顺滑;SVG/DOM 叠加
天然支持交互/任意缩放清晰/绑数据,优于 `<canvas>` 自绘;漫画是单张大图,不需要
OpenSeadragon 深缩放,引库还会破坏 `include_str!` 内联构建。仅当手写平移缩放出现难缠
边界 bug,再考虑内联单文件 panzoom 作兜底。

## 用户反馈与修订(2026-06-18,P2 目检后)

目检确认 P0–P2 渲染正常。用户提出 4 条修订意见,**可延后但必须纳入计划、未来实现**:

1. **左栏 = VSCode 式侧边栏文件结构(对应 P3/P4 修订)**
   - 分两段:① 本地选中的「文件 / 文件夹」(文件夹可像文件树一样展开浏览内部图片);
     ② 「已完成翻译」的译图列表。
   - 两段的条目都可**点击选中**,选中后在**中央画布预览**对应图片(输入图或译图)。
   - 影响:P3 从「缩略图列表」升级为「结构化可导航列表/树 + 选中态」;P4 的画布预览改为
     由左栏点击驱动(而非仅翻译完成后展示),成为主视图。文件夹展开可能需要新 IPC
     `ListDir { path }` 列出夹内图片(现有 `expand_input_paths` 仅在翻译时递归,前端没有目录内容)。

2. **运行记录面板高度可调 + 持久化(P2.5)**
   - 当前状态栏日志 56px 太矮不可用。改为**可拖拽上边框调整高度**,
     调整后写入 localStorage,下次启动按保存值恢复(参照现有 `mitWebviewLang`/`mitWebviewDebug` 持久化方式)。

3. **全局隐藏滚动条(并入 P0R)**
   - 所有滚动区域(inspector、filmstrip、canvas-stage、log-list、textarea 等)的滚动条
     一律隐藏(`scrollbar-width: none` + `::-webkit-scrollbar { display:none }`),保留可滚动行为。

4. **配色改 Claude 经典配色 + 深/浅双模式(P0R,取代原 P0)**
   - 放弃「石墨 + 朱红(暗房工作台)」方向,改用 Claude 经典配色(暖米白/陶土橙系):
     浅色底约 `#F5F4EF` 暖白、强调色 Claude 陶土橙 `#CC785C` 系、深文字;
     深色为暖灰褐底 `#262624` 系 + 同强调色。
   - 必须同时提供**浅色 / 深色两套**,工具栏加切换入口,选择 localStorage 持久化。
   - 实现便利:P0 已把全部颜色收敛进 `:root` CSS 变量,改主题只需换变量值 +
     增加 `[data-theme="light"]` 覆盖块 + 一个 toggle,不动结构。

5. **原生窗口几何记忆(P2.6,追加于 P2.5 测试后)**
   - 窗口尺寸/位置/最大化状态没有记忆,每次启动回默认 1180×780。改为退出(`CloseRequested`)
     时把 `inner_size`/`outer_position`/`is_maximized` 存到 `config/window.json`,启动时读回并
     用 `with_inner_size`/`with_position`/`set_maximized` 恢复。
   - 这是一期里**唯一的后端改动**(`webview_ui.rs`),不走 IPC;localStorage 在 webview 内拿不到
     原生窗口几何,故落在 Rust 侧用配置文件持久化。强制杀进程那次不记录(可接受取舍)。

### 已完成进度(2026-06-18 / 06-19)
P0 / P1 / P2 / P0R / P2.5 / P2.6 / P3a / P3b / P3c / P3d / P3e 均已实现、打包目检通过并 commit 到 master(未 push)。
**P3 整体完成。** P4 画布预览 + pan/zoom 已在 P3b 提前落地。

### 剩余实现顺序
B、C、**G v1.0.1 GPU 统一包(G0~G5 全完成,v1.0.1 已发布)**、**A P5 可编辑嵌字(P5.0~P5.2 + 渲染修复轮完成,P5.3 重修背景延后)** 均已完成。
**剩余:H 放大器彻底优化(下一主线大块)** → D/E/M1c(工程卫生/下载体验/模型根统一,均锦上添花)。
另有可选小项:P5.3 重修背景、给上游提 png `create_buffer` 隐患 issue。
(顺序可按发布节奏调整;H 是独立大块技术债。)

### B/UX 一轮实现记录(2026-06-19,目检通过)
一轮目检反馈连带做了 5 件,均已编译+打包+目检:
- **B 终端流式阶段日志(完成)**:`process_one` 每图发 `▶ [i/N] 文件`(始终)、`✓ 文件 · 总耗时`(始终)、`⊘ 文件 · 未检测到文本`(始终);勾「调试输出」时,复用现有 stage 回调在阶段边界发**上一阶段耗时**(`图片预处理/文字检测/OCR 识别/翻译文本/图像修补…`,缩进嵌套在 ▶ 下)。`run_translation_job` 加「模型已就绪,开始处理 N 张」。多图并发交错,每行带文件名前缀。
- **下载 404 降级**:`run_download_jobs` 把 404(上游未发布的变体,如 waifu2x `swin_unet-art-4x`)从红色"下载失败"降为 `⊘ 上游未发布(404),跳过`,不计失败;汇总显示"成功/失败/跳过"。
- **已完成列表撑满**:基类 `.result-list` 的 `max-height:220px` 未被左栏 override 重置 → 拖高「已完成翻译」区只多空白、行数不变。`.filmstrip .result-list` 加 `max-height:none` 修复。
- **导出目录模型(关键 UX 定型)**:翻译**只写内部临时目录 + 可预览**(不落用户目录);工具栏「运行」组加 **📁 导出目录** 按钮(持久化 `mitOutputDir`,重启记得);「导出选中」= 导出到该持久化目录,未设则当场选目录并持久化。`StartTranslation` **不带** output_dir(曾试"翻译直接落盘"后按用户要求撤回)。inspector「输出」组改为只读回显导出目录 + 输出格式。
- **右栏重影修复 + 交互动画**:`.inspector-scroll` 加 `transform:translateZ(0)`(独占合成层,消除展开/滚动后的 WebView2 stale-tile 重影);原生 `<details>` 瞬间展开改 `initAccordions()` 的 WAAPI 高度+透明度动画(170ms,关闭 `fill:forwards` 防闪)→ 连续重绘从根上避开故障;模型弹窗 `mitFadeIn`/`mitPopIn` 淡入上浮;全部 `prefers-reduced-motion` 守卫。

### 后续 backlog(2026-06-19 评审敲定,全部纳入计划)
- **B 终端流式 debug ✅ 已完成**(见上)。
- **C 下载完整性校验 ✅ 已完成(2026-06-20,commit `4d0e641`)**:`db.rs::download_and_extract` 加 Content-Length 截断检测(identity 响应收字节<总长即报错不落盘;带 Content-Encoding 跳过避免 ureq 自动解压误判);`failure()` 的 `"###"` 占位 hash 分支由「存在即就绪」改 `has_nonempty_content()`(空/截断为 0 重下)+4 单测。
- **A P5 叠加层 + 可编辑嵌字(2026-06-24 重新规划,详见下「A P5 实施计划细化」)**:从「只读叠加层」升级为**人工可编辑嵌字**。核心定位变更见细化文档。
- **D 工程卫生(删 UI 部分 ✅;34 文件 clippy 复核 ✅ 2026-06-20)**:见下「D 实现记录」。`46b986c` 全 34 文件已逐文件复核,结论全部行为等价、可保留(详见会话记录/memory)。剩余未做:CI 接 `cargo clippy`、补 M1b 单测;既有 `test_get_panics_on_double_hash_failure` 在 ureq 3.x 下恒失败(404→Err 不再 panic),建议加 `#[ignore]`。
- **E 下载体验**:取消进行中下载、失败重试入口、并行下载(现串行)。锦上添花。
- **M1c(可选)**:Cargo `[patch]` 统一 git 0.11.0 interface-model → 本地翻译器模型(Sugoi/NLLB 等)也听配置根。

### G — v1.0.1 GPU 统一包 + 应用内 CUDA 运行时下载(2026-06-20 敲定,紧跟 v1.0.0)
**背景**:v1.0.0 只发 CPU 包(69MB、人人能跑、不含 NVIDIA DLL 无再分发 EULA 顾虑)。GPU 不再单独发 CUDA 包,改为单包 + 选装。
**目标**:出**一个**包,exe 用 `--features cuda` 构建 + 捆 `onnxruntime_providers_cuda.dll`(172MB,不在 PyPI 须捆),基础 ~250MB;**不捆**那 ~0.9GB NVIDIA CUDA12/cuDNN9 runtime DLL。模型页旁加「下载 GPU 加速运行时」按钮,从 **NVIDIA 官方 PyPI wheel**(`nvidia-cuda-runtime-cu12`/`cublas-cu12`/`cufft-cu12`/`cudnn-cu12`,版本见 `scripts/package-windows-portable.ps1` 的 `Install-CudaRuntimeFromPython`)拉 DLL 进应用目录,下完重启自动启用。
**可行性已确认**:`base-util/onnx.rs` 的 session 构建器已按 provider 顺序逐个试、CUDA EP 失败自动回退 CPU(仅 `MIT_REQUIRE_CUDA` 时才 bail,~line 121-131);`update.rs::check_cuda_error()` 主动探测。故**带 cuda feature 的同一 exe 在无 GPU/无 DLL 机器上照跑 CPU**,回退逻辑无需新写。
**新工作量**:① Rust 侧端口 `Install-CudaRuntimeFromPython` 的下载/解压逻辑(拉 wheel→取 DLL)+ 新 IPC + progress;② UI「下载 GPU 运行时」入口 + 状态(已下载/缺失/驱动不兼容);③ 驱动版本友好提示(需 CUDA 12 能力驱动 ≈ R525+,驱动无法捆/下);④ 打包脚本出统一包变体(cuda feature + 捆 provider_cuda,不捆 runtime)。
**让用户从 NVIDIA 官方源拉**也绕开我们再分发 NVIDIA DLL 的 EULA。

#### G 实施计划细化(2026-06-20 决策敲定)

**已敲定设计决策**:
- **三态推理设备**(取代现有 `requireCuda` 布尔):**Auto**(默认,检测到机器有可用 CUDA 环境则优先 CUDA,否则 CPU)/ **强制 CUDA**(不可用报错)/ **强制 CPU**(跳过所有 GPU provider)。
- **⚠ 三层探测**(关键,避免「鸡生蛋」):**不能**用 `update.rs::check_cuda_error()` 判断「是否提示下载 DLL」——该函数靠**试建 CUDA session**,缺 DLL 时恰好报「不可用」,会导致该提示下载时反而不提示。改分三层:
  1. **能力层(有没有卡+驱动)**:用 **`nvidia-smi`**(驱动自带,**不依赖**待下载的 runtime DLL;`perf.rs::sample_nvidia_gpu_memory` 已证可用)。加查 `--query-gpu=driver_version`,判断驱动是否够 CUDA 12 能力(门槛 ≈ Windows R527+,实现时核准确值)。
  2. **DLL 齐不齐**:检查那几个 runtime DLL 文件存在性(清单见 `onnx.rs::cuda_error_hint`:cublasLt64_12/cublas64_12/cufft64_11/cudart64_12/cudnn64_9 等)。
  3. **最终验证**:沿用现有 `check_cuda_error()`(试建 session)。
- **提示触发矩阵**:有卡+驱动够新+缺 DLL → 提示下载;有卡+驱动太旧 → 提示更新驱动(不让白下 ~0.9GB);无卡 → 不提示(Auto 静默 CPU、强制 CUDA 报错);DLL 齐+EP 能跑 → 无提示。
- **提示强度分级**:**强制 CUDA + 缺 DLL** = 强提示(面板/弹窗);**Auto + 有卡但缺 DLL** = 温和可关闭横幅(「检测到 NVIDIA 显卡,下载 GPU 运行时可加速」),不强制拦截。
- **DLL 存放**:**预览版先放便携包目录(exe 同级)**——好处=默认在 DLL 搜索路径无需 `SetDllDirectory`;**已知取舍=更新便携包会清掉,需重下**(用户接受;存放位置后期统一规划,见末尾「未来规划」)。
- **DLL 下完必须重启生效**(进程级加载,不同于模型即用):下载完成弹窗「下载完成,需重启启用 GPU 加速 [立即重启]」→ 点击=`spawn(current_exe)` + 退出当前进程,自动重启。
- **session 缓存处理**:`Models` 注册表全局缓存(`ensure_models`),session 在**首次翻译**按当时设备模式固化;切换设备需**清空 ModelPool**(`*guard=None`)让下次翻译重建,或重启。UI 切换设备时提示「将重新加载模型」。
- **UI 位置**:模型 modal 内 **Model list 下方新建「GPU 加速」分区**(复用模型面板风格 + C 完整性校验 + 字节进度节流);**不改「模型」入口名**(本就模型强相关)。三层状态可视化:① 显卡名 ✓ / ② 驱动版本 ✓/✗ / ③ 运行时 DLL 就绪/缺失 → [下载]。

**实施阶段(每阶段单独 commit + 打包目检)**:
| 阶段 | 内容 |
|---|---|
| **G0 统一包构建** | 打包脚本出 `--features cuda` + 捆 `onnxruntime_providers_cuda.dll`(172MB),**不捆** ~0.9GB runtime DLL;基础 ~250MB |
| **G1 三态 setting(后端)** | `onnx.rs` 加全局 `set_device_mode(Auto/Cuda/Cpu)`,`new_session_` 中心点分支(Cpu=跳过 provider 循环;Cuda=现 require bail;Auto=现状),**13 个 setup 调用点不动**;替换 `require_cuda` 布尔为三态(settings+IPC);切设备清 ModelPool |
| **G2 三层探测(后端)** | nvidia-smi 查 name+driver_version + 驱动门槛判断;DLL 存在性检查;复用 `check_cuda_error`;新 IPC `GetGpuRuntimeStatus -> {gpu_name, driver_version, driver_ok, dll_present, ep_ok}` |
| **G3 DLL 下载(后端)** | 端口 `Install-CudaRuntimeFromPython` 的「拉 PyPI wheel → 取 DLL」逻辑;新 IPC `DownloadCudaRuntime` + progress(复用 C 截断校验 + 字节节流);落 exe 同级目录 |
| **G4 UI** | Model modal 加「GPU 加速」分区 + 三层状态可视化 + 下载按钮/进度;三态设备单选(运行组);Auto 温和横幅 / 强制 CUDA 强提示;下载完成弹窗 →「立即重启」自动重启 |
| **G5 驱动提示 + 收尾** | 驱动太旧提示更新;打包脚本统一包变体定稿;回归目检(Auto/CUDA/CPU 三态 + 有卡/无卡/缺 DLL 各路径) |

**未来规划(不在 v1.0.1)**:① ~~存放位置统一规划~~ CUDA DLL 部分已解决(见下「CUDA 运行时 DLL 固定缓存」);模型目录仍是独立的可配置项(M1a),暂不合并;② 模型 modal 升级为总 Settings 页面(「模型」入口届时并入)。

#### CUDA 运行时 DLL 固定缓存(2026-07-01,已实现)

**问题**:`gpu_runtime.rs::cuda_runtime_dir()` 原先返回 exe 自身所在目录;每次重新打包/测试新 exe,`dist/` 都是全新目录,app 误判 DLL 缺失,重新触发 ~0.9GB 下载,严重拖慢本机测试迭代。

**修复**:
- `cuda_runtime_dir()` 改为返回固定的 `%LOCALAPPDATA%\manga-image-translator-rust\cuda-runtime\`(与 exe 位置解耦,重新编译/打包/发新版都不受影响),`LOCALAPPDATA` 缺失时保留旧的 exe-目录 fallback。
- 新增 `init_dll_search_path()`(`#[cfg(windows)]`,在 `main()` 最早处调用,早于 `check_cuda_error()`):调 `SetDllDirectoryW` 指向该固定目录,让 onnxruntime 运行时动态加载 CUDA EP 依赖(cublas/cufft/cudart/完整 cudnn 引擎家族)时能在此找到。
- **`cudnn64_9.dll` 保持现状不变**:它是 `simple-runtime.exe` 进程启动阶段(Windows 装载器解析导入表,早于 `main()`/`SetDllDirectory` 生效时机)的硬依赖,必须继续物理捆绑在 exe 旁边(打包脚本未改);只有运行时才动态加载的大头 DLL 受益于新缓存目录。
- **已验证**:本机 `../tools/cuda-runtime-cu12/`(此前打包脚本用 pip 下载过)一次性搬进新缓存目录,不需重新下载;`cargo build -p simple-runtime --features cuda` 的 debug exe(`target/debug`,其目录本身**不含**cublas/cufft/cudart/完整 cudnn)跑 `-vv ui-webview` 日志显示 `Successfully registered CUDAExecutionProvider` + `cuda_available=true`,证明确实是从新缓存目录加载成功,而非目录巧合含全部文件。
- **效果**:此后无论重新编译多少次、重新打包多少次,只要该缓存目录还在就直接复用,不会再触发 0.9GB 下载;对真实用户升级便携包同样受益(无需额外改动)。
- 涉及文件:`crates/runtimes/simple-runtime/src/gpu_runtime.rs`(`cuda_runtime_dir`/`init_dll_search_path`)、`src/main.rs`(启动时调用)。打包脚本 `package-windows-portable.ps1` 未改动。

### A P5 实施计划细化(2026-06-24 重新规划,决策已敲定)

**定位变更**:原计划 P5 只是「只读叠加层(画检测框+译文 SVG)」。本次升级为**人工可编辑嵌字**。
依据:模型质量已到天花板(见本文档开头「关键负结论」/ memory `project-phase-ui-redesign`),
调参更差、放大器不可用、突破需换更强模型或 LLM 多模态(短期不投)。**人工可编辑嵌字是「效果
不理想」最务实的解**——不等模型升级,让人去修模型修不对的地方(嵌错字、位置偏、译文错)。
模型/算法优化是独立研究线,不并入 P5。

**架构关键发现(已读码确认)**:
- `Export` 二进制格式(`Renderer::Raw` = `exp.export()`,见 `crates/modules/renderer/export/src/lib.rs:84`)
  **已序列化可编辑所需的一切**:原图 `img` + **已抠字的修补背景 `overlay`** + 全部 `TextBlock`
  (四边形 `lines:Vec<[MyPoint;4]>`、`text`、`font_size`、`angle`、`fg_color`、`bg_color`、`translations`;
  见 `crates/textline-merge/src/lib.rs:177`)。
- PNG 渲染器是**纯函数、不吃模型**:`render_export_bytes_with_settings(exp, settings)`
  (`main.rs:90`)只调 `PngRenderer::render(exp, config)`,毫秒级。
  ⇒ **「改文字/挪位置 → 重新嵌字」完全不需要重跑任何模型**。
- **背景修补层与文字层分开存** ⇒ 需求1(改嵌字)不碰 inpainter;需求2(重修背景)才需重跑 inpainter,
  难度差一个数量级,**必须拆开**。
- **当前唯一缺口**:`process_one`(`webview_ui.rs:1639`)渲染完 PNG 后**把 `exp` 丢弃**,只落地扁平 PNG。
  ⇒ 支持编辑的前置条件 = **持久化 `Export`**(它自带 `.export()` 二进制格式)。

**已敲定决策(2026-06-24,用户确认)**:
1. **预览保真** = **近似 SVG + 提交时 Rust 精渲**。编辑时前端用 SVG/HTML 即时显示(近似字体,响应快、实现轻);
   点「应用」才让 Rust 重渲出精确 PNG。**已知取舍**:编辑中画面与最终图有细微差异(浏览器字体 ≠ cosmic-text),
   **以 Rust 重渲 PNG 为准**。
2. **P5.2 首版编辑能力** = **文字内容 + 拖拽位置**(覆盖最常见的「嵌错字/位置偏」)。字号·颜色·框缩放留后续迭代。
3. **需求2(重修背景)排期** = **先做需求1,需求2(P5.3)延后**。P5.2 上线验证后再单独开 P5.3。

**坐标基准风险(老风险点,必须守)**:`upscaler` / 预处理 `auto_rotate` 会改变 blocks 坐标基准 → 叠加错位。
两者**默认都关**。⇒ **P5 先只支持「不放大 / 不旋转」路径**,检测到放大/旋转时画布横幅提示「该图不支持叠加编辑」,
放大场景留到 H 放大器重做之后。

**实施阶段(每阶段单独 commit + 打包目检)**:
| 阶段 | 内容 | 吃模型 | 风险 |
|---|---|---|---|
| **P5.0 持久化 Export**(前置) | `process_one` 渲染 PNG 的**同时**把 `exp` 序列化成 sidecar(`<结果名>.mit`,落结果目录)。`Export` 非 `Clone`,需 `let raw=exp.export(); let exp=Export::load(raw)?` 走一遍(或给 `Export/Image/TextBlock` 派生 `Clone`,取其轻者)。新 IPC `LoadEditable{result_path} -> {bg_data_url, regions:[{idx,quad,angle,text,fg,bg,font_size}], width, height}`:返回**修补背景 data-URL** + 轻量 region 列表(**不回传整个 Export**)。坐标用渲染基准图尺寸,前端按 `naturalWidth/Height` 比例换算。 | 否 | 低 |
| **P5.1 只读叠加层** | 画布在 `#canvasStage` 上叠一层 SVG(随 pan/zoom 同步 transform):译图为底 + 半透明检测框 + region 编号。工具栏开关「显示叠加层」。**先验证坐标对齐**再加编辑。 | 否 | 低 |
| **P5.2 可编辑嵌字**(需求1核心) | 单击 region → 选中(高亮框);双击/编辑按钮 → 原位 `contenteditable`/`textarea` 改译文;拖拽框体整体挪位(更新 `quad` 偏移)。前端维护 `editedBlocks` 副本。「应用」→ 新 IPC `RerenderExport{result_path, edits:[{idx, text?, dx?, dy?}]}` → Rust 加载 sidecar Export、按 idx 改对应 block(`text` 写进 `translations` 的 last_trans 链 / 平移 `lines`)、`PngRenderer` 重渲、覆写结果 PNG + 更新 sidecar、回传新 data-URL。「撤销」=丢弃副本重载。 | 否 | 中(预览保真、平移坐标换算) |
| **P5.3 重修背景**(需求2,延后) | 蒙版编辑(刷选残留文字区)→ 重跑 inpainter(吃 GPU semaphore + progress)生成新 `overlay` → 更新 Export → 重新嵌字。需 mask 画笔 UI + 重 IPC。**独立大块,P5.2 上线后再开**。 | **是** | 高 |

**前端落点**(遵循 AGENT.md「四件套同步」):`webview/index.html`(叠加层 SVG 容器 + 编辑工具条)、
`webview/styles.css`(选中/拖拽/编辑态样式)、`webview/app.js`(叠加层渲染 + 选中/拖拽/编辑 + 与 pan/zoom transform 同步 + IPC)、
`src/webview_ui.rs`(`LoadEditable`/`RerenderExport` 两个 IPC + sidecar 读写)。**不引第三方 JS 库**(保 `include_str!` 内联)。

**收尾验证**:`cargo fmt` + `cargo check` + 打包 + Computer Use 回归(加载译图叠加层对齐、改字重渲、拖拽重渲、放大/旋转图正确提示不可编辑)。

#### A P5.0–P5.2 实现记录(2026-06-24,已编码 + `cargo check` exit 0,待用户测试)

**关键简化(实测确认)**:编辑流程里画布背景=已渲染的结果 PNG(精确、带嵌字),SVG 叠加层只画**可选中/可拖拽的框**(不画近似文字字形,避免浏览器字体与 cosmic-text 排版不一致)。每次提交(改字/拖拽放手)→ `RerenderExport`(纯渲染、不吃模型)覆写 PNG 并回传 data-URL 即时刷新。因背景与框都来自同一 `Export` 坐标空间,**坐标天然对齐,原计划的 upscaler/rotate 错位风险在编辑流程不存在**。

**后端(`webview_ui.rs` + `png` + `main.rs`)**:
- `png::background_image(&Export) -> RawImage`:纯背景(get_image→normalize→apply_inpaint_overlay,不画字),供未来 P5.3 用;当前 P5.2 前端实际用结果 PNG 作背景,此函数 `LoadEditable` 仍返回(备用)。
- `main.rs::raw_image_to_png_bytes(&RawImage)`:抽出 RawImage→PNG 编码复用。
- **P5.0**:`process_one` 渲染前 `exp.export()` 落 sidecar `<结果名>.mit`(仅 PNG 渲染器写,html/raw 跳过),再 `Export::load` 回来渲染;`TranslationOutput` 加 `editable: Option<String>`(sidecar 路径,存在才填)。
- **IPC `LoadEditable{path}`**(同步、纯渲染):读 sidecar→返回 `{width,height,background(data-url),regions:[{index,x,y,w,h,angle,text,fg,bg,fontSize}]}`;region bbox=blocks 的 lines 各点 min/max;text 用渲染器同款解析(`last_trans`→key→译文)。
- **IPC `RerenderExport{path,edits:[{index,text?,dx,dy}]}`**(同步):load sidecar→按 index 改 block(text 写 `translations["last_trans"]="__manual_edit__"`+该键存文本;dx/dy 平移所有 line 点)→`exp.export()` 覆写 sidecar→`Export::load`→`render_export_to_png_bytes_with_direction`(强制 PNG)覆写结果 PNG→回传新 data-URL。

**前端(四件套)**:
- `renderCanvas` 重构:`viewport > .canvas-content(transform 目标) > img + svg.canvas-overlay`,transform 移到 content 让图与叠加层同步 pan/zoom;`setupImageViewer` 返回 live `view` 存 `state.canvasView`(供屏幕↔图像坐标换算),img 二次 `src` 切换(编辑后刷新)**保留当前缩放**不复位。
- 编辑交互:工具栏浮条「编辑嵌字/完成编辑」(仅当前预览是带 `editable` 的结果时显示);进入=`LoadEditable`;点框选中、拖框移动(放手→`RerenderExport{dx,dy}`)、双击框→浮动 textarea 改字(Ctrl+Enter/失焦提交→`RerenderExport{text}`);退出=重渲完的 PNG 即最终结果。SVG 空白区穿透给 viewport 平移(`pointer-events:none` + 框 `all`)。
- 切换预览到别的图自动退出编辑;i18n 中英齐全。

**待办/未做**:① 用户测试 + 打包目检(本次打 `-Cuda -ProviderOnly` 统一包);② `cargo fmt` 收尾;③ P5.3 重修背景(延后);④ 字号/颜色/框缩放(后续迭代);⑤ 已知取舍:每次编辑全图重渲+base64 回传(同步在事件循环线程,大图 ~数百 ms,手动编辑可接受)。

**2026-06-24 用户测试反馈 + 两轮修订**:
- **修订1(框不显示)**:叠加层 `<svg>` 只设了 width/height **属性**没设 CSS 尺寸,Chromium 里绝对定位 svg 算成 **0×0 视口** → viewBox 把内容按 `0÷宽` 缩成一点(看不见也点不中)。修:显式 `style.width/height`。同时修 `[hidden]` 被 `.ghost-button{display}` 覆盖(加 `.canvas-edit-bar [hidden]{display:none!important}`)、框填充改 `rgba` 直写不依赖 `color-mix`。
- **修订2(就地编辑,用户要求)→ 修订3(回退,因 HTML 模拟乱码)**:
  - 修订2 曾试:编辑模式底图换干净抠字背景 + HTML 文字层(每框 `.overlay-text` 按检测框位/字号画译文)。**实测乱码**:Rust 渲染器嵌字时做了**防重叠自动挪位 + 字号自适应缩到框内 + 竖排检测**,HTML 文字层无法复刻 → 框密集/重叠或字号-框比例不符时,文字溢出、互相重叠糊成一团。
  - **修订3(最终方案,采用)**:**能精确所见即所得的只有渲染器本身=烘焙 PNG**。① 编辑模式画布**始终显示精确烘焙 PNG**(不再 HTML 模拟文字,杜绝乱码);② 叠加层=**透明可选中/拖拽框**(`.overlay-box`,无文字,透过它看真实嵌字+漫画);③ 双击框→**屏幕空间浮动 textarea**(`.overlay-editor`,位置/尺寸/字号按 view 换算贴合框,**bg=region.bg 白底**遮住下方旧字、像气泡,fg=region.fg,居中);④ 提交(Enter/失焦,Esc 取消)→`RerenderExport`→**立即把返回的 data-URL 塞回 `img.src` 刷新成精确结果**(zoom 保留)。所见即所得、即时、无乱码;编辑框不再是黑色大块。坐标/字号近似仅限编辑那一刻的输入框,**画布本身永远是精确渲染**。
  - 同时修:`[hidden]` 被 `.ghost-button{display}` 覆盖、SVG 0×0、`color-mix` 依赖(均见修订1)。
- **修订4(编辑框背景透明,用户要求)**:双击编辑框原为灰/白实底。改为 `.overlay-editor` 用 `state.editData.background`(`LoadEditable` 的干净抠字图)做 `background-image`,按 view 缩放(`background-size=图×scale`)+ 偏移(`background-position=-region.x×scale,-region.y×scale`)裁切到该框 → 编辑时显示「去字干净漫画 + 输入文字」,既透出背景又无旧字重影(纯透明会露底层烘焙旧字)。边框改 `outline`(不占布局)保证裁切对齐。

**✅ 用户测试通过(2026-06-24)**:渲染正确、背景可见、编辑/拖拽/重渲即时精确。原**两个遗留小问题已于 2026-06-25 全部修复**(详见下「P5 渲染修复轮」),并在修复过程中又定位并解决了一个更关键的「有字却烘成空框」真 bug:
1. ~~编辑框字号轻微不一致~~ → **已修**:IPC 回传渲染器实算字号(横排实算/竖排回退检测)。
2. ~~多行文本只渲染一行~~ → **已修**:根因不是 `render_block`/挑位,而是字号钳制把字号抬过框能装下的尺寸,换行被裁。

#### A P5 渲染修复轮(2026-06-25,已 commit `6155423`/`ab04aeb`/`ba6e4a8`)

承接上面两个遗留问题,实测又暴露第三个更关键的 bug,一并修复。**这些渲染器缺陷多为既有潜在 bug(非 P5 引入)**,且经核对**上游 master 的 png `render` 整段被注释(stub、不烘字)**,所以上游不会以运行时现象出现,但其 `create_buffer` 仍把 buffer 绑死在框高维度、隐患同源。

- **`6155423 fix(render)` — PNG 渲染器三处缺陷**(`crates/modules/renderer/png/src/lib.rs`):
  - 抽出 `block_layout` 共享 helper(判向→定尺寸→算字号),`render` 与新增 `font_sizes`/`BlockFontMetric` 复用。
  - **多行只渲一行**:字号钳制(向检测字号靠拢)会被抬到超过框能装下的 `fitting` 尺寸,换行后超出固定高度缓冲被 `y>=h` 裁掉 → 只剩顶行。改 `fit_font_size` 以 `max_fontsize` 结果**封顶**(`.min(fitting)`)。
  - **有字却烘成空框**:竖排气泡(`w=64,h=1372,θ≈-90°`)译文转横排塞进 64px 高细横条;`create_buffer` 把 buffer 高度限死=框高,行高>框高时 cosmic-text `shape_until_scroll` 把整行**滚没**→`layout_runs` 空→`wh()` 测成 (0,0)→`max_fontsize` 误判"装得下"→字号爆到 clamp 上限(检测+offset=72)→`render_block` 同样滚没→空白。改横排 `create_buffer` 为 `set_size(Some(width), None)`(**只约束宽度做换行、高度不限**),测量 truthful、二分恢复单调。
  - **文字被甩到图中心**:移除 `find_non_overlapping_position` 防重叠挑位,每块就地在检测框中心合成(保证字贴框 + 编辑时框↔字对应)。重叠就重叠(用户决定)。
  - 回归测试:`fit_font_size_never_exceeds_box_fit` / `max_fontsize_fits_short_box_height` 等,`cargo test -p png@1.0.1` 6 项全过。
- **`ab04aeb feat(gui)` — 编辑交互 + 字号回传**(`main.rs`/`webview_ui.rs`/`app.js`/`styles.css`/`index.html`):
  - **双击→单击改字**:未拖动的点击即进编辑、光标落点击字符处(canvas `measureText` 自算偏移),去掉全选与 `dblclick` 入口。
  - **编辑框关软换行**(`wrap=off`+`white-space:pre`):单行译文不再因浏览器字体比渲染器略宽而回流成两行(编辑框仅作输入、画布始终精确;字体度量差是 HTML 固有,不再强行模拟)。
  - **IPC 回传实算字号**:`load_editable`/`rerender_export` 用 `png::font_sizes` 返回渲染器真实字号(横排实算、竖排/空回退检测),前端 `applyEdit` 合并回 region。提示文案「双击改字」→「单击改字」。
- **`ba6e4a8 fix(gui)` — 状态栏溢出**(`styles.css`):中间画布列 `.canvas` 缺裁剪边界,CUDA 下载进度等超长状态文字画到右侧 inspector 上;加 `overflow:hidden` 围住该列(内部 `canvas-stage`/`log-list` 各自管滚动)。

**⚠ 注意**:旧 dist 里**已经烘成空白的结果 PNG 需重新翻译/重渲**才修正(空白是写死进旧 PNG 的,sidecar 文字其实在)。

**P5 后续仍未做**:① P5.3 重修背景(吃 inpainter,延后);③ 可给上游提 issue/PR(把横排 `create_buffer` 也改成约束宽度不限高,根治同源隐患)。

#### A P5 编辑工具补全(2026-06-25,初版,UI 后续单开会话精修)

在改字/拖拽挪位基础上补齐**字号 / 颜色 / 框缩放**三项手动编辑。后端 `BlockEdit` 扩展 `fontSize`/`fg`/`bg`/`bbox`(全 `Option`、向后兼容),`rerender_export` 应用;前端编辑条加区域控件 + 框右下角缩放手柄。仍走 `RerenderExport` 即时精确重渲。

- **手动字号**:`png` 新增 `pub const MANUAL_FONTSIZE_KEY`("__manual_fontsize__"),`block_layout` 检测到该 translations 哨兵键即**直接用其值、跳过自适应**(与手动改字哨兵同构);`BlockEdit.fontSize=Some(n>0)` 设、`Some(0)` 清回自适应。
- **颜色**:`fg`/`bg` 直写 `TextBlock.fg_color`/`bg_color`(现成字段);前端原生 `<input type=color>`。
- **框缩放**:`BlockEdit.bbox=[x,y,w,h]` → 后端把 `lines` 四点从旧 bbox **缩放+平移**到新 bbox;前端**屏幕空间单角手柄**(`.overlay-resize`,恒定大小、在编辑框之上),松手 `RerenderExport`,自适应字号随新框重算回传。
- 前端 `applyEdit` **promise 链串行化**,快速连点字号/颜色不争抢同一 sidecar;`placeEditOverlays` 统一在 pan/zoom 重定位 textarea+手柄;i18n 中英齐。
- **已知取舍(初版)**:① 缩放手柄只在**非旋转框**上语义精确(竖排气泡 θ≈±90° 按 bbox 各向缩放会略变形);② 取色器打开会让正在编辑的文本提交(无文本改动则无副作用)。
- **遗留(下一会话精修 UI)**:控件布局/视觉、旋转框缩放、颜色/字号交互细节、可能的"重置为自适应"入口。

#### A P5 编辑工具精修(2026-06-25,承接上节遗留)

精修上节列的四个遗留点,后端 + 四件套同步。**已完成,用户已 WebView 实测四项交互无误**。

- **字号 auto/manual 区分 + 重置入口**(遗留①控件视觉 + 可能的重置入口):
  - 根因:`load_editable`/`rerender_export` **恒回传渲染器实算字号**,故 `region.fontSize` 永远 >0、编辑条字号值永远显数字,旧 `--` 是死状态,用户**无法区分自适应 vs 手动钉死**。
  - 后端两处 region JSON 增 `"fontManual": block.translations.contains_key(png::MANUAL_FONTSIZE_KEY)`(查手动哨兵);前端 `region.fontManual` 据此:auto 时值显**灰字**(`.rc-value.is-auto`)、manual 时正常字 + 显 **↺ 重置**按钮(`#fontReset`,`resetFontSize()` 发 `fontSize:0` 清回自适应)。`doApplyEdit` 从 rerender 回传同步 `fontManual`。
- **旋转框禁缩放**(遗留②已知取舍①):`block.angle` 是**度**且 merge 已把 `|angle|<3°` 吸附成 `0`(`textline-merge/src/lib.rs:65`)→ 横排恒 `0`、竖排 ≈`±90`。`ensureResizeHandle` 按 `region.angle!==0` **隐藏手柄**(诚实兑现后端注释的 v1「只非旋转框暴露缩放」契约;字号/颜色仍可改)。完整旋转轴缩放属大改,不在精修范围。
- **取色器不打断编辑**(遗留②已知取舍②):取色 = 原生 OS 对话框,必然 blur textarea。新增 `state.editorKeepAlive`:取色 input `pointerdown` 置位 → textarea `blur` 跳过 commit(不撕编辑框);取色 `change` 把**待提交文本折叠进同一条 edit**(`pendingTextEditFor`,单次 rerender、不丢字);取色 input `blur` 清位 + 重新聚焦 textarea。同一折叠逻辑也用于**字号步进/重置**,顺带修了「步进时刚打的字被旧字 rerender 盖掉」的潜在不一致。
- **缩放手柄可发现性**:`.overlay-resize` 加 hover 放大 + 填 accent 色。
- **未动**:取色器 swatch 样式(`A`/`▣` 字形 + 原生 swatch)判定已够克制,light-touch 不改。
- **验证**:用户已 WebView 实测四项交互无误;JS 无构建步骤。

### H — 放大器(upscaler)彻底分析与优化(2026-06-20 用户提出,独立大块)
**现状**:三个 upscaler(anime4k / waifu2x / esrgan)roadmap 标已实现,但**实际不可用**——整页 f32 不分块 → 显存/内存爆 + 极慢(ESRGAN ~500s/图);且 `detector.options.detect_size=2048` 把放大后的分辨率又下采样丢弃,**放大白做**;默认关闭。
**目标**:彻底分析根因 + 优化,让该功能**完整、正确地提供服务**。
**分析维度(先分析后动手)**:
1. **正确性/链路接入**:厘清放大结果到底服务谁——给 detector 看?给最终渲染?对照 Python 上游 upscaler 接入点(`python/source/manga_translator` 的 upscaling 流程)。找出"放大被 detect_size/下采样丢弃"的确切位置,决定正确的接入顺序(主链路:预处理→upscaler→detector→…→render)。
2. **性能/分块**:整页改 **tile 分块推理**(patch + overlap + 拼接;`interface_image` 已有 `generate_patches_m`/`combine_patches_m` 可复用)+ 评估 f16/批处理;消除单图数百秒。
3. **显存/内存**:大图 OOM 防护;与 G 的 GPU 包结合(CPU 上 upscaler 可能不现实,需明确"仅 GPU 推荐")。
4. **配置/默认值**:理清 `upscale_ratio` / `detect_size` / `inpainting_size` 关系,给可用默认。
5. **模型可用性**:waifu2x 部分变体上游 404(`swin_unet-art-4x` 等);esrgan = realesrgan 可用性。
**交付**:先出分析结论(根因 + 瓶颈定位),再定优化方案(分块推理 + 正确接入渲染链 + 合理默认),最终让放大功能默认可开、结果正确、速度可接受。涉及核心 crate `crates/modules/upscaler/*` + `crates/util` + 主链路 `execute/`。

### D 实现记录(2026-06-20,Codex 执行 + 本会话 review)
**目标**:fork 只主推 WebView GUI,删 egui / 旧 web UI 死代码 + 卫生。任务由主 agent 写 prompt、派 Codex 在本机执行。两个 commit:
- `2eeaf57 refactor: remove legacy desktop and web UI`:删 `src/ui/`(egui)、`src/api.rs` + `web/`(actix + 旧 React)、`Commands::Ui`/`Api`、`src/cache.rs`(确认无引用);Cargo 删 `egui`/`eframe`/`actix-*`(根 + 成员);打包脚本删失效 `run-egui.bat`/`run-webui.bat`;`DEVELOPMENT_GUIDE.md` 同步。**保留** `Cli`/`UiWebview`、`rfd`/`tao`/`wry`/`reqwest`。
- `46b986c chore: clean build and clippy warnings`:**全工作区** clippy/告警清零(34 文件,含 `interface/image`、`renderer/png`、`mask-refinement`、`util`、`detector` 等核心 crate)——**超出了 prompt 的 `-p simple-runtime` 范围**。
**Review 结论(本会话,抽查高风险项)**:核心删除正确完整;**GUI/M1b(`webview_ui.rs`/`webview/`/`models_catalog.rs`)一行未动**;`cargo build`/`clippy -D warnings`/`build --release --features cuda` 三项均过。抽查的 clippy 改动行为等价,且**顺带修了 2 处真问题**:`cpu.rs` 的 `Vec::with_capacity+unsafe set_len`(读未初始化内存 UB)→ `vec![0;n]`;直方图均衡里 u32 可能下溢的 `-` → `saturating_sub`。`base-util default-features=false` 为 no-op(`default=[]`)。唯一非纯等价:`png` 字号搜索 `high` 初值在 `max_size≤1.0` 边角不同(常规字号一致,可忽略)。
**待办**:① 逐文件复核 `46b986c` 全部 34 文件(目前仅抽查);② 范围决策——若想 fork 历史只聚焦"删 UI",可单独 `git revert 46b986c`(独立 commit,不影响删除);③ CI 接 clippy、补 M1b 单测(未做)。

### P3d 实现记录(2026-06-19 完成)
- 左栏拆**两段**:上=输入树,下=「已完成翻译」(本次运行结果,关程序即清空)。结果行紧凑
  (复选框 + 文件名 + 状态徽标,**无缩略图**),`renderResults` 渲进左栏 `#results`,`#filmstripResults`
  整体随有无结果 `hidden`。
- **画布 = 纯预览区**:`renderCanvas` 渲进 `#canvasStage`(原图/译图共用 pan/zoom viewer),无预览时显
  `previewHint`。翻译完成自动预览第一张译图。`fileUrl`/结果缩略图 data-URL 已撤(改 canvas 单图预览)。
- **联动**:单击结果行 `previewResultOutput` → 画布;单击输入树文件 → 画布;两处 active 高亮互斥
  (结果行比 `output===state.preview`,树行比 `path===state.activePath`,切换时清另一侧)。
- 导出沿用:结果行复选框 + 顶栏「导出选中」+「全选」;双击结果行=系统查看器(`PreviewResult`),
  右键=在资源管理器中显示。

### P3e 布局增强(2026-06-19,用户临时追加)
- **左栏宽度可调 + 记忆**:`--filmstrip-width` + 右边缘 `#filmstripResizer`,存 `mitFilmstripWidth`。
- **左栏内部竖向占比可调 + 记忆**:`#filmstripResults` 高度 `--results-height` + 上边缘 `#resultsResizer`,
  存 `mitResultsHeight`。
- **log 改 VSCode 终端式**:`.statusbar` 从 `.app` 全宽行移进 `.canvas` 列(`.canvas` grid:stage 1fr + statusbar auto);
  左右栏从顶到底贯通,log 只占中间列下方。`.app` 行改 `auto minmax(0,1fr)`。log 上边缘仍可拖高(P2.5 不变)。

### P3c 实现记录(2026-06-19 完成)
- **搜索/过滤框**:树顶 `#treeSearch`,`state.filter` 实时过滤;`nodeVisible` 决定显示(文件按名匹配,
  文件夹自身匹配或有已加载匹配后代则显示并**自动展开**到匹配项);无匹配显示 `treeNoMatch`;
  懒加载未展开子树只能按文件夹名匹配(已知局限)。
- **键盘导航**:`#inputList` 加 `tabindex=0`,`handleTreeKey` 基于 `state.visibleNodes`(渲染序快照):
  ↑/↓ 移动并预览、→ 展开/进子项、← 折叠/回父级、空格切勾选、回车预览/展开;`setActiveNode` 统一
  设 active + 预览 + 聚焦 + `scrollIntoView`。
- **右键菜单**:`showTreeContextMenu` 浮层,「在资源管理器中显示」(所有条目)+「移除」(仅根);
  点击空白/Esc/失焦/滚动关闭。新增后端 `IpcKind::RevealInExplorer`。
- **后端坑**:`explorer /select,<path>` 必须用 Windows `CommandExt::raw_arg` 拼 `/select,"<path>"`
  自己加引号;用 `.arg()` 会被 std 把整个 `/select,...` 加引号 → explorer 静默打开「文档」。

### ⚠ 关键架构修正:本地图片不能用 `file://`,改走 IPC 返回 `data:` URL
方案最初(「关键现状结论」第 1 条)假设 WebView2 在 `mit://` 页面下能用 `file:///` 在
`<img>` 显示本地图——**实测为假,该假设从未真机验证过**。WebView2 会拦截 `mit://` 页面发起的
`file://` 子资源请求;改走自定义协议路径段(`mit://localhost/localfile/...`)后,**自定义
scheme 的 `<img>` 子资源 fetch 同样被 Chromium 静默丢弃**(只有顶层导航和浏览器自动请求的
favicon 能命中 handler)。
**最终方案(已实现)**:新增 `IpcKind::ReadImage { path } -> { data_url }`,Rust 端读文件、
按扩展名定 mime、标准 base64 编码成 `data:image/...;base64,...`,前端 `loadLocalImage(img, path)`
异步取回塞进 `img.src`。`data:` URL 一定可渲染,绕开所有 scheme/origin 限制。预览与结果缩略图
共用此通道(`fileUrl()` 已删除)。代价:大图 base64 经 IPC 注入,单图预览可接受;P5 叠加层若需
像素级仍走此机制。

### P3b 实现记录(2026-06-18 完成)
- **复选框三态**:`state.checked`(规范化路径→原始路径 Map,作为翻译选择唯一真相)+
  `state.folderLeaves`(文件夹递归图片叶子缓存)。`enumerateFolderImages` 按需递归 `listDir`;
  `folderLeafCounts` 优先用已加载子树、回退缓存算 {checked,total};`folderCheckState` 得三态。
  懒加载子树展开时勾选态由 `state.checked` 正确回填;`renderTree` 后置回填 `indeterminate`。
- **只翻译勾选项**:`startTranslation` 的 `input_paths` = `[...state.checked.values()]`;空集合报错
  (i18n `noCheckedSelection`)。`rootPaths()` 已删。导入默认**不勾选**(import≠select)。
- **单击预览(独立于勾选)**:单击文件行 → `selectAndPreview` 设 active 高亮 + 画布渲染。change
  处理勾选、click 忽略复选框,二者独立。`removeRoot`/`clearInputs` 同步清理勾选/缓存/预览。
- **画布 pan/zoom(P4 核心提前)**:`setupImageViewer(viewport, img)` 手写 ~60 行,单 CSS transform。
  默认 fit-to-view(保持比例、整张可见);滚轮以光标为中心缩放(下限=fit,上限=fit×12);拖拽平移;
  双击复位;`ResizeObserver` 在 fit 态时随窗口重新 fit。

## P3 需求细化(2026-06-18 敲定)

### 工作流重定义(关键)
旧流程是"添加输入 → 翻译全部输入"。**新流程**:
1. 像 VSCode 一样**导入**文件 / 文件夹(只是加进左栏树,不立即处理)。
2. 树中每个文件/文件夹**前面带复选框**;勾选需要翻译的项。
3. 点「开始翻译」→ **只翻译被勾选的项**。
4. 勾选文件夹 = 默认勾选其下**所有合法目标图片**(排除不支持格式)。

### 左栏结构
- **两段**:① 导入的文件/文件夹(带复选框的递归树)② 已完成翻译(仅本次运行结果,关程序即清空)。
- 两段条目都可**单击 → 中央画布预览**(预览渲染本身属 P4;P3 先建立 active 选中态与基础 `<img>` 预览,使树即时可用)。

### 文件树(VSCode 式)
- **完整递归树**:任意层级子文件夹可逐级展开/折叠。
- 条目样式 = **文件图标 + 文件名**(无缩略图,紧凑轻量)。
- **懒加载**:展开某文件夹时才列其下一层;靠新 IPC `ListDir`。

### 交互(确认要的)
- 展开/折叠文件夹、单击选中预览(基础)。
- **复选框勾选**:决定翻译哪些;文件夹复选框三态(选中/未选/部分),勾选文件夹联动其合法图片子项。
- **右键菜单**:移除条目 / 在资源管理器中显示(`explorer /select`);**不含**磁盘上重命名/删除(太危险)。
- **键盘 ↑/↓ 导航**:在可见行间移动选中并预览;→/← 展开/折叠,空格切换复选框。
- **顶部搜索/过滤框**:按文件名过滤树。

### 选择 → 翻译 的数据契约
- 选择状态以"勾选的图片文件路径集合"为准(精确,支持文件夹内部分取消勾选)。
- 勾选文件夹时,**按需递归 `ListDir`** 枚举其下合法图片并全部加入勾选集合;取消则移除。
- `startTranslation` 的 `input_paths` 改为传**勾选集合**(后端 `expand_input_paths` 仍可兜底过滤,但前端已解析为图片列表)。其余 payload 不变。

### 需要的后端/IPC 改动
- **新增 `IpcKind::ListDir { path } -> { entries: [{ name, path, is_dir, is_image }] }`**:列目录直接子项,标注是否目录、是否受支持图片。
- **`explorer /select,<path>`**:右键"在资源管理器中显示";可扩展现有 `open_path` 或加 `IpcKind::RevealInExplorer`。
- 现有 `PickImages/PickFolder` 保留:用于导入(把路径加进树);`expand_input_paths`/导出/结果模型不变。

### 建议的实现子步骤(便于增量、各自可验证)
- **P3a** ✅:后端 `ListDir`(+ reveal);左栏树数据模型 + 渲染(图标+名、展开/折叠、懒加载)。
- **P3b** ✅:复选框三态 + 文件夹联动子项;重定向 `startTranslation` 为"只翻译勾选项";单击预览
  (+ 顺带做了 `ReadImage` data-URL 图片通道、画布 pan/zoom)。详见上「P3b 实现记录」。
- **P3c** ✅:搜索/过滤框 + 键盘导航 + 右键菜单(+ 后端 `RevealInExplorer`)。详见上「P3c 实现记录」。
- **P3d** ✅:「已完成翻译」段(本次运行)接入左栏列表 + 单击预览;画布转为纯预览区。详见上「P3d 实现记录」。
- **P3e** ✅(临时追加):左栏宽度/内部占比可调并记忆 + log 改 VSCode 终端式停靠。详见上「P3e 布局增强」。

### 风险/注意
- 勾选大文件夹触发递归 `ListDir` 枚举,需异步 + 进度感知,避免卡 UI。
- 三态复选框 + 懒加载子树:未加载的子项勾选态要在展开时正确回填。
- 树渲染条目可能很多,注意虚拟化或至少避免一次性重渲染整树(沿用隐藏滚动条)。

## M1 — 模型管理(M1a 已完成 2026-06-19;M1b 待续)

### 进度
- **M1a ✅(可配置模型根 + 持久化)**:已实现、编译通过、打包成功;目检通过(模型落到外部目录 `本子翻译\model`,布局 `<kind>/<name>/<files>` 正确,不再每次重下)。
- **M1b ✅(模型状态表 + 下载 + 自动下载)**:已实现、`cargo fmt`+`cargo check` 通过;打包/目检留次日。详见下「M1b 实现记录」。
- 决策(用户 2026-06-19):**严格无默认**(webview 强制,egui/CLI 回退老默认不破坏)、M1b 用 **trait `files()`**、模型页 = **工具栏按钮 + 弹出面板**、状态表**列全部已注册模型**。

### M1b 实现记录(2026-06-19)
- **`Model` trait 加 `files() -> Vec<(&'static str key, String file)>`**(默认空):返回该模块 `models()` 里**全部** key 的 (key,file) 映射(不只当前选中)。12 个可下载模块各实现;dynamic upscaler(waifu2x/esrgan/anime4k)直接从 `models()` 键派生 `{key}.onnx`;color/native/tesseract 用默认空(不进状态表)。
- **`interface_model::db` 加两个公开 API**:`model_file_ready(kind,name,file,hash)`(复用 `failure()` 判断就绪,**不下载**;目录未设时 Err→前端视为缺失)、`download_model_file(...)`(Result 版下载,重试 3 次不 panic)。
- **`setup/models_catalog.rs`(新)**:`registry()` 构造每个具体类型(已 impl `Model`)box 成 `Box<dyn Model>`(仅分配空 wrapper,不加载 ONNX)。`model_catalog()`→按 kind 分组的 `ModelGroupStatus{id,kind,label,ready,files}`(Serialize,camelCase);`download_jobs(ids)`→缺失文件下载任务;`selected_core_download_jobs(settings)`→当前选中 detector/ocr/inpainter 的缺失项(供自动下载)。
- **⚠ 关键 quirk**:waifu2x/esrgan/anime4k **三者都声明 `name()="waifu2x"`** 共用 `upscaler/waifu2x/` 目录(文件名互不冲突)。故 group 唯一键用自定义 `id`(dbnet/.../waifu2x/esrgan/anime4k),**不能**用 `kind/name`(会把三者混成一组下全 48 个)。
- **webview IPC**:`GetModelsStatus`(同步,返回 `{modelDir,modelDirSet,groups}`)、`DownloadModels{targets:[id]}`(异步线程+progress/log,空 targets=全部缺失,逐文件 `download_model_file`,完成回传新状态)。启动时若 `auto_download` 开 + 目录已设 → 后台线程下 `selected_core_download_jobs`。
- **自动下载范围决策**:状态表列**全部**模型(用户选),但「启动自动下载」**只下当前选中的 detector/ocr/inpainter**(+ 不含 upscaler)。原因:upscaler `files()` 含全部变体(36/6/6),自动全下不合理;upscaler 仍按需在首次推理下载。状态表里每个 upscaler 组有独立「下载」按钮,用户显式点才下该组全部变体。
- **前端**:模型 modal `#modelsStatus` 渲染分组状态表(kind 分节 + 就绪点 + `N/M 文件` + 每组「下载」按钮 + 顶部「下载全部缺失」/「刷新」);打开 modal/选目录后拉 `getModelsStatus`;下载走 `downloadModels`,progress 复用底部状态栏。i18n 中英齐。CSS 用现有 token。
- **未做(留 M1c 可选)**:patch 统一 git 0.11.0 的 interface-model → 本地翻译器模型也听配置根。本地翻译器模型仍走老路径重下;API 翻译器不受影响。

### 中间栏终端式重构(2026-06-19,用户要求)
**动机**:加长下载文案后,旧 `.statusbar-top` flex 行(进度 label `auto` 宽不截断)撑爆中间列、**溢出压到右侧 inspector**;用户要求中间栏改"终端形式"。**决策**(AskUserQuestion):上下分屏 = **预览(可折叠)/ 独立进度条 strip / 终端**;debug 输出**只重排现有日志、不动后端**。
- `.canvas` 网格改 `var(--preview-height,300px) 6px auto minmax(0,1fr)` = 预览 / resizer / 进度strip / 终端;`.preview-collapsed` 时前两行塌为 0。
- **`.progress-strip`**:`meta` 行(状态标题+文案 / 进度 detail,两侧各自截断)+ **全宽进度条**独占一行 → 溢出根除。`#progressLabel` 改类 `.progress-detail`(等宽、max-width 60%、截断)。
- **`.terminal`**:`.log-list` 改 flex 填充(去掉固定 220px);日志行重排成**控制台风格**——等宽、无卡片边框、按 level 显示**左色边**(info 无、success/warn/error 彩色)、时间戳淡化、`复制/展开`**hover 才显**。
- 预览 `resizer`(`initPreviewResizer`,改 `--preview-height` 存 `mitWebviewPreviewHeight`)+ 折叠 toggle(终端头「隐藏/显示预览」,存 `mitWebviewPreviewCollapsed`);取代旧 `initLogResizer`/`--statusbar-height`。语言切换后回填折叠按钮文案。
- 旧 `.statusbar*`/`.progress-wrap` CSS 已删。**纯前端**,`cargo check`+打包通过。

### M1b 目检反馈修订(2026-06-19,用户首测后)
下载落点正确;两处改进已实现、编译+打包通过:
- **字节级下载进度**:`db.rs::download_and_extract` 加可选 `progress: Option<&mut dyn FnMut(u64,u64)>`(推理路径 `ModelDb::get` 传 `None` 不变),`download_model_file` 透传;原 `ProgressReader`+`io::copy` 改手动读循环(保留 indicatif 控制台条)。webview `run_download_jobs` 给每个文件挂节流(~150ms)回调,发 `ProgressEvent{current=文件序号,total=文件数,percent=字节%,message}`,文案含 `已下/总大小 · 速度/s · 剩 ETA`(`fmt_bytes`/`fmt_eta` 助手);前端 `updateProgress` 自动补 `· n/total · p%`,故 message 不再重复文件计数。
- **模型弹窗 UI 重做**:`关闭`文字钮 → **× SVG 图标钮**(`.icon-close`);`.modal-panel overflow:auto`(整面板滚动致头部滚走)→ `overflow:hidden` + 头部 `flex:none` + body 独立滚动(**关闭键恒可达**);目录选择器原 `.path-picker` 32px 按钮列把「选择目录」挤成竖排、撑大输入框 → 改 `.models-dir-row` flex + 自动宽度按钮 + 等宽单行路径。`.modal-head/.modal-body/.modal-panel` 为全局但仅此 modal 使用。

### M1a 实现记录
- `interface_model`(本地 0.12.2)加全局 `static MODEL_ROOT: RwLock<ModelRootMode{Default|Configured(PathBuf)|RequireConfigured}>`
  + `set_model_root()` / `model_base_dir()`;`ModelDb::get` 改用 `model_base_dir()?.join(kind).join(name)`。
- `webview_ui.rs`:`config/models.json`(`{model_dir, auto_download}`)load/save;`run()` 启动即 `apply_models_config`
  (有目录→Configured;否则→RequireConfigured);新增 IPC `GetModelsConfig`/`SetModelsDir`(选目录存盘+设根)/`SetAutoDownload`。
  simple-runtime 新增直接依赖 `interface-model`。
- 前端:工具栏「模型」按钮 + 弹出面板(modal,目录显示 + 选择目录 + 自动下载开关 + `#modelsStatus` 占位留 M1b)。

### ⚠ 关键限制(dual interface-model,务必记住)
项目里有**两个 interface-model**:本地 **0.12.2**(path)给 detector/OCR/inpainter/upscaler 用;git **0.11.0**
(`github.com/frederik-uni/manga-image-translator-rust`)给 **aio-translator-*** 即本地翻译器 Sugoi/NLLB/M2M100/MBart/JParaCrawl 用。
两者是**不同 crate 实例**,全局 `static` 不互通 → **M1a 的可配置根只对 0.12.2 那批(detector/ocr/inpainter/upscaler)生效**;
本地翻译器模型仍走老 `root_path()/models` → 便携包内仍会重下。**API 翻译器(OpenAI/DeepSeek 等)无模型下载,不受影响**。
**待办 M1c(可选)**:用 Cargo `[patch."https://github.com/frederik-uni/manga-image-translator-rust"] interface-model = { path = "crates/interface/model" }`
把 git 0.11.0 重定向到本地 0.12.2 统一实例 → 翻译器也听配置;风险=0.11→0.12 API 若不兼容需排查适配。次日新会话再评估。

### 问题/根因(原始)

### 问题/根因
便携包每次推理都重新下载模型。根因:模型根路径 = `base_util::project::root_path()/models/<kind>/<name>`
(`crates/interface/model/src/db.rs` 的 `ModelDb::get`),而 `root_path()` 在便携包内(运行时无
`CARGO_MANIFEST_DIR`)回退为 `current_dir()` = `dist\...\` 包内;打包脚本每次重建 dist 把
`dist\...\models` 一并清掉 → 重下。下载在首次推理惰性触发(`ModelLoad::load → download_model → ModelDb::get`)。

### 已敲定设计(用户 2026-06-18)
1. **模型路径可配置 + 无默认**:首次启动不预设路径;必须在「模型」页**手动选**一个外部文件夹后才下载/推理。
2. **路径经 GUI 选择并存 config**(`config/app.json` 或单独 `config/models.json`);下次启动沿用。
3. **完整模型页**:选路径 + 按 kind 列出各模型及**已下载/缺失**状态 + **一键下载缺失** + 可勾选
   **「启动时自动下载」**(避免首次推理等待)。

### 实现要点(开工时再细化)
- 后端:`ModelDb::get` 改用**配置的模型根**而非 `root_path()/models`;根值在启动时从 config 读入
  (可用 `OnceCell`/全局,或把根 thread 进调用)。无路径时给明确错误而非默认下载。
- 枚举模型:遍历 `setup` 注册表里的模型实例,取各自 `Model::models()`(`ModelSource{url,hash}`)+ 文件清单,
  逐项用类似 `failure()`/`ModelDb` 的逻辑判断是否已就绪。
- 新增 IPC(估):`GetModelsStatus`(返回当前路径 + 各模型状态)、`SetModelsDir`、`DownloadModels`
  (带 `progress`/`log` 事件)。下载复用 `db.rs::download_and_extract`/`ModelDb::get`。
- 前端:新增「模型」页/面板(可作工具栏入口或 inspector 一段),展示状态表 + 下载按钮 + 自动下载开关。
- 风险:下载需异步 + 进度;路径未设时推理要拦截并提示去模型页设置;hash 校验沿用现有 `failure()`。

### 现状提示
M1a 目检通过。M1b 已实现并通过 `cargo fmt`+`cargo check`(打包后台进行中,目检留次日)。
剩余仅可选 **M1c**(patch 统一 git 0.11.0 interface-model → 本地翻译器模型也听配置根)。详见上「M1b 实现记录」。
