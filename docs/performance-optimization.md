# Performance Optimization Notes

## Current Guardrails

- Do not lower `detector.options.detect_size`, `inpainter.inpainting_size`, or render quality as the default performance path.
- Compare before/after runs with the same images, settings, output format, and model files.
- Treat the first image as cold-start and later images as warm-start.

## Implemented Measurement

- Runtime diagnostics report whether the binary was built with the `cuda` feature, whether CUDA is available, and whether CUDA is required.
- `MIT_REQUIRE_CUDA=1` prevents silent CPU fallback when CUDA is unavailable.
- WebView exposes a CUDA requirement toggle plus image/GPU concurrency controls.
- WebView uses a bounded model pool for multi-image jobs; keep GPU concurrency low on 8 GB VRAM unless measured.
- Job logs are written to `logs/job_<timestamp>.log`.
- Logs include major stage timings, model preparation timing, render/write timing, and NVIDIA GPU memory samples when `nvidia-smi` is available.
- ONNX session creation logs the provider order and the selected execution provider.

## Baseline Procedure

1. Use a fixed test folder with representative manga pages.
2. Run the non-CUDA release build and save the generated `logs/job_*.log`.
3. Run the CUDA release build with the same config and save the generated log.
4. Compare cold-start first image, warm-start later images, total job time, selected provider, and GPU memory samples.
5. Confirm GPU activity with:

```powershell
nvidia-smi --query-gpu=name,utilization.gpu,memory.used,memory.total --format=csv
```

## Candidate Optimizations Not Yet Implemented

- Finer-grained model-pool policy that reuses hot ONNX sessions while avoiding excessive duplication in 8 GB VRAM.
- Local inpainting by mask bounding boxes instead of full-image inpainting.
- Translation cache and request batching for repeated text.
- Renderer cache for font shaping, glyph rasterization, and repeated style work.
- GUI-exposed ONNX Runtime provider and thread controls.
- FP16/TensorRT profiling for models that support it.
- Explicit model predownload and warm-up controls.
