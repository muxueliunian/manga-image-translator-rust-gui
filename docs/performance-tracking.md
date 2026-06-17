# 性能优化追踪

本文件是 Rust `simple-runtime` 翻译流水线的**性能优化工作日志**：

1. 记录当前的瓶颈调查结论（来自代码走读）。
2. 记录这次补齐的可观测能力，以及如何从 `job_*.log` 读出"每部分占比"。
3. 提供基线数据表与优化记录表，**之后每做一次优化，都把 before/after 量化填进来**。

配套文档：

- `docs/performance-optimization.md`：守则与候选优化清单（英文）。
- `DEVELOPMENT_GUIDE.md`：流水线结构、构建命令、便携包验证。

---

## 1. 目标与方法

- 衡量对象：单图主链路（预处理 → upscaler → detector → OCR → textline-merge → translator → mask-refinement → inpainter → compose → render → write）。
- **冷启动 vs 热启动必须分开看**：第一张图包含模型懒加载（下载 + ONNX session 创建），后续图是热启动。
- 对比口径固定：同一组图片、同一份 `Settings`、同一输出格式、同一套模型文件。
- 不通过降低 `detector.options.detect_size` / `inpainter.inpainting_size` / 渲染质量来"提速"——那是降质不是优化。

---

## 2. 这次补齐的可观测能力

改动集中在日志，不改算法。改动文件：

- `crates/runtimes/simple-runtime/src/perf.rs`：新增 `StageReport`，收集各阶段耗时并生成**按耗时排序 + 占比**的汇总表。
- `crates/runtimes/simple-runtime/src/execute/mod.rs`：每个阶段计时记入 `StageReport`，流水线结束时把汇总表写入 job log。
- `crates/runtimes/simple-runtime/src/webview_ui.rs` 与 `main.rs`：原 `render-write` 拆成 **`render`（生成字节）** 与 **`write`（落盘）** 两段独立计时。

### job 日志现在长这样（节选）

```
[ts][INFO] detector finished in 1.20s
[ts][INFO] ocr finished in 4.80s
...
[ts][INFO] stage timing summary (measured total 9.50s):
[ts][INFO]   ocr             4.80s   50.5%
[ts][INFO]   inpainter       2.10s   22.1%
[ts][INFO]   detector        1.20s   12.6%
[ts][INFO]   ...
[ts][INFO] image pipeline finished in 9.70s (measured stages 9.50s)
[ts][INFO] render finished in 0.40s
[ts][INFO] write finished in 0.05s
[ts][INFO] image output written: ...\out.png (10.2s)
```

要点：

- **`stage timing summary`** 就是"每部分占比"，直接看，不用手动做减法。
- 汇总表只覆盖**模型流水线（render 之前）**；`render` / `write` 是表外的独立行，`image output written` 的括号是含渲染落盘的整图总时间。
- `measured total`（各阶段之和）≈ `image pipeline finished` 的墙钟时间；正常关闭 debug 时两者接近，差值是未计时的间隙（debug 存图等）。

### 如何分离冷启动 / 模型加载

模型加载耗时目前**没有独立埋点**（会算进当前图第一次用到该模型的阶段里）。第一遍用下面的方法分离：

- 一次任务里放 **≥2 张图**（同一任务内模型池常驻，第 2 张起是热启动）。
- 对比第 1 张 vs 第 2 张的 `detector` / `ocr` / `inpainter` / `translator` 耗时，**差值≈该模型的加载成本**。
- 控制台（`run-ui-debug.bat`）里还有 `Model <kind>/<name> loaded in X s` 行可参考（但受日志级别影响，不一定出现；job log 更可靠）。

> 后续若确认需要，再把模型加载单独埋点写进 job log（需要跨 crate 改 `Model::load` / `Ocr` trait，属于第二步）。

---

## 3. 采集基线的操作步骤

> 重要：WebView 运行区的「调试输出」复选框必须**保持关闭**再测基线。开启它会写每张图的诊断中间产物（中间图、OCR 切片、JSON），既变慢又会把存盘 I/O 算进 OCR 等阶段、污染计时。关闭时轻量计时日志 `job_*.log` 仍照常写。

