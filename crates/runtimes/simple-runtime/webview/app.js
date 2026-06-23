const i18n = {
  zh: {
    title: "漫画图片翻译",
    subtitle: "本地漫画页翻译与导出",
    inputTitle: "输入队列",
    inputStatsEmpty: "0 项 · 0 文件 · 0 文件夹",
    inputStats: "{total} 项 · {files} 文件 · {folders} 文件夹 · 勾选 {checked}",
    pickImages: "添加图片",
    pickFolder: "添加文件夹",
    clearInputs: "清空",
    pathKindFile: "文件",
    pathKindFolder: "文件夹",
    noInput: "尚未添加输入。可多次添加图片或文件夹。",
    treeEmpty: "尚未导入。点顶部「添加图片 / 添加文件夹」导入。",
    treeLoading: "正在读取…",
    treeFolderEmpty: "（无图片）",
    treeSearchPlaceholder: "过滤文件名…",
    treeNoMatch: "无匹配项",
    revealInExplorer: "在资源管理器中显示",
    noCheckedSelection: "请先勾选要翻译的图片。",
    outputGroup: "输出",
    runtimeGroup: "运行",
    outputDir: "导出目录",
    outputDirUnset: "未设置（导出时选择）",
    outputDirHint: "选择导出目录（翻译结果导出到此）",
    outputFormat: "输出格式",
    textDirection: "文字方向",
    deviceMode: "推理设备",
    deviceModeAuto: "自动",
    deviceModeCuda: "强制 CUDA",
    deviceModeCpu: "仅 CPU",
    deviceModeAutoHint: "优先使用 GPU（CUDA），不可用时自动回退 CPU。",
    deviceModeCudaHint: "强制使用 CUDA：不可用时直接报错，绝不回退 CPU。",
    deviceModeCpuHint: "始终使用 CPU 推理，忽略 GPU。",
    gpuSectionTitle: "GPU 加速",
    gpuLayerDetect: "显卡",
    gpuLayerDriver: "驱动",
    gpuLayerDll: "运行时 DLL",
    gpuLayerEp: "ONNX 加速",
    gpuNoNvidia: "未检测到 NVIDIA 显卡",
    gpuDriverNeed: "需 ≥ {min}",
    gpuDriverOk: "已满足",
    gpuDllReady: "已就绪",
    gpuDllPartial: "{ready}/{total} 已就绪",
    gpuEpOk: "可用",
    gpuEpNo: "不可用",
    gpuRecCpuOnly: "当前为 CPU-only 构建，不含 GPU 加速。",
    gpuRecNoGpu: "未检测到 NVIDIA 显卡，将使用 CPU 推理。",
    gpuRecDriver: "NVIDIA 驱动过旧，请更新到 {min} 或更高版本后重新检测。",
    gpuRecNeedDll: "检测到可用显卡，下载 CUDA 运行时即可启用 GPU 加速。",
    gpuRecReady: "GPU 加速已就绪。",
    gpuDownloadBtn: "下载 CUDA 运行时（约 0.9 GB）",
    gpuDriverUpdateBtn: "前往 NVIDIA 驱动下载页",
    gpuDownloading: "下载中…",
    gpuRefresh: "重新检测",
    gpuRestartReady: "CUDA 运行时已安装，重启应用后生效。",
    gpuRestartNow: "立即重启",
    gpuRestartLater: "稍后",
    deviceBannerAutoNeedDll: "检测到可用 GPU。下载 CUDA 运行时即可启用加速。",
    deviceBannerAutoDriverOld: "检测到 NVIDIA 显卡，但驱动过旧（需 ≥ {min}）。更新驱动后即可启用 GPU 加速，当前回退 CPU。",
    deviceBannerCudaUnavailable: "已选「强制 CUDA」，但 CUDA 当前不可用，翻译会直接报错。",
    deviceBannerCudaDriverOld: "已选「强制 CUDA」，但 NVIDIA 驱动过旧（需 ≥ {min}），翻译会直接报错。请先更新驱动。",
    deviceBannerOpen: "前往 GPU 设置",
    debugMode: "调试输出",
    debugModeHint:
      "勾选后保存每张图的诊断中间产物（输入/mask/修补图、OCR 切片、JSON）到 logs/job_*_diagnostics。会变慢并占磁盘；阶段计时日志 job_*.log 始终保留，无需开此项。",
    recommendedSuffix: "· 推荐",
    targetLangSugoiNote: "Sugoi 固定日→英，目标语言对它无效。",
    detectSizeHint: "检测时缩放到的最大边长。越大越能检出小字但越慢，默认 2048。",
    unclipRatioHint: "文本框外扩比例。偏小漏字、偏大粘连，默认 2.3。",
    textThresholdHint: "判定文字像素的阈值，越高越严格。默认 0.5。",
    boxThresholdHint: "保留文本框的最低分数，越高越严格。默认 0.7。",
    minTextLengthHint: "短于该字符数的识别结果会被丢弃。默认 1。",
    ocrProbHint: "OCR 置信度下限，越高越少误检低质文本。默认 0.2。",
    beamSizeHint: "仅对 Ocr48px 生效。1=贪心最快但略降准确率，越大越稳越慢。默认 5。",
    filterTextHint: "命中这些正则的文本不翻译（逗号分隔，如拟声词）。",
    ignoreBubbleHint: "忽略小于该像素阈值的气泡噪点，0 为不忽略。",
    dilationOffsetHint: "mask 膨胀像素，越大覆盖越广但易糊到画面。默认 20。",
    kernelSizeHint: "mask 形态学核大小，越大越平滑。默认 3。",
    inpaintingSizeHint: "修补时缩放到的尺寸，过大易显存不足。默认 2048。",
    maskSourceHint: "决定把修补结果按哪种 mask 贴回原图。",
    maskMethodHint: "mask 细化方法：FitText 贴合文字，FillMask 填充整块。",
    upscalePatchHint: "放大分块边长，0 为整图。显存小可调小。",
    upscalePaddingHint: "放大分块重叠像素，减少拼接缝。",
    filterLangHint: "源文本属于这些语言时跳过翻译（如 en, ja）。",
    timeoutSecsHint: "OpenAI 兼容请求的超时秒数。",
    temperatureHint: "采样温度，越低越稳定、越高越发散。留空用模型默认。",
    topPHint: "核采样阈值。留空用模型默认。",
    preInvertHint: "检测前反相颜色，适合白字黑底。",
    preGammaHint: "检测前做 Gamma 校正，改善明暗。",
    preRotateHint: "检测前旋转图片以适配竖排。",
    preAutoRotateHint: "自动判断并旋转方向。",
    furiganaHint: "扩展遮罩以覆盖假名注音，避免残留。",
    textDirectionHint: "嵌字方向：自动 / 横排 / 竖排。",
    maxParallelImages: "图片并发",
    maxParallelGpuJobs: "GPU 并发",
    cudaDetails: "详情",
    cudaHide: "收起",
    themeToggle: "切换深色 / 浅色",
    textDirectionAuto: "自动",
    textDirectionHorizontal: "横排",
    textDirectionVertical: "竖排",
    configTitle: "翻译配置",
    reloadDefaults: "默认值",
    loadConfig: "加载",
    saveConfig: "保存",
    modelsBtn: "模型",
    modelsTitle: "模型管理",
    modelDir: "模型目录",
    modelDirUnset: "未设置（请选择仓库外的文件夹）",
    modelDirHint: "便携包每次重建会清空内置目录，导致重复下载。请选择一个仓库外的固定文件夹长期存放模型。未设置时无法下载或翻译。",
    chooseDir: "选择目录",
    autoDownload: "启动时自动下载缺失模型",
    close: "关闭",
    modelsStatusTitle: "模型状态",
    refresh: "刷新",
    modelsDirNotSet: "尚未设置模型目录，无法检测状态。请先在上方选择目录。",
    modelKindDetector: "检测",
    modelKindOcr: "OCR",
    modelKindInpainter: "修补",
    modelKindUpscaler: "放大",
    modelReady: "已就绪",
    modelMissing: "缺失",
    modelFilesCount: "{ready}/{total} 文件",
    modelDownload: "下载",
    modelDownloading: "下载中…",
    modelDownloadMissingAll: "下载全部缺失",
    modelStatusLoading: "正在检测模型状态…",
    modelStatusError: "无法获取模型状态：{err}",
    tabTranslation: "翻译",
    tabDetectionOcr: "检测 / OCR",
    tabInpaintMask: "修补 / Mask",
    tabUpscaleRender: "放大 / 渲染",
    translator: "翻译器",
    targetLang: "目标语言",
    provider: "模型供应商",
    baseUrl: "Base URL",
    apiKey: "API Key",
    modelName: "模型名称",
    translationSection: "翻译",
    detectorSection: "检测",
    ocrSection: "OCR",
    maskSection: "Mask 与修补",
    upscalerSection: "放大",
    openaiSection: "OpenAI Compatible",
    filterLang: "跳过语言",
    preDict: "前置字典",
    postDict: "后置字典",
    detector: "检测器",
    detectSize: "检测尺寸",
    unclipRatio: "Unclip Ratio",
    textThreshold: "Text Threshold",
    boxThreshold: "Box Threshold",
    preInvert: "反相检测",
    preGamma: "Gamma 校正",
    preRotate: "旋转检测",
    preAutoRotate: "自动旋转",
    ocrModel: "OCR 模型",
    minTextLength: "最短文本",
    ocrProb: "OCR 置信度",
    beamSize: "Beam 宽度",
    filterText: "过滤文本",
    maskMethod: "Mask 方法",
    ignoreBubble: "忽略气泡",
    dilationOffset: "膨胀偏移",
    kernelSize: "Kernel Size",
    furigana: "扩展假名遮罩",
    inpainter: "修补模型",
    inpaintingSize: "修补尺寸",
    maskSource: "覆盖 Mask",
    inpaintColor: "填充颜色",
    upscaler: "放大器",
    upscalePatch: "Patch Size",
    upscalePadding: "Padding",
    timeoutSecs: "超时秒数",
    temperature: "Temperature",
    topP: "Top P",
    systemPrompt: "System Prompt",
    userPrompt: "User Prompt Template",
    advancedJson: "高级 JSON",
    readyTitle: "准备就绪",
    readyText: "选择输入与输出目录后即可开始。",
    start: "开始翻译",
    running: "翻译中…",
    resultTitle: "预览与导出",
    completedTitle: "已完成翻译",
    previewHint: "单击左侧条目预览图片。",
    editEnter: "编辑嵌字",
    editExit: "完成编辑",
    editHintIdle: "点选文字框：双击改字 · 拖动挪位",
    editEntered: "已进入编辑模式",
    editApplied: "已更新嵌字",
    editFailed: "编辑失败",
    resultStatsEmpty: "暂无结果",
    resultStats: "完成 {done} · 失败 {failed} · 跳过 {skipped}",
    resultEmpty: "暂无结果。完成翻译后可预览图片，并选择导出到指定目录。",
    selectAllResults: "全选",
    deselectAllResults: "取消全选",
    exportSelected: "导出选中",
    preview: "预览",
    exported: "已导出",
    exportNeedSelection: "请先勾选要导出的结果。",
    exportNeedDir: "请先选择导出目录。",
    remove: "删除",
    logTitle: "运行记录",
    logResizeHint: "拖拽调整运行记录高度",
    previewResizeHint: "拖拽调整预览高度",
    hidePreview: "隐藏预览",
    showPreview: "显示预览",
    clearLog: "清空",
    logEmpty: "暂无日志",
    copy: "复制",
    expand: "展开",
    collapse: "收起",
    copied: "已复制",
    selected: "已选择",
    added: "新增",
    folderSelected: "已添加文件夹",
    outputSelected: "导出目录已设置",
    defaultsLoaded: "默认配置已加载",
    configLoaded: "配置已加载",
    configSaved: "配置已保存",
    starting: "已发送任务",
    backendPending: "正在执行翻译任务",
    progressIdle: "等待任务",
    progressPreparing: "正在准备模型",
    progressRunning: "正在处理",
    progressDone: "处理完成",
    jsonError: "JSON 配置格式错误",
    openingImages: "正在打开图片选择窗口…",
    openingFolder: "正在打开文件夹选择窗口…",
    openingOutput: "正在打开输出目录选择窗口…",
    statusDone: "完成",
    statusFailed: "失败",
    statusSkipped: "跳过",
    statusPartial: "部分完成",
    ipcUnavailable: "IPC 未连接",
    backendReady: "后端已连接",
    updateBtn: "检查更新",
    updateTitle: "应用更新",
    updateCheck: "检查更新",
    updateChecking: "正在检查更新…",
    updateCurrentVersion: "当前版本",
    updateLatestVersion: "最新版本",
    updateUpToDate: "已是最新版本。",
    updateAvailable: "发现新版本 {version}。",
    updateNoCompatibleAsset: "发现新版本，但没有当前构建可用的便携包。",
    updateCheckFailed: "检查更新失败：{err}",
    updateViewNotes: "查看发布说明",
    updateReleaseNotes: "发布说明",
    updateAssetSize: "下载大小",
    updateDownloadBtn: "下载更新",
    updateDownloading: "正在下载更新…",
    updateStaged: "更新已下载，准备安装。",
    updateInstallBtn: "安装并重启",
    updateInstalling: "正在重启以完成更新…",
    updateDownloadFailed: "下载更新失败：{err}",
    updateInstallFailed: "启动安装失败：{err}",
    updateLinkCopied: "链接已复制",
  },
  en: {
    title: "Manga Image Translator",
    subtitle: "Local manga page translation and export",
    inputTitle: "Input Queue",
    inputStatsEmpty: "0 items · 0 files · 0 folders",
    inputStats: "{total} items · {files} files · {folders} folders · {checked} checked",
    pickImages: "Add Images",
    pickFolder: "Add Folders",
    clearInputs: "Clear",
    pathKindFile: "File",
    pathKindFolder: "Folder",
    noInput: "No input yet. Add images or folders in multiple passes.",
    treeEmpty: "Nothing imported yet. Use Add Images / Add Folders above.",
    treeLoading: "Reading…",
    treeFolderEmpty: "(no images)",
    treeSearchPlaceholder: "Filter by name…",
    treeNoMatch: "No matches",
    revealInExplorer: "Reveal in Explorer",
    noCheckedSelection: "Check at least one image to translate first.",
    outputGroup: "Output",
    runtimeGroup: "Runtime",
    outputDir: "Export Directory",
    outputDirUnset: "Not set (chosen on export)",
    outputDirHint: "Choose export directory (translated results export here)",
    outputFormat: "Output Format",
    textDirection: "Text Direction",
    deviceMode: "Inference Device",
    deviceModeAuto: "Auto",
    deviceModeCuda: "Force CUDA",
    deviceModeCpu: "CPU only",
    deviceModeAutoHint: "Prefer GPU (CUDA); fall back to CPU automatically when unavailable.",
    deviceModeCudaHint: "Force CUDA: fail fast if unavailable, never fall back to CPU.",
    deviceModeCpuHint: "Always run inference on CPU, ignoring the GPU.",
    gpuSectionTitle: "GPU Acceleration",
    gpuLayerDetect: "GPU",
    gpuLayerDriver: "Driver",
    gpuLayerDll: "Runtime DLLs",
    gpuLayerEp: "ONNX EP",
    gpuNoNvidia: "No NVIDIA GPU detected",
    gpuDriverNeed: "needs ≥ {min}",
    gpuDriverOk: "OK",
    gpuDllReady: "Ready",
    gpuDllPartial: "{ready}/{total} present",
    gpuEpOk: "Available",
    gpuEpNo: "Unavailable",
    gpuRecCpuOnly: "This is a CPU-only build; no GPU acceleration.",
    gpuRecNoGpu: "No NVIDIA GPU detected; CPU inference will be used.",
    gpuRecDriver: "NVIDIA driver is too old. Update to {min} or newer, then re-check.",
    gpuRecNeedDll: "GPU detected. Download the CUDA runtime to enable acceleration.",
    gpuRecReady: "GPU acceleration is ready.",
    gpuDownloadBtn: "Download CUDA runtime (~0.9 GB)",
    gpuDriverUpdateBtn: "Open NVIDIA driver downloads",
    gpuDownloading: "Downloading…",
    gpuRefresh: "Re-check",
    gpuRestartReady: "CUDA runtime installed. Restart the app to take effect.",
    gpuRestartNow: "Restart now",
    gpuRestartLater: "Later",
    deviceBannerAutoNeedDll: "A usable GPU was detected. Download the CUDA runtime to enable acceleration.",
    deviceBannerAutoDriverOld: "An NVIDIA GPU was detected, but its driver is too old (needs ≥ {min}). Update the driver to enable GPU acceleration; falling back to CPU for now.",
    deviceBannerCudaUnavailable: "“Force CUDA” is selected, but CUDA is currently unavailable — translation will fail.",
    deviceBannerCudaDriverOld: "“Force CUDA” is selected, but the NVIDIA driver is too old (needs ≥ {min}) — translation will fail. Update the driver first.",
    deviceBannerOpen: "Open GPU settings",
    debugMode: "Debug dump",
    debugModeHint:
      "Save per-image diagnostics (input/mask/inpainted images, OCR patch crops, JSON) under logs/job_*_diagnostics. Slower and uses disk; the stage-timing job_*.log is always written, so leave this off for normal runs.",
    recommendedSuffix: "· recommended",
    targetLangSugoiNote: "Sugoi is fixed JA→EN; the target language has no effect.",
    detectSizeHint: "Max side length the image is scaled to for detection. Larger finds small text but is slower. Default 2048.",
    unclipRatioHint: "How much each text box is expanded. Too small clips text, too large merges boxes. Default 2.3.",
    textThresholdHint: "Threshold for classifying text pixels; higher is stricter. Default 0.5.",
    boxThresholdHint: "Minimum score to keep a text box; higher is stricter. Default 0.7.",
    minTextLengthHint: "Drop OCR results shorter than this many characters. Default 1.",
    ocrProbHint: "Minimum OCR confidence; higher drops more low-quality text. Default 0.2.",
    beamSizeHint: "Ocr48px only. 1 = greedy (fastest, slightly less accurate); higher is more robust but slower. Default 5.",
    filterTextHint: "Text matching these regexes is left untranslated (comma-separated, e.g. sound effects).",
    ignoreBubbleHint: "Ignore bubble specks smaller than this pixel threshold; 0 disables.",
    dilationOffsetHint: "Mask dilation in pixels. Larger covers more but can bleed into art. Default 20.",
    kernelSizeHint: "Morphology kernel size for the mask; larger is smoother. Default 3.",
    inpaintingSizeHint: "Size the image is scaled to for inpainting; too large risks OOM. Default 2048.",
    maskSourceHint: "Which mask is used to paste the inpainted result back onto the page.",
    maskMethodHint: "Mask refinement: FitText hugs the glyphs, FillMask fills the whole region.",
    upscalePatchHint: "Upscaler tile size; 0 means the whole image. Lower it on small VRAM.",
    upscalePaddingHint: "Overlap pixels between upscaler tiles to hide seams.",
    filterLangHint: "Skip translation when the source is one of these languages (e.g. en, ja).",
    timeoutSecsHint: "Request timeout in seconds for OpenAI-compatible calls.",
    temperatureHint: "Sampling temperature; lower is steadier, higher is more varied. Blank uses the model default.",
    topPHint: "Nucleus sampling threshold. Blank uses the model default.",
    preInvertHint: "Invert colors before detection; good for white text on dark.",
    preGammaHint: "Apply gamma correction before detection to balance brightness.",
    preRotateHint: "Rotate the image before detection to fit vertical text.",
    preAutoRotateHint: "Auto-detect and rotate orientation.",
    furiganaHint: "Extend the mask over furigana so ruby text isn't left behind.",
    textDirectionHint: "Rendered text direction: auto / horizontal / vertical.",
    maxParallelImages: "Image Concurrency",
    maxParallelGpuJobs: "GPU Concurrency",
    cudaDetails: "Details",
    cudaHide: "Hide",
    themeToggle: "Toggle dark / light",
    textDirectionAuto: "Auto",
    textDirectionHorizontal: "Horizontal",
    textDirectionVertical: "Vertical",
    configTitle: "Translation Config",
    reloadDefaults: "Defaults",
    loadConfig: "Load",
    saveConfig: "Save",
    modelsBtn: "Models",
    modelsTitle: "Model Management",
    modelDir: "Model Directory",
    modelDirUnset: "Not set (choose a folder outside the repo)",
    modelDirHint: "The portable package wipes its bundled folder on every rebuild, causing repeated downloads. Pick a fixed folder outside the repo to store models. Downloading and translating are blocked until this is set.",
    chooseDir: "Choose Folder",
    autoDownload: "Auto-download missing models on startup",
    close: "Close",
    modelsStatusTitle: "Model Status",
    refresh: "Refresh",
    modelsDirNotSet: "Model directory not set; status unavailable. Choose a folder above first.",
    modelKindDetector: "Detection",
    modelKindOcr: "OCR",
    modelKindInpainter: "Inpainting",
    modelKindUpscaler: "Upscaling",
    modelReady: "Ready",
    modelMissing: "Missing",
    modelFilesCount: "{ready}/{total} files",
    modelDownload: "Download",
    modelDownloading: "Downloading…",
    modelDownloadMissingAll: "Download all missing",
    modelStatusLoading: "Checking model status…",
    modelStatusError: "Failed to get model status: {err}",
    tabTranslation: "Translation",
    tabDetectionOcr: "Detection / OCR",
    tabInpaintMask: "Inpaint / Mask",
    tabUpscaleRender: "Upscale / Render",
    translator: "Translator",
    targetLang: "Target Language",
    provider: "Provider",
    baseUrl: "Base URL",
    apiKey: "API Key",
    modelName: "Model Name",
    translationSection: "Translation",
    detectorSection: "Detection",
    ocrSection: "OCR",
    maskSection: "Mask and Inpainting",
    upscalerSection: "Upscaling",
    openaiSection: "OpenAI Compatible",
    filterLang: "Skip Languages",
    preDict: "Pre Dictionary",
    postDict: "Post Dictionary",
    detector: "Detector",
    detectSize: "Detect Size",
    unclipRatio: "Unclip Ratio",
    textThreshold: "Text Threshold",
    boxThreshold: "Box Threshold",
    preInvert: "Invert",
    preGamma: "Gamma Correct",
    preRotate: "Rotate",
    preAutoRotate: "Auto Rotate",
    ocrModel: "OCR Model",
    minTextLength: "Min Text Length",
    ocrProb: "OCR Probability",
    beamSize: "Beam Width",
    filterText: "Filter Text",
    maskMethod: "Mask Method",
    ignoreBubble: "Ignore Bubble",
    dilationOffset: "Dilation Offset",
    kernelSize: "Kernel Size",
    furigana: "Furigana Mask",
    inpainter: "Inpainter",
    inpaintingSize: "Inpainting Size",
    maskSource: "Overlay Mask",
    inpaintColor: "Fill Color",
    upscaler: "Upscaler",
    upscalePatch: "Patch Size",
    upscalePadding: "Padding",
    timeoutSecs: "Timeout Seconds",
    temperature: "Temperature",
    topP: "Top P",
    systemPrompt: "System Prompt",
    userPrompt: "User Prompt Template",
    advancedJson: "Advanced JSON",
    readyTitle: "Ready",
    readyText: "Choose input and output directory to begin.",
    start: "Start Translating",
    running: "Translating…",
    resultTitle: "Preview and Export",
    completedTitle: "Completed",
    previewHint: "Click an item on the left to preview.",
    editEnter: "Edit Text",
    editExit: "Done",
    editHintIdle: "Click a box: double-click to edit · drag to move",
    editEntered: "Edit mode on",
    editApplied: "Typeset updated",
    editFailed: "Edit failed",
    resultStatsEmpty: "No results",
    resultStats: "Done {done} · Failed {failed} · Skipped {skipped}",
    resultEmpty: "No results yet. Finished images can be previewed and exported after translation.",
    selectAllResults: "Select All",
    deselectAllResults: "Deselect All",
    exportSelected: "Export Selected",
    preview: "Preview",
    exported: "Exported",
    exportNeedSelection: "Select at least one result first.",
    exportNeedDir: "Choose an export directory first.",
    remove: "Remove",
    logTitle: "Run Log",
    logResizeHint: "Drag to resize the run log",
    previewResizeHint: "Drag to resize the preview",
    hidePreview: "Hide preview",
    showPreview: "Show preview",
    clearLog: "Clear",
    logEmpty: "No logs yet",
    copy: "Copy",
    expand: "Expand",
    collapse: "Collapse",
    copied: "Copied",
    selected: "selected",
    added: "added",
    folderSelected: "Folder added",
    outputSelected: "Output directory set",
    defaultsLoaded: "Default settings loaded",
    configLoaded: "Config loaded",
    configSaved: "Config saved",
    starting: "Job sent",
    backendPending: "Translation is running",
    progressIdle: "Idle",
    progressPreparing: "Preparing models",
    progressRunning: "Processing",
    progressDone: "Done",
    jsonError: "Invalid JSON settings",
    openingImages: "Opening image picker…",
    openingFolder: "Opening folder picker…",
    openingOutput: "Opening output directory picker…",
    statusDone: "Done",
    statusFailed: "Failed",
    statusSkipped: "Skipped",
    statusPartial: "Partial",
    ipcUnavailable: "IPC unavailable",
    backendReady: "Backend ready",
    updateBtn: "Check for Updates",
    updateTitle: "App Update",
    updateCheck: "Check for updates",
    updateChecking: "Checking for updates…",
    updateCurrentVersion: "Current version",
    updateLatestVersion: "Latest version",
    updateUpToDate: "You're on the latest version.",
    updateAvailable: "A new version {version} is available.",
    updateNoCompatibleAsset: "A new version is available, but no portable package matches this build.",
    updateCheckFailed: "Update check failed: {err}",
    updateViewNotes: "View release notes",
    updateReleaseNotes: "Release notes",
    updateAssetSize: "Download size",
    updateDownloadBtn: "Download update",
    updateDownloading: "Downloading update…",
    updateStaged: "Update downloaded, ready to install.",
    updateInstallBtn: "Install and restart",
    updateInstalling: "Restarting to finish the update…",
    updateDownloadFailed: "Update download failed: {err}",
    updateInstallFailed: "Failed to start installer: {err}",
    updateLinkCopied: "Link copied",
  },
};

