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

### 已完成进度(2026-06-18)
P0 / P1 / P2 / P0R / P2.5 / P2.6 / P3a / P3b 均已实现、打包目检通过并 commit 到 master(未 push)。

### 剩余实现顺序
P3c(搜索/键盘/右键) → P3d(已完成翻译段) → P5(二期叠加层)。
**P4 的画布预览 + pan/zoom 已在 P3b 提前落地**(见下「P3b 实现记录」)。

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
- **P3c**:搜索/过滤框 + 键盘导航 + 右键菜单。
- **P3d**:「已完成翻译」段(本次运行)接入树式列表 + 单击预览。

### 风险/注意
- 勾选大文件夹触发递归 `ListDir` 枚举,需异步 + 进度感知,避免卡 UI。
- 三态复选框 + 懒加载子树:未加载的子项勾选态要在展开时正确回填。
- 树渲染条目可能很多,注意虚拟化或至少避免一次性重渲染整树(沿用隐藏滚动条)。

## 待办 · M1 — 模型管理(2026-06-18 追加,**仅记录,未开工**)

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
此项为**独立工作流,未排期、未开工**;先继续 P3(P3b→P3c→P3d),M1 待用户另行安排。