1. 用 CUDA release 构建，设 `MIT_REQUIRE_CUDA=1`，确认日志 provider 选了 CUDA、没有静默回退 CPU。
2. 准备一个固定测试集（建议 3–5 张有代表性的对话页）。
3. 跑一次完整任务，留存生成的 `logs/job_*.log`。
4. 从每张图的 `stage timing summary` 抄出各阶段耗时，填进下面【基线数据】表。
5. 用第 1 张 vs 第 2 张的差值估算模型加载成本。
6. 需要时用 `nvidia-smi` 确认 GPU 确实在跑：
   ```powershell
   nvidia-smi --query-gpu=name,utilization.gpu,memory.used,memory.total --format=csv
   ```

---

## 4. 瓶颈调查结论（代码走读，待基线数据验证）

按预估收益排序，**等基线数据出来后修正**：

| # | 位置 | 现象 | 优化方向 | 类型 |
|---|------|------|----------|------|
| 1 | `crates/modules/ocr/ocr-48px/src/infer.rs` | OCR 是自回归 encoder-decoder + beam search（beams_k=5, max_seq=255），decoder 逐 token 串行调用，且每步重建 `activation_cache` 做 O(seq²) host 拷贝 | 换 `ctc-48px`（单次前向）；或 beam_size 配置化降到 1；或 KV-cache 留设备侧 | 换模型 / 换算法 |
| 2 | `crates/modules/inpainter/lama_aot` | 整页在 `inpainting_size=2048` 下修补，文字稀疏也跑全图 | 按 mask bbox 局部裁剪修补再贴回 | 换算法 |
| 3 | `crates/base-util/src/onnx.rs` | 线程数写死（intra=4/inter=2）；provider 顺序 CUDA 在 TensorRT 前，N 卡上 TensorRT 永远进不去；全程 FP32 | 线程/EP/精度配置化；FP16/TensorRT engine 缓存 | 换 runtime/算法 |
| 4 | `crates/runtimes/simple-runtime/src/webview_ui.rs` | 多图并发整份复制 `Models`，显存翻倍；而 `AsyncSessionPool` 内部已能并发 | 共享只读 session，靠池内信号量做受控并发 | 换算法 |
| 5 | `execute/mod.rs` compose / `renderer/png` | compose 多次整图 clone；render 无字体测量/glyph 缓存；无翻译缓存 | 减少 clone；缓存 shaping；重复文本翻译缓存 | 换算法 |

> 注：以上为纯代码走读推断。**第 5 节实测已修正排序**——用 DeepSeek 这类 LLM 端点时 translator 反而是单图最大头（但网络/LLM 受限），本地侧则是 inpainter + ocr + detector，且受显存近满与模型 session 争用影响。

---

## 5. 基线数据（2026-06-16 实测）

- 机器：RTX 4070 Laptop 8GB。
- 构建：CUDA release 便携包。
- 测试集：2 张日文漫画页（`01.jpg` / `02.jpg`）。
- 翻译器：OpenAI Compatible（DeepSeek），日→简中。
- 并发：`max_parallel_images=2` / `max_parallel_gpu_jobs=2`；两图并发处理。
- **debug=ON**（诊断转储开启，会略微抬高耗时；干净基线应 debug=OFF 重测）。
- 均为热启动（`model-prepare 0ms`，模型已常驻）。
- 日志：`logs/job_1781619508991.log`。

**墙钟总时长：17.62s / 2 图。** 各图阶段（measured total）：

| 阶段 | 01.jpg | 02.jpg | 说明 |
|------|-------:|-------:|------|
| translator (LLM) | 8.23s | 8.02s | DeepSeek 网络往返，最大头；两请求并发重叠 |
| inpainter | 4.95s | 1.69s | 整页 LaMa；差异大 = GPU 争用 + 显存近满 |
| ocr | 2.41s | 1.22s | Ocr48px 自回归；受并发争用影响 |
| detector | 1.10s | 0.49s | DBNet |
| mask-refinement | 0.25s | 0.30s |  |
| compose | 0.14s | 0.10s |  |
| render + write | ~0.10s | ~0.12s | 表外独立行 |
| **measured total** | 17.09s | 11.83s | image output written 括号值 17.57s / 12.45s |