const providerBaseUrls = {
  OpenAI: "https://api.openai.com/v1",
  DeepSeek: "https://api.deepseek.com/v1",
  OpenRouter: "https://openrouter.ai/api/v1",
  SiliconFlow: "https://api.siliconflow.cn/v1",
  DashScope: "https://dashscope.aliyuncs.com/compatible-mode/v1",
  Moonshot: "https://api.moonshot.cn/v1",
  Zhipu: "https://open.bigmodel.cn/api/paas/v4",
};

const IMAGE_EXTENSIONS = new Set([
  ".png",
  ".jpg",
  ".jpeg",
  ".webp",
  ".bmp",
  ".gif",
  ".tif",
  ".tiff",
  ".avif",
]);

const LOG_SUMMARY_LIMIT = 140;
const PATH_DISPLAY_LIMIT = 56;
const CUDA_SUMMARY_LIMIT = 96;
const PREVIEW_HEIGHT_KEY = "mitWebviewPreviewHeight";
const PREVIEW_COLLAPSE_KEY = "mitWebviewPreviewCollapsed";
const PREVIEW_HEIGHT_MIN = 120;
const PREVIEW_HEIGHT_DEFAULT = 300;
const FILMSTRIP_WIDTH_KEY = "mitFilmstripWidth";
const FILMSTRIP_WIDTH_MIN = 160;
const FILMSTRIP_WIDTH_MAX = 520;
const RESULTS_HEIGHT_KEY = "mitResultsHeight";
const RESULTS_HEIGHT_MIN = 96;
const OUTPUT_DIR_KEY = "mitOutputDir";

const ICON_CHEVRON =
  '<svg viewBox="0 0 24 24" aria-hidden="true"><path d="m9 6 6 6-6 6" /></svg>';
const ICON_FOLDER =
  '<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M3 7.5A2.5 2.5 0 0 1 5.5 5H10l2 2h6.5A2.5 2.5 0 0 1 21 9.5v7A2.5 2.5 0 0 1 18.5 19h-13A2.5 2.5 0 0 1 3 16.5v-9Z" /></svg>';
const ICON_FILE =
  '<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M13 3H6.5A1.5 1.5 0 0 0 5 4.5v15A1.5 1.5 0 0 0 6.5 21h11a1.5 1.5 0 0 0 1.5-1.5V9l-6-6Z" /><path d="M13 3v6h6" /></svg>';

const state = {
  lang: localStorage.getItem("mitWebviewLang") || "zh",
  theme: localStorage.getItem("mitWebviewTheme") || "dark",
  tree: [],
  // Translation selection = checked image files. Keyed by normalized path,
  // value is the original on-disk path (what the backend needs).
  checked: new Map(),
  // Cache of a folder's full recursive image leaves (original paths), so the
  // tristate checkbox can be computed without re-enumerating on every render.
  folderLeaves: new Map(),
  activePath: "",
  preview: "",
  filter: "",
  visibleNodes: [],
  contextMenu: null,
  outputDir: "",
  results: [],
  selectedResults: new Set(),
  // P5 editable typeset. editData = { path, width, height, regions }; canvasView is
  // the live pan/zoom transform of the current preview (for screen<->image mapping).
  editMode: false,
  editData: null,
  editSelected: null,
  editEditor: null,
  canvasView: null,
  settings: null,
  requestId: 0,
  pending: new Map(),
  isRunning: false,
  cudaErrorExpanded: false,
  gpuStatus: null,
  appVersion: "",
  // App self-update panel. status drives the rendered branch:
  // idle | checking | uptodate | available | error | downloading | staged | installing
  update: { status: "idle", info: null, staged: null, error: "", notesExpanded: false },
};

const els = {
  langToggle: document.getElementById("langToggle"),
  themeToggle: document.getElementById("themeToggle"),
  backendBadge: document.getElementById("backendBadge"),
  pickImages: document.getElementById("pickImages"),
  pickFolder: document.getElementById("pickFolder"),
  clearInputs: document.getElementById("clearInputs"),
  pickOutputDir: document.getElementById("pickOutputDir"),
  outputDirLabel: document.getElementById("outputDirLabel"),
  outputDirReadout: document.getElementById("outputDirReadout"),
  outputFormat: document.getElementById("outputFormat"),
  textDirection: document.getElementById("textDirection"),
  providerStatus: document.getElementById("providerStatus"),
  cudaErrorWrap: document.getElementById("cudaErrorWrap"),
  cudaErrorSummary: document.getElementById("cudaErrorSummary"),
  cudaErrorToggle: document.getElementById("cudaErrorToggle"),
  cudaErrorDetail: document.getElementById("cudaErrorDetail"),
  deviceModeGroup: document.getElementById("deviceModeGroup"),
  deviceModeHint: document.getElementById("deviceModeHint"),
  deviceBanner: document.getElementById("deviceBanner"),
  debugMode: document.getElementById("debugMode"),
  maxParallelImages: document.getElementById("maxParallelImages"),
  maxParallelGpuJobs: document.getElementById("maxParallelGpuJobs"),
  inputList: document.getElementById("inputList"),
  treeSearch: document.getElementById("treeSearch"),
  inputStats: document.getElementById("inputStats"),
  translator: document.getElementById("translator"),
  targetLang: document.getElementById("targetLang"),
  filterLang: document.getElementById("filterLang"),
  preDict: document.getElementById("preDict"),
  postDict: document.getElementById("postDict"),
  detector: document.getElementById("detector"),
  detectSize: document.getElementById("detectSize"),
  unclipRatio: document.getElementById("unclipRatio"),
  textThreshold: document.getElementById("textThreshold"),
  boxThreshold: document.getElementById("boxThreshold"),
  preInvert: document.getElementById("preInvert"),
  preGamma: document.getElementById("preGamma"),
  preRotate: document.getElementById("preRotate"),
  preAutoRotate: document.getElementById("preAutoRotate"),
  ocrModel: document.getElementById("ocrModel"),
  minTextLength: document.getElementById("minTextLength"),
  ocrProb: document.getElementById("ocrProb"),
  beamSize: document.getElementById("beamSize"),
  filterText: document.getElementById("filterText"),
  maskMethod: document.getElementById("maskMethod"),
  ignoreBubble: document.getElementById("ignoreBubble"),
  dilationOffset: document.getElementById("dilationOffset"),
  kernelSize: document.getElementById("kernelSize"),
  furigana: document.getElementById("furigana"),
  inpainter: document.getElementById("inpainter"),
  inpaintingSize: document.getElementById("inpaintingSize"),
  maskSource: document.getElementById("maskSource"),
  inpaintColor: document.getElementById("inpaintColor"),
  upscaler: document.getElementById("upscaler"),
  upscalePatch: document.getElementById("upscalePatch"),
  upscalePadding: document.getElementById("upscalePadding"),
  provider: document.getElementById("provider"),
  baseUrl: document.getElementById("baseUrl"),
  apiKey: document.getElementById("apiKey"),
  modelName: document.getElementById("modelName"),
  timeoutSecs: document.getElementById("timeoutSecs"),
  temperature: document.getElementById("temperature"),
  topP: document.getElementById("topP"),
  systemPrompt: document.getElementById("systemPrompt"),
  userPrompt: document.getElementById("userPrompt"),
  translatorHint: document.getElementById("translatorHint"),
  ocrHint: document.getElementById("ocrHint"),
  detectorHint: document.getElementById("detectorHint"),
  inpainterHint: document.getElementById("inpainterHint"),
  upscalerHint: document.getElementById("upscalerHint"),
  openaiBlock: document.getElementById("openaiBlock"),
  settingsJson: document.getElementById("settingsJson"),
  reloadDefaults: document.getElementById("reloadDefaults"),
  loadConfig: document.getElementById("loadConfig"),
  saveConfig: document.getElementById("saveConfig"),
  openModels: document.getElementById("openModels"),
  openUpdate: document.getElementById("openUpdate"),
  appUpdate: document.getElementById("appUpdate"),
  modelsModal: document.getElementById("modelsModal"),
  closeModels: document.getElementById("closeModels"),
  modelDir: document.getElementById("modelDir"),
  pickModelDir: document.getElementById("pickModelDir"),
  autoDownload: document.getElementById("autoDownload"),
  modelsStatus: document.getElementById("modelsStatus"),
  gpuRuntime: document.getElementById("gpuRuntime"),
  startTranslation: document.getElementById("startTranslation"),
  statusTitle: document.getElementById("statusTitle"),
  statusText: document.getElementById("statusText"),
  progressBar: document.getElementById("progressBar"),
  progressLabel: document.getElementById("progressLabel"),
  selectAllResults: document.getElementById("selectAllResults"),
  exportSelected: document.getElementById("exportSelected"),
  resultStats: document.getElementById("resultStats"),
  filmstripResults: document.getElementById("filmstripResults"),
  results: document.getElementById("results"),
  canvasStage: document.getElementById("canvasStage"),
  canvasEditBar: document.getElementById("canvasEditBar"),
  toggleEdit: document.getElementById("toggleEdit"),
  exitEdit: document.getElementById("exitEdit"),
  editHint: document.getElementById("editHint"),
  filmstripResizer: document.getElementById("filmstripResizer"),
  resultsResizer: document.getElementById("resultsResizer"),
  logList: document.getElementById("logList"),
  clearLog: document.getElementById("clearLog"),
  canvas: document.querySelector(".canvas"),
  previewResizer: document.getElementById("previewResizer"),
  togglePreview: document.getElementById("togglePreview"),
};

