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
| **P0R** | 配色改 Claude 经典配色,`[data-theme=light/dark]` 双模式 + 工具栏切换并 localStorage 持久化;全局隐藏滚动条 | 反馈 ③④ |
| **P2.5** | 运行记录面板可拖拽边框调高,高度存 localStorage 下次沿用 | 反馈 ② |
| P3(修订) | 左栏改 VSCode 式结构:两段——①本地选中(文件/文件夹,可展开树)②已完成翻译(译图列表),均可点击选中 | 反馈 ① |
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

### 修订后的实现顺序建议
P0R(配色双模式 + 隐藏滚动条) → P2.5(日志可调高持久化) → P3(左栏文件树双段) →
P4(点击预览画布) → P5(二期叠加层)。P0R 是基础(改全局观感),建议优先。