显存：全程约 7.6–7.9 GB / 8.2 GB（**≈96%，逼近 OOM**），因为模型池开了 2 份 `Models` 各自持有 ONNX session。

**关键结论（修正第 4 节的纯代码推断）：**

1. **translator（LLM/DeepSeek）是单图最大耗时（~8s，48–68%），但属网络/LLM 延迟，不是本地算法**。并发已让两请求重叠，所以总时长没翻倍。提速主要靠更快的模型/更小输出/或本地 MT（多属"换模型/配置"而非"优化算法"）。
2. **本地 GPU 计算里 inpainter + ocr + detector 是大头**，且都呈约 2× 抖动——根因是**显存近满 + 2 份模型 session 互相争用**。
3. inpainter 整页 LaMa 在显存紧张时尤其慢（4.95s）。
4. debug=ON 抬高了计时；后续每次对比都应 debug=OFF。

**据此修正的优化优先级：**

- **A. 降低显存占用（模型池共享只读 session，而非整份复制 `Models`）** —— 直接缓解 96% 显存压力与争用，让 inpainter/ocr/detector 抖动收敛，并允许更高并发。**最高杠杆**。
- **B. inpainter 改 bbox 局部修补** —— 减少 inpainter 时间与显存。
- **C. OCR `Ctc48px` / beam 配置化 / 新增 `PP-OCRv6`** —— 减少 ocr 时间（质量需 A/B）。
- **D. translator** —— 本地算法层能做的有限：加翻译缓存（重复台词/拟声词）、保持整页一次批量请求（已是）。换更快 LLM / 本地 MT 属选型。

### PP-OCRv6 排期（2026-06-17 加入计划）

新增 PP-OCRv6 系列 OCR 模型属"模型选型"，与 A/B/D 的"算法/内存优化"是两类事，且**新增 OCR 会改变 baseline**，中途换 OCR 会污染 inpainter/显存/缓存这几项的 before/after 对比。因此排期如下：

1. inpainter bbox 局部修补（B）。
2. 共享只读 session、降显存（A，文档标为最高杠杆）。
3. 翻译缓存（D 的本地部分）。
4. **OCR 统一 A/B**：在固定 baseline 上一次性对比 `Ocr48px`（现状）/ `Ctc48px` / beam 配置化 / **`PP-OCRv6`**，按速度 + 质量选默认。
5. PP-OCRv6 的**模块开发可并行**起独立分支推进（实现 trait → settings enum → setup 注册 → execute 接入 → schema/GUI），但只在第 4 步并入对比与默认切换，避免干扰前三项测量。

### OCR A/B 实测结论（2026-06-18）

对第 4 步「OCR 统一 A/B」做了实测，对比项均为零代码配置切换（三者都已在 `setup/ocr.rs` 注册）：

- **识别准确率排名：`Ocr48px` > `Ctc48px` > `MangaOcr`**。
- 据此**默认维持 `Ocr48px`**（本就是默认），不切换。`Ctc48px` 虽更快（CTC 单次前向）但准确率不及，`MangaOcr` 在本测试集上反而最差且较重、且不预测文字颜色（`fg/bg=None`，渲染回退黑白）。

**社区新模型调研结论（不投入）：**

- PaddleOCR 最新是 **PP-OCRv5（无 v6）**；通用域、非漫画专训，且 rec 不预测颜色——预期对日漫准确率无优势，搁置。
- 准确率最强的是 **PaddleOCR-VL-For-Manga**（VLM，Manga109-s 微调，整句 ~27%→70%），但属重模型、吃显存、难塞进 ort 流水线，与"轻量本地 + 8GB 显存近满"现状冲突，**不投入**。
- 共识：要进一步提升识别/翻译质量，方向是 **LLM 介入（上下文 + 多模态）**，但难度高且"重"，非当前轻量本地优化范围。

**已做的代码改动：beam 配置化（仅 `Ocr48px` 生效，尚未实测收益）**