window.MIT_BRIDGE = {
  resolve(response) {
    const pending = state.pending.get(response.id);
    if (!pending) return;
    state.pending.delete(response.id);
    if (response.ok) {
      pending.resolve(response.data);
    } else {
      pending.reject(new Error(response.error || "IPC request failed"));
    }
  },
  emit(name, payload) {
    if (name === "log") {
      addLog(payload.level || "info", payload.message || "");
    } else if (name === "progress") {
      updateProgress(payload || {});
    }
  },
};

function t(key, vars = {}) {
  const template = i18n[state.lang][key] || key;
  return Object.entries(vars).reduce(
    (text, [name, value]) => text.replaceAll(`{${name}}`, String(value)),
    template,
  );
}

const DEVICE_MODES = ["auto", "cuda", "cpu"];
const DEVICE_MODE_HINT_KEYS = {
  auto: "deviceModeAutoHint",
  cuda: "deviceModeCudaHint",
  cpu: "deviceModeCpuHint",
};

function getDeviceMode() {
  const value = els.deviceModeGroup?.querySelector("input:checked")?.value;
  return DEVICE_MODES.includes(value) ? value : "auto";
}

function setDeviceMode(mode) {
  const value = DEVICE_MODES.includes(mode) ? mode : "auto";
  const target = els.deviceModeGroup?.querySelector(`input[value="${value}"]`);
  if (target) target.checked = true;
  updateDeviceModeHint();
}

function updateDeviceModeHint() {
  if (!els.deviceModeHint) return;
  els.deviceModeHint.textContent = t(DEVICE_MODE_HINT_KEYS[getDeviceMode()]);
}

// Contextual nudge under the device selector: Auto + GPU-present-but-DLL-missing
// gets a gentle prompt; Force-CUDA-but-unavailable gets a hard warning. Other
// states (ready, CPU-only build, no GPU under Auto) show nothing.
function updateDeviceBanner() {
  const el = els.deviceBanner;
  if (!el) return;
  el.innerHTML = "";
  el.className = "device-banner hidden";
  const status = state.gpuStatus;
  if (
    !status ||
    status.recommendation === "ready" ||
    status.recommendation === "cpu_only_build"
  ) {
    return;
  }
  const mode = getDeviceMode();
  const driverOld = status.recommendation === "need_driver_update";
  let tone;
  let msgKey;
  if (mode === "cuda") {
    tone = "error";
    msgKey = driverOld
      ? "deviceBannerCudaDriverOld"
      : "deviceBannerCudaUnavailable";
  } else if (mode === "auto" && status.recommendation === "need_download_dll") {
    tone = "warn";
    msgKey = "deviceBannerAutoNeedDll";
  } else if (mode === "auto" && driverOld) {
    tone = "warn";
    msgKey = "deviceBannerAutoDriverOld";
  } else {
    return;
  }
  el.className = `device-banner device-banner-${tone}`;
  const text = document.createElement("span");
  text.className = "device-banner-text";
  text.textContent = t(msgKey, { min: status.minDriver });
  const open = document.createElement("button");
  open.type = "button";
  open.className = "link-button";
  open.dataset.deviceBannerOpen = "1";
  open.textContent = t("deviceBannerOpen");
  el.append(text, open);
}

// One-line "what is this and when to pick it" guidance per model option.
const MODEL_HINTS = {
  translator: {
    Sugoi: { zh: "日→英本地模型，离线免费、质量好；仅日译英。", en: "Local JA→EN model, offline and free, solid quality; Japanese to English only." },
    OpenAICompatible: { zh: "调用 LLM API，质量最高、可译任意语言、可定制 prompt；需 API Key、联网、按量计费。", en: "Calls an LLM API: top quality, any language pair, customizable prompt; needs API key, network, pay-per-use." },
    Google: { zh: "Google 在线翻译，需 GOOGLE_API_KEY 环境变量。", en: "Google online translation; needs the GOOGLE_API_KEY env var." },
    Deepl: { zh: "DeepL 在线翻译，质量好，需 DEEPL_AUTH_KEY。", en: "DeepL online translation, strong quality; needs DEEPL_AUTH_KEY." },
    Baidu: { zh: "百度在线翻译，需 BAIDU_APP_ID / BAIDU_SECRET_KEY。", en: "Baidu online; needs BAIDU_APP_ID / BAIDU_SECRET_KEY." },
    Caiyun: { zh: "彩云在线翻译，需 CAIYUN_TOKEN。", en: "Caiyun online; needs CAIYUN_TOKEN." },
    Youdao: { zh: "有道在线翻译，需 YOUDAO_APP_KEY / YOUDAO_SECRET_KEY。", en: "Youdao online; needs YOUDAO_APP_KEY / YOUDAO_SECRET_KEY." },
    Papago: { zh: "Papago 在线翻译，韩语较强。", en: "Papago online translation, strong for Korean." },
    MyMemory: { zh: "免费在线翻译，额度有限、质量一般。", en: "Free online translation; limited quota, modest quality." },
    NLLBSmallDistilled: { zh: "NLLB 多语种本地模型（小），快但质量一般。", en: "NLLB multilingual local (small): fast, modest quality." },
    NLLBBase: { zh: "NLLB 多语种本地模型（中）。", en: "NLLB multilingual local (base)." },
    NLLBLarge: { zh: "NLLB 多语种本地模型（大），质量高但慢、吃显存。", en: "NLLB multilingual local (large): higher quality, slower, more VRAM." },
    M2M100Small: { zh: "M2M100 多语种本地模型（小），多对多直译。", en: "M2M100 multilingual local (small), direct many-to-many." },
    M2M100Large: { zh: "M2M100 多语种本地模型（大），质量高但更重。", en: "M2M100 multilingual local (large), higher quality, heavier." },
    MBart: { zh: "mBART-50 多语种本地模型。", en: "mBART-50 multilingual local model." },
    JParaCrawlSmall: { zh: "JParaCrawl 日↔英本地模型（小）。", en: "JParaCrawl JA-EN local model (small)." },
    JParaCrawlBase: { zh: "JParaCrawl 日↔英本地模型（中）。", en: "JParaCrawl JA-EN local model (base)." },
    JParaCrawlLarge: { zh: "JParaCrawl 日↔英本地模型（大）。", en: "JParaCrawl JA-EN local model (large)." },
  },
  ocr: {
    Ocr48px: { zh: "质量高、含颜色预测；自回归较慢。追求速度可换 Ctc48px。", en: "High quality with color prediction; autoregressive and slower. Switch to Ctc48px for speed." },
    Ctc48px: { zh: "CTC 单次前向，快很多，质量略低。", en: "CTC single forward pass: much faster, slightly lower quality." },
    MangaOcr: { zh: "专为日漫手写体优化，日文识别强；较重。", en: "Tuned for Japanese manga handwriting; heavier." },
    Native: { zh: "通用 OCR，印刷体可用，手写体弱。", en: "Generic OCR: okay for print, weak on handwriting." },
    Tesseract: { zh: "Tesseract 通用 OCR，需额外环境。", en: "Tesseract generic OCR; needs extra setup." },
  },
  detector: {
    DBNet: { zh: "漫画文字检测综合最好。", en: "Best all-round detector for manga text." },
    Paddle: { zh: "PaddleOCR 检测，偏印刷体场景。", en: "PaddleOCR detector; leans toward print." },
    Ctd: { zh: "漫画专用检测，对气泡/竖排友好。", en: "Comic-specific detector; good for bubbles and vertical text." },
  },
  inpainter: {
    LamaAot: { zh: "速度/质量平衡。", en: "Balanced speed and quality." },
    LamaLarge: { zh: "质量更高，更慢、更吃显存。", en: "Higher quality, slower, more VRAM." },
    LamaMpe: { zh: "LaMa 变体。", en: "LaMa variant." },
  },
  upscaler: {
    none: { zh: "不放大，最快。低分辨率原图才考虑开启。", en: "No upscaling, fastest. Only enable for low-resolution sources." },
    Esrgan2x: { zh: "ESRGAN 2× 放大，更清晰但更慢。", en: "ESRGAN 2x: sharper but slower." },
    Esrgan4x: { zh: "ESRGAN 4× 放大，明显更慢、更吃显存。", en: "ESRGAN 4x: much slower, more VRAM." },
    EsrganAnime4x: { zh: "ESRGAN 动漫 4× 放大。", en: "ESRGAN anime 4x." },
    Anime4k: { zh: "Anime4K 放大，动漫专用。", en: "Anime4K upscaler, anime-tuned." },
    custom: { zh: "保留高级 JSON 中的自定义放大设置。", en: "Keep the custom upscaler from advanced JSON." },
  },
};

const RECOMMENDED = {
  translator: "Sugoi",
  ocr: "Ocr48px",
  detector: "DBNet",
  inpainter: "LamaAot",
  upscaler: "none",
};

function modelHintText(group, value) {
  const entry = MODEL_HINTS[group] && MODEL_HINTS[group][value];
  if (!entry) return "";
  const base = entry[state.lang] || entry.en || "";
  return value === RECOMMENDED[group] ? `${base} ${t("recommendedSuffix")}` : base;
}

// Refresh model hint lines, progressive disclosure, and dependent control states.
function refreshGuidance() {
  const pairs = [
    ["translator", els.translator, els.translatorHint],
    ["ocr", els.ocrModel, els.ocrHint],
    ["detector", els.detector, els.detectorHint],
    ["inpainter", els.inpainter, els.inpainterHint],
    ["upscaler", els.upscaler, els.upscalerHint],
  ];
  for (const [group, select, hint] of pairs) {
    if (!select || !hint) continue;
    hint.textContent = modelHintText(group, select.value);
    hint.classList.toggle("is-recommended", select.value === RECOMMENDED[group]);
  }
  if (els.openaiBlock) {
    els.openaiBlock.hidden = els.translator.value !== "OpenAICompatible";
  }
  const isSugoi = els.translator.value === "Sugoi";
  if (els.targetLang) {
    els.targetLang.disabled = isSugoi;
    els.targetLang.title = isSugoi ? t("targetLangSugoiNote") : "";
  }
}

function applyTheme(theme) {
  const normalized = theme === "light" ? "light" : "dark";
  state.theme = normalized;
  document.documentElement.dataset.theme = normalized;
  localStorage.setItem("mitWebviewTheme", normalized);
}

function applyLang() {
  document.documentElement.lang = state.lang === "zh" ? "zh-CN" : "en";
  document.querySelectorAll("[data-i18n]").forEach((node) => {
    node.textContent = t(node.dataset.i18n);
  });
  document.querySelectorAll("[data-i18n-title]").forEach((node) => {
    node.title = t(node.dataset.i18nTitle);
  });
  document.querySelectorAll("[data-i18n-placeholder]").forEach((node) => {
    node.placeholder = t(node.dataset.i18nPlaceholder);
  });
  els.langToggle.textContent = state.lang === "zh" ? "English" : "中文";
  if (els.cudaErrorToggle && !els.cudaErrorWrap.classList.contains("hidden")) {
    els.cudaErrorToggle.textContent = state.cudaErrorExpanded ? t("cudaHide") : t("cudaDetails");
  }
  const startLabel = els.startTranslation.querySelector(".start-label");
  if (startLabel && !state.isRunning) {
    startLabel.textContent = t("start");
  }
  renderTree();
  renderResults();
  renderCanvas();
  renderLogEmptyState();
  renderAppUpdate();
  refreshGuidance();
}

function invoke(kind, payload = {}) {
  const id = `req_${Date.now()}_${++state.requestId}`;
  const message = JSON.stringify({ id, kind, payload });
  return new Promise((resolve, reject) => {
    state.pending.set(id, { resolve, reject });
    const ipc =
      window.ipc && typeof window.ipc.postMessage === "function"
        ? window.ipc
        : window.chrome?.webview && typeof window.chrome.webview.postMessage === "function"
          ? window.chrome.webview
          : null;
    if (!ipc) {
      state.pending.delete(id);
      reject(new Error("WebView IPC bridge is not available."));
      return;
    }
    ipc.postMessage(message);
  });
}

function summarizeText(text, limit = LOG_SUMMARY_LIMIT) {
  const normalized = String(text || "").replace(/\s+/g, " ").trim();
  if (normalized.length <= limit) return normalized;
  return `${normalized.slice(0, limit)}…`;
}

function truncateMiddle(text, limit = PATH_DISPLAY_LIMIT) {
  const value = String(text || "");
  if (value.length <= limit) return value;
  const head = Math.ceil((limit - 1) / 2);
  const tail = Math.floor((limit - 1) / 2);
  return `${value.slice(0, head)}…${value.slice(-tail)}`;
}

function pathBaseName(path) {
  return String(path).split(/[\\/]/).filter(Boolean).pop() || String(path);
}

function makeNode(path, name, isDir) {
  return {
    path,
    name,
    isDir,
    expanded: false,
    loaded: false,
    loading: false,
    children: isDir ? null : undefined,
  };
}

function findNode(path, nodes = state.tree) {
  for (const node of nodes) {
    if (node.path === path) return node;
    if (node.children) {
      const found = findNode(path, node.children);
      if (found) return found;
    }
  }
  return null;
}

function isChecked(path) {
  return state.checked.has(normalizePathKey(path));
}

function setChecked(path, on) {
  const key = normalizePathKey(path);
  if (on) state.checked.set(key, path);
  else state.checked.delete(key);
}

function isUnder(child, parent) {
  if (!child || !parent) return false;
  return normalizePathKey(child).startsWith(`${normalizePathKey(parent)}/`);
}

// Drop a removed path (and anything beneath it) from the selection and caches.
function pruneSelection(path) {
  const key = normalizePathKey(path);
  const prefix = `${key}/`;
  for (const k of [...state.checked.keys()]) {
    if (k === key || k.startsWith(prefix)) state.checked.delete(k);
  }
  for (const k of [...state.folderLeaves.keys()]) {
    if (k === key || k.startsWith(prefix)) state.folderLeaves.delete(k);
  }
}

// Recursively list a folder's image files (original paths), caching the result
// for the folder and every nested subfolder so tristate stays cheap.
async function enumerateFolderImages(dirPath) {
  const cached = state.folderLeaves.get(normalizePathKey(dirPath));
  if (cached) return cached;
  const data = await invoke("listDir", { path: dirPath });
  let leaves = [];
  for (const entry of data.entries || []) {
    if (entry.is_dir) {
      leaves = leaves.concat(await enumerateFolderImages(entry.path));
    } else if (entry.is_image) {
      leaves.push(entry.path);
    }
  }
  state.folderLeaves.set(normalizePathKey(dirPath), leaves);
  return leaves;
}

// Count {checked, total} image leaves under a folder, preferring the live
// loaded subtree and falling back to the recursive enumeration cache.
function folderLeafCounts(node) {
  if (node.loaded && Array.isArray(node.children)) {
    let checked = 0;
    let total = 0;
    for (const child of node.children) {
      if (child.isDir) {
        const sub = folderLeafCounts(child);
        checked += sub.checked;
        total += sub.total;
      } else {
        total += 1;
        if (isChecked(child.path)) checked += 1;
      }
    }
    return { checked, total };
  }
  const leaves = state.folderLeaves.get(normalizePathKey(node.path));
  if (leaves) {
    let checked = 0;
    for (const p of leaves) if (isChecked(p)) checked += 1;
    return { checked, total: leaves.length };
  }
  return { checked: 0, total: 0 };
}

// Tristate for a folder checkbox: "all" / "partial" / "none".
function folderCheckState(node) {
  const { checked, total } = folderLeafCounts(node);
  if (total === 0 || checked === 0) return "none";
  if (checked >= total) return "all";
  return "partial";
}

function matchesFilter(node) {
  return !state.filter || node.name.toLowerCase().includes(state.filter);
}

// Whether a node should render under the active name filter. A folder also shows
// if any loaded descendant matches (so the path to a match stays visible). Lazy,
// unloaded subtrees can only match by the folder's own name.
function nodeVisible(node) {
  if (!state.filter) return true;
  if (matchesFilter(node)) return true;
  if (node.isDir && Array.isArray(node.children)) {
    return node.children.some((child) => nodeVisible(child));
  }
  return false;
}

// Parent node of `path`, or null for a root; undefined if not found.
function findParent(path, nodes = state.tree, parent = null) {
  for (const node of nodes) {
    if (node.path === path) return parent;
    if (node.children) {
      const found = findParent(path, node.children, node);
      if (found !== undefined) return found;
    }
  }
  return undefined;
}

function countInputKinds() {
  let files = 0;
  let folders = 0;
  state.tree.forEach((node) => {
    if (node.isDir) folders += 1;
    else files += 1;
  });
  return { total: state.tree.length, files, folders };
}

function renderInputStats() {
  const counts = countInputKinds();
  els.inputStats.textContent =
    counts.total === 0
      ? t("inputStatsEmpty")
      : t("inputStats", { ...counts, checked: state.checked.size });
}

async function copyText(text) {
  try {
    await navigator.clipboard.writeText(text);
    return true;
  } catch (_) {
    const area = document.createElement("textarea");
    area.value = text;
    area.style.position = "fixed";
    area.style.left = "-9999px";
    document.body.append(area);
    area.select();
    const ok = document.execCommand("copy");
    area.remove();
    return ok;
  }
}

function renderLogEmptyState() {
  if (els.logList.children.length) {
    els.logList.classList.remove("is-empty");
    return;
  }
  els.logList.classList.add("is-empty");
  els.logList.textContent = t("logEmpty");
}

