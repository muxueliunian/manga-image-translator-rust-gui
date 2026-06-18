const i18n = {
  zh: {
    title: "漫画图片翻译",
    subtitle: "本地漫画页翻译与导出",
    inputTitle: "输入队列",
    inputStatsEmpty: "0 项 · 0 文件 · 0 文件夹",
    inputStats: "{total} 项 · {files} 文件 · {folders} 文件夹",
    pickImages: "添加图片",
    pickFolder: "添加文件夹",
    clearInputs: "清空",
    pathKindFile: "文件",
    pathKindFolder: "文件夹",
    noInput: "尚未添加输入。可多次添加图片或文件夹。",
    outputGroup: "输出",
    runtimeGroup: "运行",
    outputDir: "导出目录",
    outputFormat: "输出格式",
    textDirection: "文字方向",
    requireCuda: "强制 CUDA",
    requireCudaHint: "勾选后 CUDA 不可用时直接报错，不静默回退 CPU。",
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
  },
  en: {
    title: "Manga Image Translator",
    subtitle: "Local manga page translation and export",
    inputTitle: "Input Queue",
    inputStatsEmpty: "0 items · 0 files · 0 folders",
    inputStats: "{total} items · {files} files · {folders} folders",
    pickImages: "Add Images",
    pickFolder: "Add Folders",
    clearInputs: "Clear",
    pathKindFile: "File",
    pathKindFolder: "Folder",
    noInput: "No input yet. Add images or folders in multiple passes.",
    outputGroup: "Output",
    runtimeGroup: "Runtime",
    outputDir: "Export Directory",
    outputFormat: "Output Format",
    textDirection: "Text Direction",
    requireCuda: "Require CUDA, no CPU fallback",
    requireCudaHint: "Fail fast if CUDA is unavailable instead of silently falling back to CPU.",
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

const state = {
  lang: localStorage.getItem("mitWebviewLang") || "zh",
  theme: localStorage.getItem("mitWebviewTheme") || "dark",
  inputPaths: [],
  outputDir: "",
  results: [],
  selectedResults: new Set(),
  settings: null,
  requestId: 0,
  pending: new Map(),
  isRunning: false,
  cudaErrorExpanded: false,
};

const els = {
  langToggle: document.getElementById("langToggle"),
  themeToggle: document.getElementById("themeToggle"),
  backendBadge: document.getElementById("backendBadge"),
  pickImages: document.getElementById("pickImages"),
  pickFolder: document.getElementById("pickFolder"),
  clearInputs: document.getElementById("clearInputs"),
  pickOutputDir: document.getElementById("pickOutputDir"),
  outputDir: document.getElementById("outputDir"),
  outputFormat: document.getElementById("outputFormat"),
  textDirection: document.getElementById("textDirection"),
  providerStatus: document.getElementById("providerStatus"),
  cudaErrorWrap: document.getElementById("cudaErrorWrap"),
  cudaErrorSummary: document.getElementById("cudaErrorSummary"),
  cudaErrorToggle: document.getElementById("cudaErrorToggle"),
  cudaErrorDetail: document.getElementById("cudaErrorDetail"),
  requireCuda: document.getElementById("requireCuda"),
  debugMode: document.getElementById("debugMode"),
  maxParallelImages: document.getElementById("maxParallelImages"),
  maxParallelGpuJobs: document.getElementById("maxParallelGpuJobs"),
  inputList: document.getElementById("inputList"),
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
  startTranslation: document.getElementById("startTranslation"),
  statusTitle: document.getElementById("statusTitle"),
  statusText: document.getElementById("statusText"),
  progressBar: document.getElementById("progressBar"),
  progressLabel: document.getElementById("progressLabel"),
  selectAllResults: document.getElementById("selectAllResults"),
  exportSelected: document.getElementById("exportSelected"),
  resultStats: document.getElementById("resultStats"),
  results: document.getElementById("results"),
  logList: document.getElementById("logList"),
  clearLog: document.getElementById("clearLog"),
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
  els.langToggle.textContent = state.lang === "zh" ? "English" : "中文";
  if (els.cudaErrorToggle && !els.cudaErrorWrap.classList.contains("hidden")) {
    els.cudaErrorToggle.textContent = state.cudaErrorExpanded ? t("cudaHide") : t("cudaDetails");
  }
  const startLabel = els.startTranslation.querySelector(".start-label");
  if (startLabel && !state.isRunning) {
    startLabel.textContent = t("start");
  }
  renderInputList();
  renderResults();
  renderLogEmptyState();
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

function classifyInputPath(path) {
  const name = path.split(/[\\/]/).filter(Boolean).pop() || "";
  const dot = name.lastIndexOf(".");
  if (dot <= 0) return "folder";
  const ext = name.slice(dot).toLowerCase();
  return IMAGE_EXTENSIONS.has(ext) ? "file" : "folder";
}

function countInputKinds() {
  let files = 0;
  let folders = 0;
  state.inputPaths.forEach((path) => {
    if (classifyInputPath(path) === "file") files += 1;
    else folders += 1;
  });
  return { total: state.inputPaths.length, files, folders };
}

function renderInputStats() {
  const counts = countInputKinds();
  els.inputStats.textContent =
    counts.total === 0
      ? t("inputStatsEmpty")
      : t("inputStats", counts);
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

function renderInputList() {
  renderInputStats();
  els.inputList.innerHTML = "";

  if (!state.inputPaths.length) {
    els.inputList.classList.add("is-empty");
    els.inputList.textContent = t("noInput");
    return;
  }

  els.inputList.classList.remove("is-empty");

  state.inputPaths.forEach((path, index) => {
    const kind = classifyInputPath(path);
    const name = path.split(/[\\/]/).filter(Boolean).pop() || path;
    const item = document.createElement("div");
    item.className = "path-item";
    item.innerHTML = `
      <div class="path-text">
        <span class="path-kind">${escapeHtml(t(kind === "file" ? "pathKindFile" : "pathKindFolder"))}</span>
        <strong title="${escapeAttr(path)}">${escapeHtml(truncateMiddle(name, 36))}</strong>
      </div>
      <button class="tiny-button" type="button" data-remove-input="${index}">${escapeHtml(t("remove"))}</button>
    `;
    const strong = item.querySelector("strong");
    if (strong) strong.title = path;
    els.inputList.append(item);
  });
}

function addInputPaths(paths) {
  const before = state.inputPaths.length;
  const seen = new Set(state.inputPaths.map((path) => normalizePathKey(path)));
  (paths || []).forEach((path) => {
    if (!path) return;
    const key = normalizePathKey(path);
    if (seen.has(key)) return;
    seen.add(key);
    state.inputPaths.push(path);
  });
  return state.inputPaths.length - before;
}

function normalizePathKey(path) {
  return String(path || "").trim().replaceAll("\\", "/").toLowerCase();
}

function removeInput(index) {
  state.inputPaths.splice(index, 1);
  renderInputList();
  setStatus(t("selected"), `${state.inputPaths.length} ${t("selected")}`);
}

function clearInputs() {
  state.inputPaths = [];
  renderInputList();
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
    const added = addInputPaths(data.paths || []);
    renderInputList();
    setStatus(t("selected"), `${state.inputPaths.length} ${t("selected")}`);
    addLog("info", `${t("added")}: ${added}; ${t("selected")}: ${state.inputPaths.length}`);
  } catch (err) {
    addLog("error", err.message);
  }
}

async function chooseFolder() {
  try {
    setStatus(t("starting"), t("openingFolder"));
    const data = await invoke("pickFolder");
    const added = addInputPaths(data.paths || []);
    renderInputList();
    setStatus(t("folderSelected"), `${state.inputPaths.length} ${t("selected")}`);
    addLog("info", `${t("folderSelected")}: ${t("added")} ${added}; ${t("selected")}: ${state.inputPaths.length}`);
  } catch (err) {
    addLog("error", err.message);
  }
}

async function chooseOutputDir() {
  try {
    setStatus(t("starting"), t("openingOutput"));
    const data = await invoke("pickOutputDir");
    state.outputDir = (data.paths || [])[0] || "";
    els.outputDir.value = state.outputDir;
    if (state.outputDir) {
      setStatus(t("outputSelected"), truncateMiddle(state.outputDir, 72));
      addLog("info", `${t("outputSelected")}: ${state.outputDir}`);
    }
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
  if (
    els.provider.value !== "Custom" ||
    els.baseUrl.value.trim() ||
    els.apiKey.value.trim()
  ) {
    els.translator.value = "OpenAICompatible";
  }
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

  try {
    setRunningState(true);
    updateProgress({ current: 0, total: state.inputPaths.length || 1, message: t("progressPreparing") });
    setStatus(t("starting"), t("backendPending"));
    const result = await invoke("startTranslation", {
      input_paths: state.inputPaths,
      settings,
      output_format: els.outputFormat.value,
      require_cuda: els.requireCuda.checked,
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

function renderResult(result) {
  const outputs = Array.isArray(result.outputs) ? result.outputs : [];
  state.results = outputs;
  state.selectedResults = new Set(
    outputs
      .filter((item) => item.status === "done" && item.output)
      .map((item) => item.output),
  );
  renderResults(result);
}

function renderResults(result = null) {
  const counts = countResults(state.results);
  els.resultStats.textContent =
    state.results.length === 0
      ? t("resultStatsEmpty")
      : t("resultStats", counts);

  if (!state.results.length) {
    els.results.className = "empty-state";
    els.results.textContent = t("resultEmpty");
    els.selectAllResults.textContent = t("selectAllResults");
    return;
  }

  els.results.className = "result-list";
  const summary = result
    ? `<div class="result-summary"><strong>${escapeHtml(statusLabel(result.status || "done"))}</strong><span class="muted">${escapeHtml(summarizeText(result.message || "", 100))}</span></div>`
    : "";
  const rows = state.results.map((item, index) => resultCard(item, index)).join("");
  els.results.innerHTML = `${summary}<div class="result-grid">${rows}</div>`;

  const doneOutputs = state.results.filter((item) => item.status === "done" && item.output);
  const allSelected =
    doneOutputs.length > 0 && doneOutputs.every((item) => state.selectedResults.has(item.output));
  els.selectAllResults.textContent = allSelected ? t("deselectAllResults") : t("selectAllResults");
}

function resultCard(item, index) {
  const output = item.output || "";
  const checked = output && state.selectedResults.has(output) ? "checked" : "";
  const canUse = item.status === "done" && output;
  const status = item.status || "";
  const thumb = canUse && output.toLowerCase().endsWith(".png")
    ? `<div class="result-thumb"><img alt="" src="${escapeHtml(fileUrl(output))}"></div>`
    : `<div class="result-thumb">${escapeHtml(statusLabel(status))}</div>`;
  const message = item.message ? summarizeText(item.message, 80) : "";
  return `
    <article class="result-item" data-status="${escapeHtml(status)}">
      <label class="result-check">
        <input type="checkbox" data-result-index="${index}" ${checked} ${canUse ? "" : "disabled"}>
        <span title="${escapeAttr(item.file_name || status)}">${escapeHtml(item.file_name || statusLabel(status))}</span>
        <span class="status-badge" data-status="${escapeHtml(status)}">${escapeHtml(statusLabel(status))}</span>
      </label>
      ${thumb}
      <p class="result-path" title="${escapeAttr(output || item.input || "")}">${escapeHtml(truncateMiddle(output || item.input || "-", 42))}</p>
      ${message ? `<p class="result-message" title="${escapeAttr(item.message || "")}">${escapeHtml(message)}</p>` : ""}
      <div class="result-actions-row">
        <button class="ghost-button small-button" type="button" data-preview-index="${index}" ${canUse ? "" : "disabled"}>${t("preview")}</button>
      </div>
    </article>
  `;
}

function fileUrl(path) {
  return `file:///${String(path).replaceAll("\\", "/").split("/").map(encodeURIComponent).join("/")}`;
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
  if (!state.outputDir) {
    setStatus(t("exportNeedDir"), "");
    addLog("error", t("exportNeedDir"));
    return;
  }
  try {
    const data = await invoke("exportResults", {
      outputs,
      export_dir: state.outputDir,
    });
    const count = Array.isArray(data.exported) ? data.exported.length : 0;
    setStatus(t("exported"), `${count} ${t("selected")}`);
    addLog("success", `${t("exported")}: ${count}`);
  } catch (err) {
    setStatus(t("backendPending"), summarizeText(err.message, 120));
    addLog("error", err.message);
  }
}

async function bootstrap() {
  applyTheme(state.theme);
  applyLang();
  renderInputList();
  renderLogEmptyState();

  els.langToggle.addEventListener("click", () => {
    state.lang = state.lang === "zh" ? "en" : "zh";
    localStorage.setItem("mitWebviewLang", state.lang);
    applyLang();
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
  els.inputList.addEventListener("click", (event) => {
    const removeIndex = event.target?.dataset?.removeInput;
    if (removeIndex !== undefined) {
      removeInput(Number(removeIndex));
    }
  });
  els.results.addEventListener("click", (event) => {
    const previewIndex = event.target?.dataset?.previewIndex;
    if (previewIndex !== undefined) {
      previewResult(Number(previewIndex));
    }
  });
  els.results.addEventListener("change", (event) => {
    const resultIndex = event.target?.dataset?.resultIndex;
    if (resultIndex !== undefined) {
      toggleResult(Number(resultIndex), event.target.checked);
    }
  });
  els.clearLog.addEventListener("click", () => {
    els.logList.innerHTML = "";
    renderLogEmptyState();
  });

  els.debugMode.checked = localStorage.getItem("mitWebviewDebug") === "1";
  els.debugMode.addEventListener("change", () => {
    localStorage.setItem("mitWebviewDebug", els.debugMode.checked ? "1" : "0");
  });

  try {
    const ready = await invoke("appReady");
    const diagnostics = ready.diagnostics || {};
    els.backendBadge.textContent = `${ready.backend} / ${ready.platform}`;
    updateProviderStatus(diagnostics.provider_status || "CUDA unknown");
    els.requireCuda.checked = Boolean(diagnostics.require_cuda);
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

  await loadConfig();
}

bootstrap();