- `beams_k` 原硬编码 `5`，已提为配置项 `ocr.beam_size`（默认 5），运行时经 `OcrOptions` 注入；`<1` 自动夹到 1。
- 已暴露到 WebView GUI（OCR 区「Beam 宽度」数字框，附 hint：1=贪心最快略降准确率，越大越稳越慢，仅 Ocr48px 生效）。
- schema/example 同步。**收益与质量回退尚未实测**——待后续按统一基线测 beam=1 vs 5 后再补一行到下方表格。

---

## 6. 优化记录（每优化一次追加一行）

口径：同测试集、同设置、同机器；记录**热启动均值**的 before/after。

| 日期 | 优化项 | 改动概述 | 目标阶段 | Before | After | 提升 | 质量回归? | 备注/日志 |
|------|--------|----------|----------|------:|-----:|-----:|-----------|-----------|
| 2026-06-16 | **基线** | 补齐阶段汇总日志 + debug 开关（仅可观测） | — | 17.62s / 2图 | — | — | 无 | `job_1781619508991.log`；images=2/gpu=2，debug=ON，DeepSeek |
| 2026-06-17 | **① inpainter bbox 局部修补** | 按 mask 连通域 bbox（dilate padding=32 合并 + 上下文）裁剪小图分别修补再贴回；碎框>32 或覆盖>60% 自动回退整页；空 mask 直接返回原图。helper 在 `util/lama.rs`，已 port 到全部三模型 `lama_aot`/`lama_large`/`lama_mpe`（mpe 的 rel_pos/direct 按局部 mask 计算） | inpainter | 0.93s/图 | 0.36s/图 | **-61%（≈2.6×）** | 无（已目视确认，与整页修补几乎无差异） | 热运行 4 图均值；基线 `job_1781711769500` vs 优化 `job_1781711808387`；debug=OFF, images=2/gpu=1, DeepSeek |
| 2026-06-17 | **② 共享只读 session / 降显存** | 模型池从 `Vec<Models>` 整份复制改为单份 `Arc<Models>` 共享；translator 存储改 `Mutex<HashMap>` 内部可变，整条 execute 走 `&self`；并发靠 `gpu_semaphore` 受控提交，依赖 `AsyncSessionPool` 内部并发 | 显存 / 架构 | 显存 ~7.5GB（pool=1） | ~7.5GB（pool=1） | 本配置无变化、无回归 | 无（不改算法） | 两包均 “1 instance”，本测未触发整份复制故显存无差；②的价值是高并发(pool>1)防翻倍 **且使 ③ 的缓存能跨图共享** |
| 2026-06-17 | **③ 翻译缓存** | 在共享 `Arc<Models>` 上加 `TranslationCache(Mutex<HashMap>)`，按**单条 textblock 文本**缓存译者输出；key = `hash(译者链+配置)` + 原文（改语言/模型/prompt 自动失效）；仅对未命中的文本发起翻译，命中跳过 LLM。pre-dict/post-dict 是独立 stage，不影响缓存正确性 | translator | 2.78–6.87s/图 | 0ms（全命中） | 重复内容 ≈ **-100%** | 无（命中即原译文复用） | **全命中是因为重跑同一批图**；首次翻译新内容仍走 LLM（不命中），缓存只省重复台词/角色名/重跑 |
| 2026-06-18 | **合计（①②③ 热运行）** | 同批 4 图，debug=OFF，images=2/gpu=1，DeepSeek，模型已加载、缓存已预热 | 整体 | **32.78s / 4图** | **10.17s / 4图** | **-69%（3.2×）** | 见各项 | 基线 `job_1781711769500` vs 优化 `job_1781711808387`。提速主因：缓存命中省去 ~18.5s LLM + inpainter 省 ~2.3s。**注意**：此倍数含“重跑命中”红利，首翻新章不会有 translator=0 |

> 填写提醒：
> - "提升"写百分比或倍数（如 `-35%` 或 `1.5x`）。
> - 若优化伴随质量变化（如 OCR 换模型），必须在"质量回归?"列写明并附对比样张路径。
> - 大改动同时贴上对应 `job_*.log` 文件名，便于回溯。