function addLog(level, message) {
  if (els.logList.classList.contains("is-empty")) {
    els.logList.classList.remove("is-empty");
    els.logList.textContent = "";
  }

  const fullMessage = String(message || "");
  const summary = summarizeText(fullMessage);
  const isLong = fullMessage.length > LOG_SUMMARY_LIMIT;

  const entry = document.createElement("div");
  entry.className = "log-entry";
  entry.dataset.level = level;

  const head = document.createElement("div");
  head.className = "log-head";

  const time = document.createElement("span");
  time.className = "log-time";
  time.textContent = new Date().toLocaleTimeString();

  const summaryNode = document.createElement("span");
  summaryNode.className = "log-summary";
  summaryNode.textContent = summary;

  const actions = document.createElement("div");
  actions.className = "log-actions";

  const copyButton = document.createElement("button");
  copyButton.type = "button";
  copyButton.className = "tiny-button";
  copyButton.textContent = t("copy");
  copyButton.addEventListener("click", async () => {
    const ok = await copyText(fullMessage);
    if (ok) copyButton.textContent = t("copied");
  });

  actions.append(copyButton);

  let body = null;
  if (isLong) {
    const expandButton = document.createElement("button");
    expandButton.type = "button";
    expandButton.className = "tiny-button";
    expandButton.textContent = t("expand");
    expandButton.addEventListener("click", () => {
      const expanded = !body.classList.contains("hidden");
      body.classList.toggle("hidden", expanded);
      expandButton.textContent = expanded ? t("expand") : t("collapse");
    });
    actions.append(expandButton);

    body = document.createElement("pre");
    body.className = "log-body hidden";
    body.textContent = fullMessage;
  }

  head.append(time, summaryNode, actions);
  entry.append(head);
  if (body) entry.append(body);

  els.logList.append(entry);
  els.logList.scrollTop = els.logList.scrollHeight;
}

function setStatus(title, text) {
  els.statusTitle.textContent = title;
  els.statusText.textContent = text;
}

function setRunningState(running) {
  state.isRunning = running;
  els.startTranslation.disabled = running;
  els.startTranslation.classList.toggle("is-loading", running);
  const label = els.startTranslation.querySelector(".start-label");
  if (label) label.textContent = running ? t("running") : t("start");
}

function updateProgress(payload) {
  const current = Number(payload.current ?? 0);
  const total = Number(payload.total ?? 0);
  const percent =
    typeof payload.percent === "number"
      ? payload.percent
      : total > 0
        ? Math.round((current / total) * 100)
        : 0;
  const clamped = Math.max(0, Math.min(100, percent));
  els.progressBar.style.width = `${clamped}%`;
  const message = payload.message || t("progressIdle");
  els.progressLabel.textContent =
    total > 0 ? `${message} · ${current}/${total} · ${clamped}%` : message;
}

function renderTree() {
  renderInputStats();
  els.inputList.innerHTML = "";

  if (!state.tree.length) {
    els.inputList.classList.add("is-empty");
    els.inputList.textContent = t("treeEmpty");
    return;
  }

  els.inputList.classList.remove("is-empty");
  // Rebuilt in render order so keyboard nav has the exact list of visible rows.
  state.visibleNodes = [];
  const root = document.createElement("div");
  root.className = "tree";
  const roots = state.tree.filter((node) => nodeVisible(node));
  if (!roots.length) {
    els.inputList.classList.add("is-empty");
    els.inputList.textContent = t("treeNoMatch");
    return;
  }
  roots.forEach((node) => root.append(renderNode(node, 0, true)));
  els.inputList.append(root);
  // Indeterminate can't be set via an HTML attribute; apply it post-render.
  els.inputList
    .querySelectorAll('input.tree-check[data-check="partial"]')
    .forEach((cb) => {
      cb.indeterminate = true;
    });
}

function renderNode(node, depth, isRoot) {
  state.visibleNodes.push(node);

  const wrap = document.createElement("div");
  wrap.className = "tree-node";
  wrap.style.setProperty("--depth", String(depth));

  const row = document.createElement("div");
  row.className = `tree-row${node.isDir ? " is-dir" : " is-file"}`;
  if (node.path === state.activePath) row.classList.add("is-active");
  row.dataset.path = node.path;
  row.title = node.path;

  // A filter auto-expands folders that contain a match so the path stays visible.
  const expanded = node.isDir && (node.expanded || (state.filter && !matchesFilter(node)));
  const chevron = node.isDir
    ? `<span class="tree-chevron${expanded ? " is-open" : ""}" data-action="toggle">${ICON_CHEVRON}</span>`
    : '<span class="tree-chevron is-leaf"></span>';
  const checkState = node.isDir
    ? folderCheckState(node)
    : isChecked(node.path)
      ? "all"
      : "none";
  const checkbox = `<input type="checkbox" class="tree-check" data-check="${checkState}"${checkState === "all" ? " checked" : ""}${node.checkLoading ? " disabled" : ""}>`;
  const icon = `<span class="tree-icon">${node.isDir ? ICON_FOLDER : ICON_FILE}</span>`;
  const name = `<span class="tree-name"${node.isDir ? ' data-action="toggle"' : ""}>${escapeHtml(node.name)}</span>`;
  const remove = isRoot
    ? `<button type="button" class="tiny-button tree-remove" data-action="remove">${escapeHtml(t("remove"))}</button>`
    : "";
  row.innerHTML = chevron + checkbox + icon + name + remove;
  wrap.append(row);

  if (node.isDir && expanded) {
    const childWrap = document.createElement("div");
    childWrap.className = "tree-children";
    if (node.loading) {
      childWrap.innerHTML = `<div class="tree-hint">${escapeHtml(t("treeLoading"))}</div>`;
    } else if (node.loaded && node.children && node.children.length) {
      node.children
        .filter((child) => nodeVisible(child))
        .forEach((child) => childWrap.append(renderNode(child, depth + 1, false)));
    } else if (node.loaded) {
      childWrap.innerHTML = `<div class="tree-hint">${escapeHtml(t("treeFolderEmpty"))}</div>`;
    }
    wrap.append(childWrap);
  }
  return wrap;
}

async function toggleNode(node) {
  if (!node.isDir) return;
  node.expanded = !node.expanded;
  if (node.expanded && !node.loaded && !node.loading) {
    node.loading = true;
    renderTree();
    try {
      const data = await invoke("listDir", { path: node.path });
      node.children = (data.entries || []).map((entry) =>
        makeNode(entry.path, entry.name, entry.is_dir),
      );
      node.loaded = true;
    } catch (err) {
      node.expanded = false;
      addLog("error", err.message);
    } finally {
      node.loading = false;
      renderTree();
    }
  } else {
    renderTree();
  }
}

// Checkbox toggle (independent of single-click preview). A folder check
// recursively enumerates its image leaves and selects/deselects them all.
async function toggleCheck(node, desired) {
  if (!node.isDir) {
    setChecked(node.path, desired);
    renderTree();
    return;
  }
  node.checkLoading = true;
  renderTree();
  try {
    const leaves = await enumerateFolderImages(node.path);
    leaves.forEach((p) => setChecked(p, desired));
  } catch (err) {
    addLog("error", err.message);
  } finally {
    node.checkLoading = false;
    renderTree();
  }
}

// Mark a node active (and, for files, preview it on the canvas). Used by both
// single-click and keyboard navigation; keeps the active row scrolled into view.
function setActiveNode(node, { preview = false, focus = true } = {}) {
  state.activePath = node.path;
  const showPreview = preview && !node.isDir;
  if (showPreview) state.preview = node.path;
  renderTree();
  if (showPreview) {
    renderResults(); // a tree preview deselects any active result row
    renderCanvas();
  }
  if (focus) els.inputList.focus({ preventScroll: true });
  scrollActiveIntoView();
}

function scrollActiveIntoView() {
  const row = els.inputList.querySelector(".tree-row.is-active");
  if (row) row.scrollIntoView({ block: "nearest" });
}

// Single-click a file row → select it and preview on the canvas.
function selectAndPreview(node) {
  setActiveNode(node, { preview: true });
}

// Keyboard navigation over the currently visible rows (↑/↓ move + preview,
// →/← expand/collapse or step in/out, Space toggles the checkbox).
function handleTreeKey(event) {
  const nodes = state.visibleNodes;
  if (!nodes.length) return;
  const index = nodes.findIndex((node) => node.path === state.activePath);
  const current = index >= 0 ? nodes[index] : null;

  switch (event.key) {
    case "ArrowDown": {
      event.preventDefault();
      const next = nodes[Math.min(nodes.length - 1, index + 1)] || nodes[0];
      setActiveNode(next, { preview: true });
      break;
    }
    case "ArrowUp": {
      event.preventDefault();
      const prev = index <= 0 ? nodes[0] : nodes[index - 1];
      setActiveNode(prev, { preview: true });
      break;
    }
    case "ArrowRight": {
      if (!current || !current.isDir) break;
      event.preventDefault();
      if (!current.expanded) {
        toggleNode(current);
      } else if (current.children && current.children.length) {
        const child = current.children.find((c) => nodeVisible(c));
        if (child) setActiveNode(child, { preview: true });
      }
      break;
    }
    case "ArrowLeft": {
      if (!current) break;
      event.preventDefault();
      if (current.isDir && current.expanded) {
        toggleNode(current);
      } else {
        const parent = findParent(current.path);
        if (parent) setActiveNode(parent, { preview: false });
      }
      break;
    }
    case " ":
    case "Spacebar": {
      if (!current) break;
      event.preventDefault();
      const desired = current.isDir ? folderCheckState(current) !== "all" : !isChecked(current.path);
      toggleCheck(current, desired);
      break;
    }
    case "Enter": {
      if (!current) break;
      event.preventDefault();
      if (current.isDir) toggleNode(current);
      else setActiveNode(current, { preview: true });
      break;
    }
    default:
      break;
  }
}

// ── Right-click context menu: reveal in Explorer + (roots) remove ──
function closeTreeContextMenu() {
  if (state.contextMenu) {
    state.contextMenu.remove();
    state.contextMenu = null;
  }
}

function showTreeContextMenu(node, x, y, isRoot) {
  closeTreeContextMenu();
  const menu = document.createElement("div");
  menu.className = "context-menu";
  const items = [{ label: t("revealInExplorer"), action: () => revealInExplorer(node.path) }];
  if (isRoot) items.push({ label: t("remove"), action: () => removeRoot(node.path) });
  items.forEach(({ label, action }) => {
    const button = document.createElement("button");
    button.type = "button";
    button.className = "context-menu-item";
    button.textContent = label;
    button.addEventListener("click", () => {
      closeTreeContextMenu();
      action();
    });
    menu.append(button);
  });
  menu.style.left = `${x}px`;
  menu.style.top = `${y}px`;
  document.body.append(menu);
  state.contextMenu = menu;
  // Nudge back on-screen if it would overflow the viewport edges.
  const rect = menu.getBoundingClientRect();
  if (rect.right > window.innerWidth) menu.style.left = `${window.innerWidth - rect.width - 6}px`;
  if (rect.bottom > window.innerHeight) menu.style.top = `${window.innerHeight - rect.height - 6}px`;
}

async function revealInExplorer(path) {
  try {
    await invoke("revealInExplorer", { path });
  } catch (err) {
    addLog("error", err.message);
  }
}

function addRoots(paths, isDir) {
  const before = state.tree.length;
  const seen = new Set(state.tree.map((node) => normalizePathKey(node.path)));
  (paths || []).forEach((path) => {
    if (!path) return;
    const key = normalizePathKey(path);
    if (seen.has(key)) return;
    seen.add(key);
    state.tree.push(makeNode(path, pathBaseName(path), isDir));
  });
  return state.tree.length - before;
}

function normalizePathKey(path) {
  return String(path || "").trim().replaceAll("\\", "/").toLowerCase();
}

function removeRoot(path) {
  state.tree = state.tree.filter((node) => node.path !== path);
  pruneSelection(path);
  if (state.activePath === path || isUnder(state.activePath, path)) state.activePath = "";
  if (state.preview === path || isUnder(state.preview, path)) {
    state.preview = "";
    renderCanvas();
  }
  renderTree();
  setStatus(t("selected"), `${state.tree.length} ${t("selected")}`);
}

