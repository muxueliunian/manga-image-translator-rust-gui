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
- **C. OCR `Ctc48px` / beam 配置化** —— 减少 ocr 时间（质量需 A/B）。
- **D. translator** —— 本地算法层能做的有限：加翻译缓存（重复台词/拟声词）、保持整页一次批量请求（已是）。换更快 LLM / 本地 MT 属选型。

---

## 6. 优化记录（每优化一次追加一行）

口径：同测试集、同设置、同机器；记录**热启动均值**的 before/after。

| 日期 | 优化项 | 改动概述 | 目标阶段 | Before | After | 提升 | 质量回归? | 备注/日志 |
|------|--------|----------|----------|------:|-----:|-----:|-----------|-----------|
| 2026-06-16 | **基线** | 补齐阶段汇总日志 + debug 开关（仅可观测） | — | 17.62s / 2图 | — | — | 无 | `job_1781619508991.log`；images=2/gpu=2，debug=ON，DeepSeek |
|  |  |  |  |  |  |  |  |  |

> 填写提醒：
> - "提升"写百分比或倍数（如 `-35%` 或 `1.5x`）。
> - 若优化伴随质量变化（如 OCR 换模型），必须在"质量回归?"列写明并附对比样张路径。
> - 大改动同时贴上对应 `job_*.log` 文件名，便于回溯。