function clearInputs() {
  state.tree = [];
  state.checked.clear();
  state.folderLeaves.clear();
  state.activePath = "";
  state.preview = "";
  renderTree();
  renderCanvas();
  setStatus(t("readyTitle"), t("readyText"));
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

function escapeAttr(value) {
  return escapeHtml(value).replaceAll("'", "&#39;");
}

function updateCudaError(diagnostics) {
  const errorText = diagnostics?.cuda_error || "";
  if (!errorText) {
    els.cudaErrorWrap.classList.add("hidden");
    els.cudaErrorDetail.classList.add("hidden");
    els.cudaErrorDetail.textContent = "";
    state.cudaErrorExpanded = false;
    return;
  }

  els.cudaErrorWrap.classList.remove("hidden");
  els.cudaErrorSummary.textContent = summarizeText(errorText, CUDA_SUMMARY_LIMIT);
  els.cudaErrorDetail.textContent = errorText;
  els.cudaErrorDetail.classList.toggle("hidden", !state.cudaErrorExpanded);
  els.cudaErrorToggle.textContent = state.cudaErrorExpanded ? t("cudaHide") : t("cudaDetails");
}

function updateProviderStatus(status) {
  const label = status || "CUDA unknown";
  els.providerStatus.textContent = label;
  els.providerStatus.dataset.status = label;
}

async function chooseImages() {
  try {
    setStatus(t("starting"), t("openingImages"));
    const data = await invoke("pickImages");
    const added = addRoots(data.paths || [], false);
    renderTree();
    setStatus(t("selected"), `${state.tree.length} ${t("selected")}`);
    addLog("info", `${t("added")}: ${added}; ${t("selected")}: ${state.tree.length}`);
  } catch (err) {
    addLog("error", err.message);
  }
}

async function chooseFolder() {
  try {
    setStatus(t("starting"), t("openingFolder"));
    const data = await invoke("pickFolder");
    const added = addRoots(data.paths || [], true);
    renderTree();
    setStatus(t("folderSelected"), `${state.tree.length} ${t("selected")}`);
    addLog("info", `${t("folderSelected")}: ${t("added")} ${added}; ${t("selected")}: ${state.tree.length}`);
  } catch (err) {
    addLog("error", err.message);
  }
}

function outputDirBaseName(dir) {
  return dir ? String(dir).split(/[\\/]/).filter(Boolean).pop() || dir : "";
}

// Reflect the chosen output dir in the toolbar button + inspector readout, and
// (optionally) persist it so the next launch remembers it.
function applyOutputDir(dir, persist = true) {
  state.outputDir = dir || "";
  els.outputDirLabel.textContent = outputDirBaseName(state.outputDir) || t("outputDir");
  els.pickOutputDir.title = state.outputDir || t("outputDirHint");
  els.pickOutputDir.classList.toggle("is-set", Boolean(state.outputDir));
  if (els.outputDirReadout) {
    els.outputDirReadout.textContent = state.outputDir || t("outputDirUnset");
  }
  if (persist) localStorage.setItem(OUTPUT_DIR_KEY, state.outputDir);
}

async function chooseOutputDir() {
  try {
    const data = await invoke("pickOutputDir");
    const dir = (data.paths || [])[0] || "";
    if (!dir) return;
    applyOutputDir(dir);
    setStatus(t("outputSelected"), truncateMiddle(dir, 72));
    addLog("info", `${t("outputSelected")}: ${dir}`);
  } catch (err) {
    addLog("error", err.message);
  }
}

async function loadDefaults() {
  try {
    const defaults = await invoke("defaults");
    applySettings(defaults);
    addLog("success", t("defaultsLoaded"));
  } catch (err) {
    addLog("error", err.message);
  }
}

async function loadConfig() {
  try {
    const config = await invoke("loadConfig");
    applySettings(config);
    addLog("success", t("configLoaded"));
  } catch (err) {
    addLog("error", err.message);
    await loadDefaults();
  }
}

async function saveConfig() {
  try {
    const settings = patchSettingsFromControls();
    const result = await invoke("saveConfig", { settings });
    applySettings(settings);
    addLog("success", `${t("configSaved")}: ${result.path || "config/app.json"}`);
    setStatus(t("configSaved"), result.path || "config/app.json");
  } catch (err) {
    setStatus(t("jsonError"), err.message);
    addLog("error", err.message);
  }
}

function applySettings(settings) {
  state.settings = settings || {};
  els.settingsJson.value = JSON.stringify(state.settings, null, 2);
  syncControlsFromSettings();
}

function syncControlsFromSettings() {
  const cfg = state.settings || {};
  const translation = cfg.translator?.target?.translator ? cfg.translator.target : null;
  const openai = cfg.translator?.openai_compatible || {};
  const render = cfg.render || {};
  const detector = cfg.detector || {};
  const detectorOptions = detector.options || {};
  const preprocessor = detector.preprocessor || {};
  const ocr = cfg.ocr || {};
  const maskRefinement = cfg.mask_refinement || {};
  const inpainter = cfg.inpainter || {};
  const upscaler = cfg.upscaler || {};
  els.translator.value = translation?.translator || "Sugoi";
  setSelectValue(els.targetLang, normalizeLanguageCode(translation?.target || "en"));
  els.filterLang.value = Array.isArray(cfg.translator?.filter_lang)
    ? cfg.translator.filter_lang.join(", ")
    : "";
  els.preDict.value = cfg.translator?.pre_dict || "";
  els.postDict.value = cfg.translator?.post_dict || "";
  els.textDirection.value = render.text_direction
    ? String(render.text_direction).toLowerCase()
    : "auto";
  els.detector.value = detector.detector || "DBNet";
  els.detectSize.value = detectorOptions.detect_size ?? 2048;
  els.unclipRatio.value = detectorOptions.unclip_ratio ?? 2.3;
  els.textThreshold.value = detectorOptions.text_threshold ?? 0.5;
  els.boxThreshold.value = detectorOptions.box_threshold ?? 0.7;
  els.preInvert.checked = Boolean(preprocessor.invert);
  els.preGamma.checked = Boolean(preprocessor.gamma_correct);
  els.preRotate.checked = Boolean(preprocessor.rotate);
  els.preAutoRotate.checked = Boolean(preprocessor.auto_rotate);
  els.ocrModel.value = ocr.ocr || "Ocr48px";
  els.minTextLength.value = ocr.min_text_length ?? 1;
  els.ocrProb.value = ocr.prob ?? 0.2;
  els.beamSize.value = ocr.beam_size ?? 5;
  els.filterText.value = Array.isArray(ocr.filter_text) ? ocr.filter_text.join(", ") : "";
  els.maskMethod.value = maskRefinement.method || "FitText";
  els.ignoreBubble.value = maskRefinement.ignore_bubble ?? 0;
  els.dilationOffset.value = maskRefinement.dilation_offset ?? 20;
  els.kernelSize.value = maskRefinement.kernel_size ?? 3;
  els.furigana.checked = Boolean(maskRefinement.furigana);
  els.inpainter.value = inpainter.inpainter || "LamaAot";
  els.inpaintingSize.value = inpainter.inpainting_size ?? 2048;
  els.maskSource.value = inpainter.mask || "RefinedMask";
  els.inpaintColor.value = rgbToHex(inpainter.inpaint_color || [255, 255, 255]);
  els.upscaler.value = upscalerValueForControl(upscaler.upscaler);
  els.upscalePatch.value = upscaler.patch_size ?? "";
  els.upscalePadding.value = upscaler.padding ?? 0;
  els.provider.value = openai.provider_preset || "Custom";
  els.baseUrl.value = openai.base_url || "";
  els.apiKey.value = openai.api_key || "";
  els.modelName.value = openai.model || "";
  els.timeoutSecs.value = openai.timeout_secs ?? 60;
  els.temperature.value = openai.temperature ?? "";
  els.topP.value = openai.top_p ?? "";
  els.systemPrompt.value = openai.system_prompt || "";
  els.userPrompt.value = openai.user_prompt_template || "";
  refreshGuidance();
}

function patchSettingsFromControls() {
  const cfg = JSON.parse(els.settingsJson.value || "{}");
  cfg.translator = cfg.translator || {};
  cfg.translator.target = cfg.translator.target || {};
  cfg.translator.target.translator = els.translator.value;
  cfg.translator.target.target = els.targetLang.value;
  cfg.translator.filter_lang = parseCsvList(els.filterLang.value);
  cfg.translator.pre_dict = optionalString(els.preDict.value);
  cfg.translator.post_dict = optionalString(els.postDict.value);
  cfg.translator.openai_compatible = cfg.translator.openai_compatible || {};
  cfg.translator.openai_compatible.provider_preset = els.provider.value;
  cfg.translator.openai_compatible.base_url = els.baseUrl.value.trim();
  cfg.translator.openai_compatible.api_key = els.apiKey.value.trim();
  cfg.translator.openai_compatible.model = els.modelName.value.trim();
  cfg.translator.openai_compatible.system_prompt = els.systemPrompt.value;
  cfg.translator.openai_compatible.user_prompt_template = els.userPrompt.value;
  cfg.translator.openai_compatible.temperature = parseOptionalNumber(els.temperature.value);
  cfg.translator.openai_compatible.top_p = parseOptionalNumber(els.topP.value);
  cfg.translator.openai_compatible.timeout_secs = parseNumberOrDefault(els.timeoutSecs.value, 60);
  cfg.detector = cfg.detector || {};
  cfg.detector.detector = els.detector.value;
  cfg.detector.options = cfg.detector.options || {};
  cfg.detector.options.detect_size = parseIntegerOrDefault(els.detectSize.value, 2048);
  cfg.detector.options.unclip_ratio = parseNumberOrDefault(els.unclipRatio.value, 2.3);
  cfg.detector.options.text_threshold = parseNumberOrDefault(els.textThreshold.value, 0.5);
  cfg.detector.options.box_threshold = parseNumberOrDefault(els.boxThreshold.value, 0.7);
  cfg.detector.preprocessor = cfg.detector.preprocessor || {};
  cfg.detector.preprocessor.invert = els.preInvert.checked;
  cfg.detector.preprocessor.gamma_correct = els.preGamma.checked;
  cfg.detector.preprocessor.rotate = els.preRotate.checked;
  cfg.detector.preprocessor.auto_rotate = els.preAutoRotate.checked;
  cfg.ocr = cfg.ocr || {};
  cfg.ocr.ocr = els.ocrModel.value;
  cfg.ocr.min_text_length = parseIntegerOrDefault(els.minTextLength.value, 1);
  cfg.ocr.prob = parseNumberOrDefault(els.ocrProb.value, 0.2);
  cfg.ocr.beam_size = parseIntegerOrDefault(els.beamSize.value, 5);
  cfg.ocr.filter_text = parseCsvList(els.filterText.value);
  cfg.mask_refinement = cfg.mask_refinement || {};
  cfg.mask_refinement.method = els.maskMethod.value;
  cfg.mask_refinement.ignore_bubble = parseIntegerOrDefault(els.ignoreBubble.value, 0);
  cfg.mask_refinement.dilation_offset = parseNumberOrDefault(els.dilationOffset.value, 20);
  cfg.mask_refinement.kernel_size = parseIntegerOrDefault(els.kernelSize.value, 3);
  cfg.mask_refinement.furigana = els.furigana.checked;
  cfg.inpainter = cfg.inpainter || {};
  cfg.inpainter.inpainter = els.inpainter.value;
  cfg.inpainter.inpainting_size = parseIntegerOrDefault(els.inpaintingSize.value, 2048);
  cfg.inpainter.mask = els.maskSource.value;
  cfg.inpainter.inpaint_color = hexToRgb(els.inpaintColor.value);
  cfg.upscaler = cfg.upscaler || {};
  if (els.upscaler.value !== "custom") {
    cfg.upscaler.upscaler = els.upscaler.value === "none" ? null : els.upscaler.value;
  }
  cfg.upscaler.patch_size = parseOptionalInteger(els.upscalePatch.value);
  cfg.upscaler.padding = parseIntegerOrDefault(els.upscalePadding.value, 0);
  cfg.render = cfg.render || {};
  cfg.render.text_direction = toPascalCase(els.textDirection.value || "auto");
  els.settingsJson.value = JSON.stringify(cfg, null, 2);
  state.settings = cfg;
  return cfg;
}

function toPascalCase(value) {
  const normalized = String(value || "auto").toLowerCase();
  return normalized.charAt(0).toUpperCase() + normalized.slice(1);
}

function setSelectValue(select, value) {
  const hasOption = Array.from(select.options).some((option) => option.value === value);
  select.value = hasOption ? value : select.options[0]?.value || "";
}

function normalizeLanguageCode(value) {
  const normalized = String(value || "").trim().toLowerCase().replace("_", "-");
  const aliases = {
    "zh-cn": "chs",
    "zh-hans": "chs",
    "zh-chs": "chs",
    chinese: "chs",
    "zh-tw": "cht",
    "zh-hk": "cht",
    "zh-hant": "cht",
    "zh-cht": "cht",
    english: "en",
    japanese: "ja",
    korean: "ko",
    french: "fr",
    german: "de",
    spanish: "es",
    russian: "ru",
  };
  return aliases[normalized] || normalized || "en";
}

function parseOptionalNumber(value) {
  const trimmed = String(value ?? "").trim();
  if (!trimmed) return null;
  const parsed = Number(trimmed);
  return Number.isFinite(parsed) ? parsed : null;
}

function parseOptionalInteger(value) {
  const trimmed = String(value ?? "").trim();
  if (!trimmed) return null;
  const parsed = Number.parseInt(trimmed, 10);
  return Number.isFinite(parsed) ? parsed : null;
}

function parseNumberOrDefault(value, fallback) {
  const parsed = parseOptionalNumber(value);
  return parsed === null ? fallback : parsed;
}

function parseIntegerOrDefault(value, fallback) {
  const parsed = parseOptionalInteger(value);
  return parsed === null ? fallback : parsed;
}

function optionalString(value) {
  const trimmed = String(value ?? "").trim();
  return trimmed ? trimmed : null;
}

function parseCsvList(value) {
  return String(value ?? "")
    .split(/[\n,]+/)
    .map((item) => item.trim())
    .filter(Boolean);
}

function rgbToHex(value) {
  const [r, g, b] = Array.isArray(value) ? value : [255, 255, 255];
  return `#${[r, g, b]
    .map((item) => Math.max(0, Math.min(255, Number(item) || 0)).toString(16).padStart(2, "0"))
    .join("")}`;
}

function hexToRgb(value) {
  const match = /^#?([0-9a-f]{6})$/i.exec(String(value || ""));
  if (!match) return [255, 255, 255];
  const raw = match[1];
  return [0, 2, 4].map((offset) => Number.parseInt(raw.slice(offset, offset + 2), 16));
}

function upscalerValueForControl(value) {
  if (value === null || value === undefined) return "none";
  if (typeof value === "string") return value;
  return "custom";
}

function applyProviderPreset() {
  const baseUrl = providerBaseUrls[els.provider.value];
  if (baseUrl) {
    els.baseUrl.value = baseUrl;
  }
  if (els.provider.value !== "Custom") {
    els.translator.value = "OpenAICompatible";
  }
  patchSettingsFromControls();
  refreshGuidance();
}

function statusLabel(status) {
  if (status === "done") return t("statusDone");
  if (status === "failed") return t("statusFailed");
  if (status === "skipped") return t("statusSkipped");
  if (status === "partial") return t("statusPartial");
  return status || "-";
}

function countResults(outputs) {
  const done = outputs.filter((item) => item.status === "done").length;
  const failed = outputs.filter((item) => item.status === "failed").length;
  const skipped = outputs.filter((item) => item.status === "skipped").length;
  return { done, failed, skipped };
}

async function startTranslation() {
  let settings;
  try {
    settings = patchSettingsFromControls();
  } catch (err) {
    setStatus(t("jsonError"), err.message);
    addLog("error", `${t("jsonError")}: ${err.message}`);
    return;
  }

  const inputs = [...state.checked.values()];
  if (!inputs.length) {
    setStatus(t("noCheckedSelection"), "");
    addLog("error", t("noCheckedSelection"));
    return;
  }
  try {
    setRunningState(true);
    updateProgress({ current: 0, total: inputs.length || 1, message: t("progressPreparing") });
    setStatus(t("starting"), t("backendPending"));
    const result = await invoke("startTranslation", {
      input_paths: inputs,
      settings,
      output_format: els.outputFormat.value,
      device_mode: getDeviceMode(),
      debug: els.debugMode.checked,
      max_parallel_images: parseIntegerOrDefault(els.maxParallelImages.value, 2),
      max_parallel_gpu_jobs: parseIntegerOrDefault(els.maxParallelGpuJobs.value, 1),
    });
    renderResult(result);
    updateProgress({ current: result.outputs?.length || 0, total: result.outputs?.length || 1, message: t("progressDone"), percent: 100 });
    setStatus(statusLabel(result.status || "done"), summarizeText(result.message || t("progressDone"), 120));
    addLog(result.status === "partial" ? "warn" : "success", result.message || t("progressDone"));
  } catch (err) {
    setStatus(t("backendPending"), summarizeText(err.message, 120));
    addLog("error", err.message);
  } finally {
    setRunningState(false);
  }
}

// The canvas is the preview area: it shows whatever is active (a tree file or a
// completed result), or a hint when nothing is selected. Results live in the
// left panel now (P3d), so the canvas is dedicated to the pan/zoom viewer.
function renderCanvas() {
  const stage = els.canvasStage;
  // Switching preview away from the image being edited drops edit mode.
  if (state.editMode && (!state.editData || state.editData.path !== state.preview)) {
    exitEditMode({ silent: true });
  }
  closeTextEditor();
  stage.innerHTML = "";
  state.canvasView = null;
  if (!state.preview) {
    stage.className = "canvas-stage";
    const empty = document.createElement("div");
    empty.className = "empty-state";
    empty.textContent = t("previewHint");
    stage.append(empty);
    updateEditBar();
    return;
  }
  stage.className = "canvas-stage has-preview";
  const wrap = document.createElement("div");
  wrap.className = "canvas-preview";
  const viewport = document.createElement("div");
  viewport.className = "canvas-viewport";
  // The transform target wraps the image and (in edit mode) the SVG overlay so
  // both pan/zoom together, keeping overlay boxes pinned to image pixels.
  const content = document.createElement("div");
  content.className = "canvas-content";
  const img = document.createElement("img");
  img.className = "canvas-img";
  img.alt = "";
  img.draggable = false;
  content.append(img);

  let overlay = null;
  if (state.editMode && state.editData) {
    // HTML overlay layer in image-pixel space; it scales with `content`, so the
    // text boxes (and their font-size in px) track pan/zoom automatically.
    overlay = document.createElement("div");
    overlay.className = "canvas-overlay";
    overlay.style.width = `${state.editData.width}px`;
    overlay.style.height = `${state.editData.height}px`;
    content.append(overlay);
  }
  viewport.append(content);
  wrap.append(viewport);
  stage.append(wrap);
  state.canvasView = setupImageViewer(viewport, content, img);
  // The canvas always shows the exact rendered PNG (the renderer does anti-overlap
  // repositioning + font auto-fit that HTML can't replicate, so we never fake the
  // text). In edit mode the overlay adds transparent selectable/draggable boxes;
  // editing a box re-renders and refreshes this image with the exact result.
  loadLocalImage(img, state.preview);
  if (overlay) renderOverlay(overlay);
  updateEditBar();
}

// Lightweight pan/zoom for the preview canvas (no library). The image starts
// fit-to-view (aspect preserved, fully visible); wheel zooms toward the cursor,
// drag pans, double-click re-fits. Translate/scale via a single CSS transform on
// the content wrapper. Returns the live `view` so edit code can map screen<->image.
function setupImageViewer(viewport, content, img) {
  const view = { scale: 1, base: 1, tx: 0, ty: 0, natW: 0, natH: 0, loaded: false, onchange: null };

  const apply = () => {
    content.style.transform = `translate(${view.tx}px, ${view.ty}px) scale(${view.scale})`;
    if (view.onchange) view.onchange();
  };
  view.apply = apply;
  const fit = () => {
    const vw = viewport.clientWidth;
    const vh = viewport.clientHeight;
    view.natW = img.naturalWidth || vw;
    view.natH = img.naturalHeight || vh;
    view.base = Math.min(vw / view.natW, vh / view.natH) || 1;
    view.scale = view.base;
    view.tx = (vw - view.natW * view.scale) / 2;
    view.ty = (vh - view.natH * view.scale) / 2;
    apply();
  };
  view.fit = fit;

  // Fit on first load; on later src swaps (re-render after an edit) keep the
  // current pan/zoom instead of snapping back to fit.
  img.addEventListener("load", () => {
    if (!view.loaded) {
      view.loaded = true;
      fit();
    } else {
      apply();
    }
  });

  viewport.addEventListener(
    "wheel",
    (event) => {
      event.preventDefault();
      if (!view.natW) return;
      const rect = viewport.getBoundingClientRect();
      const cx = event.clientX - rect.left;
      const cy = event.clientY - rect.top;
      const factor = event.deltaY < 0 ? 1.12 : 1 / 1.12;
      const next = Math.max(view.base, Math.min(view.base * 12, view.scale * factor));
      if (next === view.scale) return;
      // Keep the image point under the cursor fixed while zooming.
      view.tx = cx - (cx - view.tx) * (next / view.scale);
      view.ty = cy - (cy - view.ty) * (next / view.scale);
      view.scale = next;
      apply();
    },
    { passive: false },
  );

  let dragging = false;
  let startX = 0;
  let startY = 0;
  let baseTx = 0;
  let baseTy = 0;
  viewport.addEventListener("pointerdown", (event) => {
    dragging = true;
    startX = event.clientX;
    startY = event.clientY;
    baseTx = view.tx;
    baseTy = view.ty;
    viewport.setPointerCapture(event.pointerId);
    viewport.classList.add("is-grabbing");
  });
  viewport.addEventListener("pointermove", (event) => {
    if (!dragging) return;
    view.tx = baseTx + (event.clientX - startX);
    view.ty = baseTy + (event.clientY - startY);
    apply();
  });
  const endDrag = (event) => {
    if (!dragging) return;
    dragging = false;
    try {
      viewport.releasePointerCapture(event.pointerId);
    } catch (_) {}
    viewport.classList.remove("is-grabbing");
  };
  viewport.addEventListener("pointerup", endDrag);
  viewport.addEventListener("pointercancel", endDrag);
  viewport.addEventListener("dblclick", fit);

  // Re-fit on container resize only while still at the fit scale, so resizing
  // the window doesn't fight a zoom the user set.
  if (window.ResizeObserver) {
    new ResizeObserver(() => {
      if (Math.abs(view.scale - view.base) < 1e-3) fit();
    }).observe(viewport);
  }
  return view;
}

// ── P5 editable typeset overlay ──────────────────────────────────────────────
// The result PNG stays as the canvas background (exact, baked text); the SVG
// overlay draws one selectable/draggable box per text block. Editing text or
// dragging a box commits to the backend (RerenderExport, renderer-only, no
// models), which overwrites the PNG and returns a fresh data URL we swap in.

function regionByIndex(index) {
  return state.editData ? state.editData.regions[index] : null;
}

// The result item currently previewed, if it carries an editable sidecar.
function currentEditableResult() {
  if (!state.preview) return null;
  return (
    state.results.find(
      (it) => it.status === "done" && it.output === state.preview && it.editable,
    ) || null
  );
}

function updateEditBar() {
  const bar = els.canvasEditBar;
  if (!bar) return;
  const editable = currentEditableResult();
  bar.hidden = !editable && !state.editMode;
  if (els.toggleEdit) els.toggleEdit.hidden = state.editMode || !editable;
  if (els.exitEdit) els.exitEdit.hidden = !state.editMode;
  if (els.editHint) els.editHint.hidden = !state.editMode;
}

async function enterEditMode() {
  const item = currentEditableResult();
  if (!item) return;
  try {
    const data = await invoke("loadEditable", { path: item.output });
    state.editData = {
      path: item.output,
      width: data.width,
      height: data.height,
      // Text-free inpainted background; used as the edit textarea's backdrop so it
      // shows clean manga (no old baked text) behind what the user types.
      background: data.background || "",
      regions: Array.isArray(data.regions) ? data.regions : [],
    };
    state.editMode = true;
    state.editSelected = null;
    renderCanvas();
    addLog("info", `${t("editEntered")}（${state.editData.regions.length} 框）`);
  } catch (err) {
    addLog("error", `${t("editFailed")}: ${err.message}`);
  }
}

function exitEditMode({ silent } = {}) {
  closeTextEditor();
  const wasEditing = state.editMode;
  state.editMode = false;
  state.editData = null;
  state.editSelected = null;
  if (wasEditing && !silent) renderCanvas();
}

// Build the overlay: one transparent box per region (in image-pixel space inside
// `content`, so it pans/zooms with the canvas). The boxes are just selection/drag
// handles over the exact baked image — no faked text.
function renderOverlay(layer) {
  if (!state.editData) return;
  layer.innerHTML = "";
  for (const region of state.editData.regions) {
    const box = document.createElement("div");
    box.className = "overlay-box";
    box.dataset.index = region.index;
    box.style.left = `${region.x}px`;
    box.style.top = `${region.y}px`;
    box.style.width = `${Math.max(1, region.w)}px`;
    box.style.height = `${Math.max(1, region.h)}px`;
    if (state.editSelected === region.index) box.classList.add("is-selected");
    layer.append(box);
  }
  setupOverlayInteractions(layer);
}

function selectRegion(index) {
  state.editSelected = index;
  const layer = els.canvasStage.querySelector(".canvas-overlay");
  if (!layer) return;
  layer.querySelectorAll(".overlay-box").forEach((b) => {
    b.classList.toggle("is-selected", Number(b.dataset.index) === index);
  });
}

function overlayBox(index) {
  const layer = els.canvasStage.querySelector(".canvas-overlay");
  return layer ? layer.querySelector(`.overlay-box[data-index="${index}"]`) : null;
}

function setupOverlayInteractions(layer) {
  let drag = null;
  layer.addEventListener("pointerdown", (event) => {
    const box = event.target.closest(".overlay-box");
    if (!box) return; // empty area falls through to viewport pan
    const index = Number(box.dataset.index);
    // While this box is being edited, let clicks place the caret (no drag).
    if (state.editEditor && state.editEditor.index === index) return;
    event.stopPropagation();
    selectRegion(index);
    const region = regionByIndex(index);
    if (!region) return;
    drag = { index, box, startX: event.clientX, startY: event.clientY, ox: region.x, oy: region.y, moved: false };
    try {
      box.setPointerCapture(event.pointerId);
    } catch (_) {}
  });
  layer.addEventListener("pointermove", (event) => {
    if (!drag) return;
    const view = state.canvasView;
    if (!view || !view.scale) return;
    const dx = (event.clientX - drag.startX) / view.scale;
    const dy = (event.clientY - drag.startY) / view.scale;
    if (Math.abs(dx) > 2 || Math.abs(dy) > 2) drag.moved = true;
    drag.box.style.left = `${drag.ox + dx}px`;
    drag.box.style.top = `${drag.oy + dy}px`;
  });
  const endDrag = (event) => {
    if (!drag) return;
    const view = state.canvasView;
    const scale = view && view.scale ? view.scale : 1;
    const dx = Math.round((event.clientX - drag.startX) / scale);
    const dy = Math.round((event.clientY - drag.startY) / scale);
    const d = drag;
    drag = null;
    try {
      d.box.releasePointerCapture(event.pointerId);
    } catch (_) {}
    const region = regionByIndex(d.index);
    if (d.moved && (dx !== 0 || dy !== 0)) {
      if (region) {
        region.x += dx;
        region.y += dy;
      }
      applyEdit([{ index: d.index, dx, dy }]);
    } else if (region) {
      // Negligible drag: snap back.
      d.box.style.left = `${region.x}px`;
      d.box.style.top = `${region.y}px`;
    }
  };
  layer.addEventListener("pointerup", endDrag);
  layer.addEventListener("pointercancel", endDrag);
  layer.addEventListener("dblclick", (event) => {
    const box = event.target.closest(".overlay-box");
    if (!box) return;
    event.stopPropagation();
    openTextEditor(Number(box.dataset.index));
  });
}

// Edit a region's text: a textarea floats over the box in screen space, styled
// like the bubble (region bg color masks the baked text underneath, matching
// fg/size) so it reads as in-place editing. On commit the backend re-renders and
// the canvas refreshes with the exact result. Enter commits, Esc cancels.
function openTextEditor(index) {
  closeTextEditor();
  const region = regionByIndex(index);
  const viewport = els.canvasStage.querySelector(".canvas-viewport");
  const view = state.canvasView;
  if (!region || !viewport || !view) return;
  selectRegion(index);
  const box = overlayBox(index);
  if (box) box.classList.add("is-editing");

  const ta = document.createElement("textarea");
  ta.className = "overlay-editor";
  ta.value = region.text || "";
  ta.spellcheck = false;
  const fs =
    region.fontSize && region.fontSize > 0
      ? region.fontSize
      : Math.max(12, Math.round(region.h * 0.6));
  if (Array.isArray(region.fg)) {
    ta.style.color = `rgb(${region.fg[0]}, ${region.fg[1]}, ${region.fg[2]})`;
  }
  const bgUrl = state.editData.background;
  // Show the text-free manga behind the editor (not a solid box): use the clean
  // background scaled to the current view, offset so this region's patch aligns.
  const place = () => {
    ta.style.left = `${view.tx + region.x * view.scale}px`;
    ta.style.top = `${view.ty + region.y * view.scale}px`;
    ta.style.width = `${Math.max(60, region.w * view.scale)}px`;
    ta.style.minHeight = `${Math.max(24, region.h * view.scale)}px`;
    ta.style.fontSize = `${Math.max(11, fs * view.scale)}px`;
    if (bgUrl) {
      ta.style.backgroundImage = `url("${bgUrl}")`;
      ta.style.backgroundSize = `${state.editData.width * view.scale}px ${state.editData.height * view.scale}px`;
      ta.style.backgroundPosition = `${-region.x * view.scale}px ${-region.y * view.scale}px`;
    }
  };
  place();
  view.onchange = place;
  viewport.append(ta);
  state.editEditor = { index, ta, box, original: region.text || "" };
  ta.focus();
  ta.select();
  ta.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      event.preventDefault();
      cancelTextEditor();
    } else if (event.key === "Enter" && !event.shiftKey) {
      event.preventDefault();
      commitTextEditor();
    }
  });
  ta.addEventListener("blur", () => commitTextEditor());
}

function readEditorText(el) {
  // innerText preserves line breaks as \n; normalize nbsp back to spaces.
  return (el.innerText || "").replace(/ /g, " ");
}

function finishEditor(ed) {
  if (state.canvasView) state.canvasView.onchange = null;
  if (ed.box) ed.box.classList.remove("is-editing");
  if (ed.ta && ed.ta.parentNode) ed.ta.parentNode.removeChild(ed.ta);
  state.editEditor = null;
}

function commitTextEditor() {
  const ed = state.editEditor;
  if (!ed) return;
  const text = ed.ta.value;
  const index = ed.index;
  finishEditor(ed);
  const region = regionByIndex(index);
  if (region && text !== ed.original) {
    region.text = text;
    applyEdit([{ index, text }]);
  }
}

function cancelTextEditor() {
  if (state.editEditor) finishEditor(state.editEditor);
}

function closeTextEditor() {
  if (state.editEditor) commitTextEditor();
}

// Persist an edit (renderer-only, no models): overwrites the result PNG + sidecar,
// then refresh the canvas with the exact re-rendered image (zoom/pan preserved).
async function applyEdit(edits) {
  if (!state.editData) return;
  try {
    const res = await invoke("rerenderExport", { path: state.editData.path, edits });
    if (res && res.data_url) {
      const img = els.canvasStage.querySelector("img.canvas-img");
      if (img) img.src = res.data_url;
    }
    addLog("success", t("editApplied"));
  } catch (err) {
    addLog("error", `${t("editFailed")}: ${err.message}`);
  }
}

function renderResult(result) {
  const outputs = Array.isArray(result.outputs) ? result.outputs : [];
  state.results = outputs;
  state.selectedResults = new Set(
    outputs
      .filter((item) => item.status === "done" && item.output)
      .map((item) => item.output),
  );
  // Show the first finished page on the canvas right away.
  const firstDone = outputs.find((item) => item.status === "done" && item.output);
  if (firstDone) {
    state.preview = firstDone.output;
    state.activePath = "";
  }
  renderTree();
  renderResults();
  renderCanvas();
}

// Render the "completed translations" list in the left panel (P3d). Rows are
// compact (no thumbnail — the canvas is the preview); a checkbox drives export
// selection, clicking the row previews the output on the canvas.
function renderResults() {
  const has = state.results.length > 0;
  els.filmstripResults.hidden = !has;
  if (!has) {
    els.resultStats.textContent = "";
    els.results.innerHTML = "";
    return;
  }

  els.resultStats.textContent = t("resultStats", countResults(state.results));
  els.results.className = "result-list";
  els.results.innerHTML = state.results.map((item, index) => resultRow(item, index)).join("");

  const doneOutputs = state.results.filter((item) => item.status === "done" && item.output);
  const allSelected =
    doneOutputs.length > 0 && doneOutputs.every((item) => state.selectedResults.has(item.output));
  els.selectAllResults.textContent = allSelected ? t("deselectAllResults") : t("selectAllResults");
}

function resultRow(item, index) {
  const output = item.output || "";
  const status = item.status || "";
  const canUse = status === "done" && output;
  const checked = canUse && state.selectedResults.has(output) ? "checked" : "";
  const isActive = output && output === state.preview;
  return `
    <div class="result-row${isActive ? " is-active" : ""}${canUse ? " is-file" : ""}" data-result-index="${index}" title="${escapeAttr(output || item.input || item.message || "")}">
      <input type="checkbox" class="result-check-box" data-result-index="${index}" ${checked} ${canUse ? "" : "disabled"}>
      <span class="tree-icon">${ICON_FILE}</span>
      <span class="result-name">${escapeHtml(item.file_name || statusLabel(status))}</span>
      <span class="status-badge" data-status="${escapeHtml(status)}">${escapeHtml(statusLabel(status))}</span>
    </div>
  `;
}

// Click a completed result → preview its output image on the canvas.
function previewResultOutput(index) {
  const item = state.results[index];
  if (!item || item.status !== "done" || !item.output) return;
  state.preview = item.output;
  state.activePath = "";
  renderTree();
  renderResults();
  renderCanvas();
}

// WebView2 won't load file:// or custom-scheme images as subresources from a
// mit:// page, so local images come back through the ReadImage IPC as a data:
// URL and get dropped into the given <img>.
async function loadLocalImage(img, path) {
  try {
    const data = await invoke("readImage", { path });
    if (data && data.data_url) img.src = data.data_url;
  } catch (err) {
    addLog("error", `${pathBaseName(path)}: ${err.message}`);
  }
}

async function previewResult(index) {
  const item = state.results[index];
  if (!item?.output) return;
  try {
    await invoke("previewResult", { path: item.output });
  } catch (err) {
    addLog("error", err.message);
  }
}

function toggleResult(index, checked) {
  const item = state.results[index];
  if (!item?.output) return;
  if (checked) {
    state.selectedResults.add(item.output);
  } else {
    state.selectedResults.delete(item.output);
  }
  renderResults();
}

function toggleAllResults() {
  const doneOutputs = state.results.filter((item) => item.status === "done" && item.output);
  const allSelected =
    doneOutputs.length > 0 && doneOutputs.every((item) => state.selectedResults.has(item.output));
  if (allSelected) {
    doneOutputs.forEach((item) => state.selectedResults.delete(item.output));
  } else {
    doneOutputs.forEach((item) => state.selectedResults.add(item.output));
  }
  renderResults();
}

async function exportSelectedResults() {
  const outputs = [...state.selectedResults];
  if (!outputs.length) {
    setStatus(t("exportNeedSelection"), "");
    addLog("error", t("exportNeedSelection"));
    return;
  }
  // Translation stays in a temp dir; "Export selected" saves the chosen results
  // to the persisted export directory — picking + remembering one if unset.
  let exportDir = state.outputDir;
  if (!exportDir) {
    try {
      const picked = await invoke("pickOutputDir");
      exportDir = (picked.paths || [])[0] || "";
    } catch (err) {
      addLog("error", err.message);
      return;
    }
    if (!exportDir) return;
    applyOutputDir(exportDir);
  }
  try {
    const data = await invoke("exportResults", {
      outputs,
      export_dir: exportDir,
    });
    const count = Array.isArray(data.exported) ? data.exported.length : 0;
    setStatus(t("exported"), `${count} ${t("selected")}`);
    addLog("success", `${t("exported")}: ${count}`);
  } catch (err) {
    setStatus(t("backendPending"), summarizeText(err.message, 120));
    addLog("error", err.message);
  }
}

function previewHeightMax() {
  return Math.max(PREVIEW_HEIGHT_MIN, window.innerHeight - 260);
}

// Apply the preview-region height (top of the center column) via a CSS var,
// clamped so the terminal below keeps usable space. Returns the clamped value.
function applyPreviewHeight(px) {
  const clamped = Math.max(PREVIEW_HEIGHT_MIN, Math.min(previewHeightMax(), Math.round(px)));
  document.documentElement.style.setProperty("--preview-height", `${clamped}px`);
  return clamped;
}

// Collapse/expand the preview region; the terminal takes the freed space.
function setPreviewCollapsed(collapsed) {
  els.canvas.classList.toggle("preview-collapsed", collapsed);
  els.togglePreview.textContent = collapsed ? t("showPreview") : t("hidePreview");
  els.togglePreview.setAttribute("aria-pressed", String(collapsed));
  localStorage.setItem(PREVIEW_COLLAPSE_KEY, collapsed ? "1" : "0");
}

// Drag the divider under the preview to resize it; persist the height.
function initPreviewResizer() {
  const handle = els.previewResizer;
  const stage = els.canvasStage;
  if (!handle || !stage || !els.canvas) return;

  const saved = Number(localStorage.getItem(PREVIEW_HEIGHT_KEY));
  applyPreviewHeight(Number.isFinite(saved) && saved > 0 ? saved : PREVIEW_HEIGHT_DEFAULT);
  if (localStorage.getItem(PREVIEW_COLLAPSE_KEY) === "1") setPreviewCollapsed(true);

  let startY = 0;
  let startH = 0;
  let lastH = 0;
  let dragging = false;

  const onMove = (event) => {
    if (!dragging) return;
    lastH = applyPreviewHeight(startH + (event.clientY - startY));
  };
  const onUp = () => {
    if (!dragging) return;
    dragging = false;
    handle.classList.remove("is-dragging");
    localStorage.setItem(PREVIEW_HEIGHT_KEY, String(lastH));
    window.removeEventListener("pointermove", onMove);
    window.removeEventListener("pointerup", onUp);
  };

  handle.addEventListener("pointerdown", (event) => {
    if (els.canvas.classList.contains("preview-collapsed")) return;
    dragging = true;
    startY = event.clientY;
    startH = stage.offsetHeight;
    lastH = startH;
    handle.classList.add("is-dragging");
    window.addEventListener("pointermove", onMove);
    window.addEventListener("pointerup", onUp);
    event.preventDefault();
  });

  handle.addEventListener("keydown", (event) => {
    if (event.key !== "ArrowUp" && event.key !== "ArrowDown") return;
    event.preventDefault();
    lastH = applyPreviewHeight(stage.offsetHeight + (event.key === "ArrowDown" ? 24 : -24));
    localStorage.setItem(PREVIEW_HEIGHT_KEY, String(lastH));
  });

  window.addEventListener("resize", () => {
    if (!els.canvas.classList.contains("preview-collapsed")) {
      applyPreviewHeight(stage.offsetHeight);
    }
  });
}

function applyFilmstripWidth(px) {
  const clamped = Math.max(FILMSTRIP_WIDTH_MIN, Math.min(FILMSTRIP_WIDTH_MAX, Math.round(px)));
  document.documentElement.style.setProperty("--filmstrip-width", `${clamped}px`);
  return clamped;
}

// Drag the filmstrip's right edge to resize the column; persist the width.
function initFilmstripResizer() {
  const handle = els.filmstripResizer;
  const filmstrip = handle?.parentElement;
  if (!handle || !filmstrip) return;

  const saved = Number(localStorage.getItem(FILMSTRIP_WIDTH_KEY));
  if (Number.isFinite(saved) && saved > 0) applyFilmstripWidth(saved);

  let startX = 0;
  let startW = 0;
  let lastW = 0;
  let dragging = false;

  const onMove = (event) => {
    if (dragging) lastW = applyFilmstripWidth(startW + (event.clientX - startX));
  };
  const onUp = () => {
    if (!dragging) return;
    dragging = false;
    handle.classList.remove("is-dragging");
    localStorage.setItem(FILMSTRIP_WIDTH_KEY, String(lastW));
    window.removeEventListener("pointermove", onMove);
    window.removeEventListener("pointerup", onUp);
  };

  handle.addEventListener("pointerdown", (event) => {
    dragging = true;
    startX = event.clientX;
    startW = filmstrip.offsetWidth;
    lastW = startW;
    handle.classList.add("is-dragging");
    window.addEventListener("pointermove", onMove);
    window.addEventListener("pointerup", onUp);
    event.preventDefault();
  });
  handle.addEventListener("keydown", (event) => {
    if (event.key !== "ArrowLeft" && event.key !== "ArrowRight") return;
    event.preventDefault();
    lastW = applyFilmstripWidth(filmstrip.offsetWidth + (event.key === "ArrowRight" ? 16 : -16));
    localStorage.setItem(FILMSTRIP_WIDTH_KEY, String(lastW));
  });
}

function applyResultsHeight(px) {
  // Bound so the input tree above keeps a usable minimum.
  const filmstrip = els.filmstripResults?.parentElement;
  const max = filmstrip ? Math.max(RESULTS_HEIGHT_MIN, filmstrip.clientHeight - 200) : 600;
  const clamped = Math.max(RESULTS_HEIGHT_MIN, Math.min(max, Math.round(px)));
  document.documentElement.style.setProperty("--results-height", `${clamped}px`);
  return clamped;
}

// Drag the top edge of the completed-translations section to resize its height.
function initResultsResizer() {
  const handle = els.resultsResizer;
  const panel = els.filmstripResults;
  if (!handle || !panel) return;

  const saved = Number(localStorage.getItem(RESULTS_HEIGHT_KEY));
  if (Number.isFinite(saved) && saved > 0) applyResultsHeight(saved);

  let startY = 0;
  let startH = 0;
  let lastH = 0;
  let dragging = false;

  const onMove = (event) => {
    if (dragging) lastH = applyResultsHeight(startH + (startY - event.clientY));
  };
  const onUp = () => {
    if (!dragging) return;
    dragging = false;
    handle.classList.remove("is-dragging");
    localStorage.setItem(RESULTS_HEIGHT_KEY, String(lastH));
    window.removeEventListener("pointermove", onMove);
    window.removeEventListener("pointerup", onUp);
  };

  handle.addEventListener("pointerdown", (event) => {
    dragging = true;
    startY = event.clientY;
    startH = panel.offsetHeight;
    lastH = startH;
    handle.classList.add("is-dragging");
    window.addEventListener("pointermove", onMove);
    window.addEventListener("pointerup", onUp);
    event.preventDefault();
  });
  handle.addEventListener("keydown", (event) => {
    if (event.key !== "ArrowUp" && event.key !== "ArrowDown") return;
    event.preventDefault();
    lastH = applyResultsHeight(panel.offsetHeight + (event.key === "ArrowUp" ? 24 : -24));
    localStorage.setItem(RESULTS_HEIGHT_KEY, String(lastH));
  });
}

// ── Model management modal (M1) ──
async function openModelsModal() {
  els.modelsModal.hidden = false;
  try {
    applyModelsConfig(await invoke("getModelsConfig"));
  } catch (err) {
    addLog("error", err.message);
  }
  refreshModelsStatus();
  refreshGpuRuntime();
}

// Pull the downloaded/missing table and render it. Called on modal open, after
// picking a dir, and after a download finishes.
async function refreshModelsStatus() {
  els.modelsStatus.innerHTML = `<p class="models-status-empty">${t("modelStatusLoading")}</p>`;
  try {
    renderModelsStatus(await invoke("getModelsStatus"));
  } catch (err) {
    els.modelsStatus.innerHTML = `<p class="models-status-empty">${t("modelStatusError", { err: err.message })}</p>`;
  }
}

const MODEL_KIND_LABELS = {
  detector: "modelKindDetector",
  ocr: "modelKindOcr",
  inpainter: "modelKindInpainter",
  upscaler: "modelKindUpscaler",
};

function renderModelsStatus(data) {
  const status = els.modelsStatus;
  status.innerHTML = "";

  const head = document.createElement("div");
  head.className = "models-status-head";
  const title = document.createElement("span");
  title.className = "models-status-title";
  title.textContent = t("modelsStatusTitle");
  head.appendChild(title);
  const actions = document.createElement("div");
  actions.className = "models-status-actions";
  const groups = (data && data.groups) || [];
  if (data && data.modelDirSet && groups.some((g) => !g.ready)) {
    const dlAll = document.createElement("button");
    dlAll.type = "button";
    dlAll.className = "secondary-button";
    dlAll.dataset.modelDownload = "*";
    dlAll.textContent = t("modelDownloadMissingAll");
    actions.appendChild(dlAll);
  }
  const refreshBtn = document.createElement("button");
  refreshBtn.type = "button";
  refreshBtn.className = "ghost-button";
  refreshBtn.dataset.modelRefresh = "1";
  refreshBtn.textContent = t("refresh");
  actions.appendChild(refreshBtn);
  head.appendChild(actions);
  status.appendChild(head);

  if (!data || !data.modelDirSet) {
    const empty = document.createElement("p");
    empty.className = "models-status-empty";
    empty.textContent = t("modelsDirNotSet");
    status.appendChild(empty);
    return;
  }

  let lastKind = null;
  groups.forEach((group) => {
    if (group.kind !== lastKind) {
      lastKind = group.kind;
      const header = document.createElement("div");
      header.className = "models-kind-header";
      header.textContent = t(MODEL_KIND_LABELS[group.kind] || group.kind);
      status.appendChild(header);
    }
    const files = group.files || [];
    const readyCount = files.filter((f) => f.ready).length;
    const row = document.createElement("div");
    row.className = "models-row";
    const dot = document.createElement("span");
    dot.className = `models-dot ${group.ready ? "ready" : "missing"}`;
    const label = document.createElement("span");
    label.className = "models-label";
    label.textContent = group.label;
    const meta = document.createElement("span");
    meta.className = "models-meta";
    meta.textContent = group.ready
      ? t("modelReady")
      : t("modelFilesCount", { ready: readyCount, total: files.length });
    row.append(dot, label, meta);
    if (!group.ready) {
      const btn = document.createElement("button");
      btn.type = "button";
      btn.className = "secondary-button models-download";
      btn.dataset.modelDownload = group.id;
      btn.textContent = t("modelDownload");
      row.appendChild(btn);
    }
    status.appendChild(row);
  });
}

// Download the given group ids ([] = all missing). Progress streams to the
// status bar via the shared progress/log events; the table refreshes on resolve.
async function downloadModelTargets(targets, button) {
  if (button) {
    button.disabled = true;
    button.textContent = t("modelDownloading");
  }
  try {
    renderModelsStatus(await invoke("downloadModels", { targets }));
  } catch (err) {
    addLog("error", err.message);
    if (button) {
      button.disabled = false;
      button.textContent = t("modelDownload");
    }
  }
}

// ── GPU acceleration (CUDA runtime) panel ──────────────────────────────
async function refreshGpuRuntime() {
  if (!els.gpuRuntime) return;
  try {
    renderGpuRuntime(await invoke("getGpuRuntimeStatus"));
  } catch (err) {
    els.gpuRuntime.innerHTML = "";
    const empty = document.createElement("p");
    empty.className = "models-status-empty";
    empty.textContent = t("modelStatusError", { err: err.message });
    els.gpuRuntime.appendChild(empty);
  }
}

// One status line: green dot = ok, amber = problem, muted = not-applicable.
function gpuLayerRow(label, ok, meta, neutral = false) {
  const row = document.createElement("div");
  row.className = "models-row";
  const dot = document.createElement("span");
  dot.className = `models-dot ${neutral ? "missing" : ok ? "ready" : "warn"}`;
  const name = document.createElement("span");
  name.className = "models-label";
  name.textContent = label;
  const value = document.createElement("span");
  value.className = "models-meta";
  value.textContent = meta;
  row.append(dot, name, value);
  return row;
}

// Render the three-layer probe (GPU → driver → DLLs → ONNX EP) plus the
// recommendation banner and, when DLLs are missing, a download button.
function renderGpuRuntime(status) {
  const el = els.gpuRuntime;
  if (!el) return;
  el.innerHTML = "";
  if (!status) return;
  state.gpuStatus = status;

  const head = document.createElement("div");
  head.className = "models-status-head";
  const title = document.createElement("span");
  title.className = "models-status-title";
  title.textContent = t("gpuSectionTitle");
  head.appendChild(title);
  const actions = document.createElement("div");
  actions.className = "models-status-actions";
  const refreshBtn = document.createElement("button");
  refreshBtn.type = "button";
  refreshBtn.className = "ghost-button";
  refreshBtn.dataset.gpuRefresh = "1";
  refreshBtn.textContent = t("gpuRefresh");
  actions.appendChild(refreshBtn);
  head.appendChild(actions);
  el.appendChild(head);

  el.appendChild(
    gpuLayerRow(
      t("gpuLayerDetect"),
      status.gpuDetected,
      status.gpuDetected ? status.gpuName || "—" : t("gpuNoNvidia"),
    ),
  );
  if (status.gpuDetected) {
    el.appendChild(
      gpuLayerRow(
        t("gpuLayerDriver"),
        status.driverOk,
        status.driverOk
          ? `${status.driverVersion || "—"} · ${t("gpuDriverOk")}`
          : `${status.driverVersion || "—"} · ${t("gpuDriverNeed", { min: status.minDriver })}`,
      ),
    );
  }
  const dlls = status.dlls || [];
  const present = dlls.filter((d) => d.present).length;
  el.appendChild(
    gpuLayerRow(
      t("gpuLayerDll"),
      status.dllAllPresent,
      status.dllAllPresent
        ? t("gpuDllReady")
        : t("gpuDllPartial", { ready: present, total: dlls.length }),
    ),
  );
  el.appendChild(
    gpuLayerRow(
      t("gpuLayerEp"),
      status.epOk,
      status.epOk ? t("gpuEpOk") : t("gpuEpNo"),
      !status.epOk && status.recommendation !== "ready",
    ),
  );

  const recKey =
    {
      cpu_only_build: "gpuRecCpuOnly",
      no_gpu: "gpuRecNoGpu",
      need_driver_update: "gpuRecDriver",
      need_download_dll: "gpuRecNeedDll",
      ready: "gpuRecReady",
    }[status.recommendation] || "gpuRecNoGpu";
  const tone =
    status.recommendation === "ready"
      ? "ok"
      : status.recommendation === "need_download_dll" ||
          status.recommendation === "need_driver_update"
        ? "warn"
        : "info";
  const rec = document.createElement("div");
  rec.className = `gpu-rec gpu-rec-${tone}`;
  rec.textContent = t(recKey, { min: status.minDriver });
  el.appendChild(rec);

  if (status.recommendation === "need_download_dll") {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "secondary-button gpu-download";
    btn.dataset.gpuDownload = "1";
    btn.textContent = t("gpuDownloadBtn");
    el.appendChild(btn);
  } else if (status.recommendation === "need_driver_update") {
    // Drivers can't be bundled or downloaded by us; point the user to NVIDIA's
    // official driver page instead of letting them fetch the ~0.9 GB runtime in
    // vain (an old driver can't load the CUDA 12 runtime even once present).
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "secondary-button gpu-driver-update";
    btn.dataset.gpuDriverUpdate = "1";
    btn.textContent = t("gpuDriverUpdateBtn");
    el.appendChild(btn);
  }

  updateDeviceBanner();
}

// Download streams progress to the shared status bar/log (same as model
// downloads); resolve returns a fresh status. If ready, offer a restart.
async function downloadCudaRuntimeFlow(button) {
  if (button) {
    button.disabled = true;
    button.textContent = t("gpuDownloading");
  }
  try {
    const status = await invoke("downloadCudaRuntime");
    renderGpuRuntime(status);
    if (status && status.recommendation === "ready") {
      promptGpuRestart();
    }
  } catch (err) {
    addLog("error", err.message);
    refreshGpuRuntime();
  }
}

// Freshly downloaded DLLs only load in a new process, so surface a one-click
// restart prompt; the user decides when to relaunch.
function promptGpuRestart() {
  const el = els.gpuRuntime;
  if (!el) return;
  const box = document.createElement("div");
  box.className = "gpu-restart";
  const text = document.createElement("p");
  text.className = "gpu-restart-text";
  text.textContent = t("gpuRestartReady");
  const row = document.createElement("div");
  row.className = "gpu-restart-actions";
  const now = document.createElement("button");
  now.type = "button";
  now.className = "primary-button";
  now.dataset.gpuRestart = "1";
  now.textContent = t("gpuRestartNow");
  const later = document.createElement("button");
  later.type = "button";
  later.className = "ghost-button";
  later.dataset.gpuRestartCancel = "1";
  later.textContent = t("gpuRestartLater");
  row.append(now, later);
  box.append(text, row);
  el.appendChild(box);
}

async function triggerRestart() {
  try {
    await invoke("restartApp");
  } catch (err) {
    addLog("error", err.message);
  }
}

// ── App self-update panel ──────────────────────────────────────────────
// All network/download work happens in the Rust backend via the
// checkAppUpdate / downloadAppUpdate / installAppUpdate IPCs; the frontend
// only reflects state. Download progress streams through the shared
// ProgressEvent into the status strip/log, same as model/GPU downloads.

const UPDATE_REC_TONE = {
  checking: "info",
  uptodate: "ok",
  available: "warn",
  error: "warn",
  downloading: "info",
  staged: "ok",
  installing: "info",
};

function formatBytes(bytes) {
  const n = Number(bytes);
  if (!Number.isFinite(n) || n <= 0) return "—";
  const units = ["B", "KB", "MB", "GB"];
  let value = n;
  let unit = 0;
  while (value >= 1024 && unit < units.length - 1) {
    value /= 1024;
    unit += 1;
  }
  const rounded = value >= 10 || unit === 0 ? Math.round(value) : value.toFixed(1);
  return `${rounded} ${units[unit]}`;
}

function formatUpdateDate(iso) {
  if (!iso) return "";
  const date = new Date(iso);
  return Number.isNaN(date.getTime()) ? "" : date.toLocaleDateString();
}

function updateBannerText() {
  const u = state.update;
  switch (u.status) {
    case "checking":
      return t("updateChecking");
    case "uptodate":
      return t("updateUpToDate");
    case "available":
      if (u.info && !u.info.assetName) return t("updateNoCompatibleAsset");
      return t("updateAvailable", {
        version: u.info?.latestVersion || u.info?.tagName || "",
      });
    case "error":
      return t("updateCheckFailed", { err: u.error || "" });
    case "downloading":
      return t("updateDownloading");
    case "staged":
      return t("updateStaged");
    case "installing":
      return t("updateInstalling");
    default:
      return "";
  }
}

function updateInfoRow(label, value) {
  const row = document.createElement("div");
  row.className = "app-update-row";
  const key = document.createElement("span");
  key.className = "app-update-key";
  key.textContent = label;
  const val = document.createElement("span");
  val.className = "app-update-value";
  val.textContent = value;
  row.append(key, val);
  return row;
}

function stagedMatchesInfo(staged, info) {
  if (!staged || !info) return false;
  const stagedTag = staged.tagName || "";
  const infoTag = info.tagName || "";
  const stagedVersion = staged.latestVersion || "";
  const infoVersion = info.latestVersion || "";
  return Boolean(
    (stagedTag && infoTag && stagedTag === infoTag) ||
      (stagedVersion && infoVersion && stagedVersion === infoVersion),
  );
}

function renderAppUpdate() {
  const el = els.appUpdate;
  if (!el) return;
  el.innerHTML = "";
  const u = state.update;
  const busy =
    u.status === "checking" || u.status === "downloading" || u.status === "installing";

  // Header: title + re-check button (always available as the entry point).
  const head = document.createElement("div");
  head.className = "models-status-head";
  const title = document.createElement("span");
  title.className = "models-status-title";
  title.textContent = t("updateTitle");
  head.appendChild(title);
  const actions = document.createElement("div");
  actions.className = "models-status-actions";
  const checkBtn = document.createElement("button");
  checkBtn.type = "button";
  checkBtn.className = "ghost-button";
  checkBtn.dataset.updateCheck = "1";
  checkBtn.disabled = busy;
  checkBtn.textContent = u.status === "checking" ? t("updateChecking") : t("updateCheck");
  actions.appendChild(checkBtn);
  head.appendChild(actions);
  el.appendChild(head);

  // Version readout.
  const currentVersion = u.info?.currentVersion || state.appVersion || "—";
  el.appendChild(updateInfoRow(t("updateCurrentVersion"), currentVersion));

  const showLatest =
    u.info &&
    (u.status === "available" ||
      u.status === "downloading" ||
      u.status === "staged" ||
      u.status === "installing" ||
      u.status === "uptodate");
  if (showLatest) {
    const latest = u.info.latestVersion || u.info.tagName || "—";
    const date = formatUpdateDate(u.info.publishedAt);
    el.appendChild(updateInfoRow(t("updateLatestVersion"), date ? `${latest} · ${date}` : latest));
    if (u.info.assetSize && u.status !== "uptodate") {
      el.appendChild(updateInfoRow(t("updateAssetSize"), formatBytes(u.info.assetSize)));
    }
  }

  // Status banner (reuses the GPU recommendation tones).
  const bannerText = updateBannerText();
  if (bannerText) {
    const rec = document.createElement("div");
    rec.className = `gpu-rec gpu-rec-${UPDATE_REC_TONE[u.status] || "info"}`;
    rec.textContent = bannerText;
    el.appendChild(rec);
  }

  // Action row + release notes, for any state where an update exists.
  const hasUpdate =
    u.info &&
    (u.status === "available" || u.status === "downloading" || u.status === "staged");
  if (hasUpdate) {
    const actionRow = document.createElement("div");
    actionRow.className = "app-update-actions";
    if (u.info.htmlUrl) {
      const link = document.createElement("button");
      link.type = "button";
      link.className = "link-button";
      link.dataset.updateNotesLink = u.info.htmlUrl;
      link.textContent = t("updateViewNotes");
      actionRow.appendChild(link);
    }
    if (u.status === "staged") {
      const install = document.createElement("button");
      install.type = "button";
      install.className = "secondary-button";
      install.dataset.updateInstall = "1";
      install.textContent = t("updateInstallBtn");
      actionRow.appendChild(install);
    } else if (u.info.assetName) {
      const download = document.createElement("button");
      download.type = "button";
      download.className = "secondary-button";
      download.dataset.updateDownload = "1";
      download.disabled = u.status === "downloading";
      download.textContent =
        u.status === "downloading" ? t("updateDownloading") : t("updateDownloadBtn");
      actionRow.appendChild(download);
    }
    el.appendChild(actionRow);

    const body = (u.info.body || "").trim();
    if (body) {
      const notes = document.createElement("div");
      notes.className = "app-update-notes";
      const notesHead = document.createElement("div");
      notesHead.className = "app-update-notes-head";
      const notesLabel = document.createElement("span");
      notesLabel.className = "app-update-notes-label";
      notesLabel.textContent = t("updateReleaseNotes");
      const toggle = document.createElement("button");
      toggle.type = "button";
      toggle.className = "link-button";
      toggle.dataset.updateNotesToggle = "1";
      toggle.textContent = u.notesExpanded ? t("collapse") : t("expand");
      notesHead.append(notesLabel, toggle);
      notes.appendChild(notesHead);
      if (u.notesExpanded) {
        const pre = document.createElement("pre");
        pre.className = "app-update-notes-body";
        pre.textContent = body;
        notes.appendChild(pre);
      } else {
        const preview = document.createElement("p");
        preview.className = "app-update-notes-preview";
        preview.textContent = summarizeText(body, 160);
        notes.appendChild(preview);
      }
      el.appendChild(notes);
    }
  }
}

async function checkUpdateFlow() {
  if (
    state.update.status === "checking" ||
    state.update.status === "downloading" ||
    state.update.status === "installing"
  ) {
    return;
  }
  state.update.status = "checking";
  state.update.error = "";
  renderAppUpdate();
  try {
    const info = await invoke("checkAppUpdate");
    state.update.info = info || null;
    if (info?.currentVersion) state.appVersion = info.currentVersion;
    if (info?.updateAvailable && stagedMatchesInfo(state.update.staged, info)) {
      state.update.status = "staged";
    } else {
      state.update.status = info?.updateAvailable ? "available" : "uptodate";
      if (!info?.updateAvailable) state.update.staged = null;
    }
    state.update.notesExpanded = false;
  } catch (err) {
    state.update.status = "error";
    state.update.error = summarizeText(err.message, 120);
    addLog("error", t("updateCheckFailed", { err: err.message }));
  }
  renderAppUpdate();
}

async function downloadUpdateFlow() {
  if (state.update.status === "downloading") return;
  state.update.status = "downloading";
  renderAppUpdate();
  try {
    const staged = await invoke("downloadAppUpdate");
    state.update.staged = staged || null;
    state.update.status = "staged";
    addLog("success", t("updateStaged"));
  } catch (err) {
    // Fall back to the "available" state so the user can retry.
    state.update.status = "available";
    addLog("error", t("updateDownloadFailed", { err: err.message }));
  }
  renderAppUpdate();
}

async function installUpdateFlow() {
  if (state.update.status === "installing") return;
  state.update.status = "installing";
  renderAppUpdate();
  try {
    // On success the backend launches the external updater and exits/relaunches
    // the app, so there is nothing more to do here — stay in the installing state.
    await invoke("installAppUpdate");
  } catch (err) {
    state.update.status = "staged";
    addLog("error", t("updateInstallFailed", { err: err.message }));
    renderAppUpdate();
  }
}

// Release link opens in the system browser via the backend openExternal IPC.
// That IPC is optional: if the backend doesn't implement it the request never
// resolves (unknown kinds are dropped, not rejected), so we time-box the call
// and degrade to copying the URL to the clipboard.
async function openReleaseNotes(url) {
  if (!url) return;
  try {
    await Promise.race([
      invoke("openExternal", { url }),
      new Promise((_, reject) =>
        setTimeout(() => reject(new Error("openExternal unavailable")), 1500),
      ),
    ]);
  } catch (_) {
    const ok = await copyText(url);
    if (ok) addLog("info", `${t("updateLinkCopied")}: ${url}`);
  }
}

function closeModelsModal() {
  els.modelsModal.hidden = true;
}

function applyModelsConfig(cfg) {
  els.modelDir.value = (cfg && cfg.model_dir) || "";
  els.autoDownload.checked = Boolean(cfg && cfg.auto_download);
}

async function pickModelDir() {
  try {
    const cfg = await invoke("setModelsDir");
    applyModelsConfig(cfg);
    if (cfg && cfg.model_dir) {
      addLog("success", `${t("modelDir")}: ${cfg.model_dir}`);
      refreshModelsStatus();
    }
  } catch (err) {
    addLog("error", err.message);
  }
}

async function setAutoDownload(value) {
  try {
    await invoke("setAutoDownload", { value });
  } catch (err) {
    addLog("error", err.message);
  }
}

// Smoothly animate the inspector accordions. Native <details> toggles instantly,
// which both looks abrupt and triggers a WebView2 repaint glitch (stale tiles)
// when several groups are open; animating the height forces continuous repaints.
function initAccordions() {
  const reduceMotion = () =>
    window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  document.querySelectorAll("details.acc").forEach((det) => {
    const summary = det.querySelector("summary.acc-head");
    const body = det.querySelector(".acc-body");
    if (!summary || !body) return;
    let anim = null;
    summary.addEventListener("click", (event) => {
      event.preventDefault();
      if (anim) {
        anim.cancel();
        anim = null;
      }
      if (reduceMotion()) {
        det.open = !det.open;
        return;
      }
      if (det.open) {
        const start = body.offsetHeight;
        anim = body.animate(
          [
            { height: `${start}px`, opacity: 1 },
            { height: "0px", opacity: 0 },
          ],
          { duration: 170, easing: "ease", fill: "forwards" },
        );
        anim.onfinish = () => {
          det.open = false;
          if (anim) anim.cancel();
          anim = null;
        };
      } else {
        det.open = true;
        const end = body.offsetHeight;
        anim = body.animate(
          [
            { height: "0px", opacity: 0 },
            { height: `${end}px`, opacity: 1 },
          ],
          { duration: 170, easing: "ease" },
        );
        anim.onfinish = () => {
          anim = null;
        };
      }
    });
  });
}

async function bootstrap() {
  applyTheme(state.theme);
  applyLang();
  initPreviewResizer();
  initFilmstripResizer();
  initResultsResizer();
  initAccordions();
  applyOutputDir(localStorage.getItem(OUTPUT_DIR_KEY) || "", false);
  renderTree();
  renderLogEmptyState();

  els.langToggle.addEventListener("click", () => {
    state.lang = state.lang === "zh" ? "en" : "zh";
    localStorage.setItem("mitWebviewLang", state.lang);
    applyLang();
    // applyLang reset elements to their data-i18n defaults; restore dynamic labels.
    setPreviewCollapsed(els.canvas.classList.contains("preview-collapsed"));
    applyOutputDir(state.outputDir, false);
    updateDeviceModeHint();
    updateDeviceBanner();
  });
  els.togglePreview.addEventListener("click", () => {
    setPreviewCollapsed(!els.canvas.classList.contains("preview-collapsed"));
  });
  els.themeToggle.addEventListener("click", () => {
    applyTheme(state.theme === "dark" ? "light" : "dark");
  });
  els.pickImages.addEventListener("click", chooseImages);
  els.pickFolder.addEventListener("click", chooseFolder);
  els.clearInputs.addEventListener("click", clearInputs);
  els.pickOutputDir.addEventListener("click", chooseOutputDir);
  els.reloadDefaults.addEventListener("click", loadDefaults);
  els.loadConfig.addEventListener("click", loadConfig);
  els.saveConfig.addEventListener("click", saveConfig);
  els.openModels.addEventListener("click", openModelsModal);
  els.closeModels.addEventListener("click", closeModelsModal);
  els.pickModelDir.addEventListener("click", pickModelDir);
  els.autoDownload.addEventListener("change", () => setAutoDownload(els.autoDownload.checked));
  els.modelsStatus.addEventListener("click", (event) => {
    const dl = event.target.closest("[data-model-download]");
    if (dl) {
      const id = dl.dataset.modelDownload;
      downloadModelTargets(id === "*" ? [] : [id], dl);
      return;
    }
    if (event.target.closest("[data-model-refresh]")) refreshModelsStatus();
  });
  els.gpuRuntime.addEventListener("click", (event) => {
    const dl = event.target.closest("[data-gpu-download]");
    if (dl) {
      downloadCudaRuntimeFlow(dl);
      return;
    }
    if (event.target.closest("[data-gpu-driver-update]")) {
      openReleaseNotes("https://www.nvidia.com/Download/index.aspx");
      return;
    }
    if (event.target.closest("[data-gpu-refresh]")) {
      refreshGpuRuntime();
      return;
    }
    if (event.target.closest("[data-gpu-restart]")) {
      triggerRestart();
      return;
    }
    if (event.target.closest("[data-gpu-restart-cancel]")) {
      refreshGpuRuntime();
    }
  });
  els.openUpdate.addEventListener("click", async () => {
    await openModelsModal();
    els.appUpdate?.scrollIntoView({ block: "nearest" });
    checkUpdateFlow();
  });
  els.appUpdate.addEventListener("click", (event) => {
    if (event.target.closest("[data-update-check]")) {
      checkUpdateFlow();
      return;
    }
    if (event.target.closest("[data-update-download]")) {
      downloadUpdateFlow();
      return;
    }
    if (event.target.closest("[data-update-install]")) {
      installUpdateFlow();
      return;
    }
    const link = event.target.closest("[data-update-notes-link]");
    if (link) {
      openReleaseNotes(link.dataset.updateNotesLink);
      return;
    }
    if (event.target.closest("[data-update-notes-toggle]")) {
      state.update.notesExpanded = !state.update.notesExpanded;
      renderAppUpdate();
    }
  });
  els.modelsModal.addEventListener("click", (event) => {
    if (event.target === els.modelsModal) closeModelsModal();
  });
  els.cudaErrorToggle.addEventListener("click", () => {
    state.cudaErrorExpanded = !state.cudaErrorExpanded;
    els.cudaErrorDetail.classList.toggle("hidden", !state.cudaErrorExpanded);
    els.cudaErrorToggle.textContent = state.cudaErrorExpanded ? t("cudaHide") : t("cudaDetails");
  });
  document.addEventListener("keydown", (event) => {
    if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "s") {
      event.preventDefault();
      saveConfig();
    }
  });
  els.provider.addEventListener("change", applyProviderPreset);
  const configControls = [
    els.translator,
    els.targetLang,
    els.filterLang,
    els.preDict,
    els.postDict,
    els.textDirection,
    els.detector,
    els.detectSize,
    els.unclipRatio,
    els.textThreshold,
    els.boxThreshold,
    els.preInvert,
    els.preGamma,
    els.preRotate,
    els.preAutoRotate,
    els.ocrModel,
    els.minTextLength,
    els.ocrProb,
    els.beamSize,
    els.filterText,
    els.maskMethod,
    els.ignoreBubble,
    els.dilationOffset,
    els.kernelSize,
    els.furigana,
    els.inpainter,
    els.inpaintingSize,
    els.maskSource,
    els.inpaintColor,
    els.upscaler,
    els.upscalePatch,
    els.upscalePadding,
    els.baseUrl,
    els.apiKey,
    els.modelName,
    els.timeoutSecs,
    els.temperature,
    els.topP,
    els.systemPrompt,
    els.userPrompt,
  ];
  configControls.forEach((node) => {
    node.addEventListener("input", () => {
      try {
        patchSettingsFromControls();
      } catch (_) {}
    });
    node.addEventListener("change", () => {
      try {
        patchSettingsFromControls();
      } catch (_) {}
      refreshGuidance();
    });
  });
  els.settingsJson.addEventListener("change", () => {
    try {
      state.settings = JSON.parse(els.settingsJson.value || "{}");
      syncControlsFromSettings();
    } catch (err) {
      setStatus(t("jsonError"), err.message);
      addLog("error", `${t("jsonError")}: ${err.message}`);
    }
  });
  els.startTranslation.addEventListener("click", startTranslation);
  els.selectAllResults.addEventListener("click", toggleAllResults);
  els.exportSelected.addEventListener("click", exportSelectedResults);
  els.inputList.addEventListener("change", (event) => {
    const cb = event.target.closest("input.tree-check");
    if (!cb) return;
    const row = event.target.closest(".tree-row");
    const node = row && findNode(row.dataset.path);
    if (node) toggleCheck(node, cb.checked);
  });
  els.inputList.addEventListener("click", (event) => {
    // Checkbox is handled by the change listener; keep click independent.
    if (event.target.closest("input.tree-check")) return;
    const row = event.target.closest(".tree-row");
    if (!row) return;
    const node = findNode(row.dataset.path);
    if (!node) return;
    const action = event.target.closest("[data-action]")?.dataset.action;
    if (action === "remove") {
      removeRoot(node.path);
    } else if (action === "toggle" || node.isDir) {
      setActiveNode(node, { preview: false });
      toggleNode(node);
    } else {
      // File row: single-click selects and previews on the canvas.
      selectAndPreview(node);
    }
  });
  els.inputList.addEventListener("keydown", handleTreeKey);
  els.inputList.addEventListener("contextmenu", (event) => {
    const row = event.target.closest(".tree-row");
    if (!row) return;
    event.preventDefault();
    const node = findNode(row.dataset.path);
    if (!node) return;
    const isRoot = state.tree.some((rootNode) => rootNode.path === node.path);
    setActiveNode(node, { preview: false });
    showTreeContextMenu(node, event.clientX, event.clientY, isRoot);
  });
  els.treeSearch.addEventListener("input", () => {
    state.filter = els.treeSearch.value.trim().toLowerCase();
    renderTree();
  });
  // Dismiss the context menu on any outside interaction.
  document.addEventListener("click", () => closeTreeContextMenu());
  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      closeTreeContextMenu();
      closeModelsModal();
    }
  });
  window.addEventListener("blur", () => closeTreeContextMenu());
  els.inputList.addEventListener("scroll", () => closeTreeContextMenu());
  els.results.addEventListener("click", (event) => {
    if (event.target.closest("input.result-check-box")) return; // change handles it
    const row = event.target.closest(".result-row");
    if (row) previewResultOutput(Number(row.dataset.resultIndex));
  });
  els.results.addEventListener("change", (event) => {
    const cb = event.target.closest("input.result-check-box");
    if (cb) toggleResult(Number(cb.dataset.resultIndex), cb.checked);
  });
  // Double-click opens the output in the OS image viewer; right-click reveals it.
  els.results.addEventListener("dblclick", (event) => {
    const row = event.target.closest(".result-row");
    if (row) previewResult(Number(row.dataset.resultIndex));
  });
  els.results.addEventListener("contextmenu", (event) => {
    const row = event.target.closest(".result-row");
    if (!row) return;
    const item = state.results[Number(row.dataset.resultIndex)];
    if (!item?.output) return;
    event.preventDefault();
    showTreeContextMenu({ path: item.output }, event.clientX, event.clientY, false);
  });
  els.clearLog.addEventListener("click", () => {
    els.logList.innerHTML = "";
    renderLogEmptyState();
  });

  if (els.toggleEdit) els.toggleEdit.addEventListener("click", () => enterEditMode());
  if (els.exitEdit) els.exitEdit.addEventListener("click", () => exitEditMode());

  setDeviceMode(localStorage.getItem("mitWebviewDevice") || "auto");
  els.deviceModeGroup.addEventListener("change", () => {
    const mode = getDeviceMode();
    localStorage.setItem("mitWebviewDevice", mode);
    updateDeviceModeHint();
    updateDeviceBanner();
  });
  els.deviceBanner.addEventListener("click", (event) => {
    if (event.target.closest("[data-device-banner-open]")) openModelsModal();
  });

  els.debugMode.checked = localStorage.getItem("mitWebviewDebug") === "1";
  els.debugMode.addEventListener("change", () => {
    localStorage.setItem("mitWebviewDebug", els.debugMode.checked ? "1" : "0");
  });

  try {
    const ready = await invoke("appReady");
    const diagnostics = ready.diagnostics || {};
    els.backendBadge.textContent = `${ready.backend} / ${ready.platform}`;
    state.appVersion = ready.version || "";
    renderAppUpdate();
    updateProviderStatus(diagnostics.provider_status || "CUDA unknown");
    updateCudaError(diagnostics);
    addLog("success", `${t("backendReady")}: ${ready.version}`);
    addLog(
      diagnostics.cuda_available ? "success" : "info",
      `Runtime: ${diagnostics.provider_status || "unknown"}; cuda feature=${Boolean(diagnostics.cuda_feature)}`,
    );
    if (diagnostics.cuda_error) {
      addLog("warn", `CUDA: ${summarizeText(diagnostics.cuda_error, LOG_SUMMARY_LIMIT)}`);
    }
  } catch (err) {
    els.backendBadge.textContent = "IPC unavailable";
    setStatus(t("ipcUnavailable"), err.message);
    addLog("error", err.message);
  }

  // Probe GPU runtime once (nvidia-smi etc.) without blocking startup; the
  // device banner updates when it resolves.
  invoke("getGpuRuntimeStatus")
    .then((status) => {
      state.gpuStatus = status;
      updateDeviceBanner();
    })
    .catch(() => {});

  await loadConfig();
}

bootstrap();
