param(
    [switch]$Cuda,
    [switch]$NoZip,
    [switch]$DownloadCudaRuntime,
    [string[]]$CudaRuntimeDir = @()
)

$ErrorActionPreference = "Stop"

if (-not $IsWindows -and $PSVersionTable.PSEdition -eq "Core") {
    Write-Warning "This package script is intended for Windows portable builds."
}

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..")
$ReleaseDir = Join-Path $RepoRoot "target\release"
$DistDir = Join-Path $RepoRoot "dist"
$PortableName = "manga-image-translator-rust-portable"
$PortableDir = Join-Path $DistDir $PortableName
$BinaryName = "simple-runtime.exe"
$BinaryPath = Join-Path $ReleaseDir $BinaryName
$ToolsRoot = Resolve-Path (Join-Path $RepoRoot "..\tools") -ErrorAction SilentlyContinue
$CudaRuntimeCacheDir = Join-Path (Join-Path $RepoRoot "..\tools") "cuda-runtime-cu12"

function Write-Step {
    param([string]$Message)
    Write-Host "==> $Message"
}

function Use-IfExists {
    param([string]$Path)

    if ([string]::IsNullOrWhiteSpace($Path)) {
        return $null
    }

    try {
        return (Resolve-Path -LiteralPath $Path -ErrorAction Stop).Path
    } catch {
        return $null
    }
}

function Initialize-LocalBuildEnvironment {
    if ($ToolsRoot) {
        $openCvRoot = Use-IfExists (Join-Path $ToolsRoot "opencv-4.11.0\opencv\build")
        if ($openCvRoot -and -not $env:OPENCV_LINK_PATHS) {
            $env:OPENCV_LINK_LIBS = "opencv_world4110"
            $env:OPENCV_LINK_PATHS = Join-Path $openCvRoot "x64\vc16\lib"
            $env:OPENCV_INCLUDE_PATHS = Join-Path $openCvRoot "include"
            $env:OPENCV_DISABLE_PROBES = "pkg_config,cmake,vcpkg_cmake,vcpkg"
            $env:OPENCV_BIN_DIR = Join-Path $openCvRoot "x64\vc16\bin"
            $env:PATH = "$env:OPENCV_BIN_DIR;$env:PATH"
            Write-Step "Using bundled OpenCV at $openCvRoot"
        }

        $llvmBin = Use-IfExists (Join-Path $ToolsRoot "LLVM-22.1.6\bin")
        if ($llvmBin -and -not $env:LIBCLANG_PATH) {
            $env:LIBCLANG_PATH = $llvmBin
            $env:PATH = "$llvmBin;$env:PATH"
            Write-Step "Using bundled LLVM/libclang at $llvmBin"
        }
    }
}

function Copy-IfExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Destination
    )

    if (Test-Path -LiteralPath $Path) {
        Copy-Item -LiteralPath $Path -Destination $Destination -Force
        return $true
    }

    return $false
}

function Add-UniquePath {
    param(
        [System.Collections.Generic.List[string]]$List,
        [string]$Path
    )

    if ([string]::IsNullOrWhiteSpace($Path)) {
        return
    }

    try {
        $resolved = (Resolve-Path -LiteralPath $Path -ErrorAction Stop).Path
    } catch {
        return
    }

    if ((Test-Path -LiteralPath $resolved -PathType Container) -and -not $List.Contains($resolved)) {
        [void]$List.Add($resolved)
    }
}

function Add-CudaRuntimeSearchRoot {
    param(
        [System.Collections.Generic.List[string]]$List,
        [string]$Path
    )

    Add-UniquePath $List $Path
    if ([string]::IsNullOrWhiteSpace($Path)) {
        return
    }

    Add-UniquePath $List (Join-Path $Path "bin")
    if (Test-Path -LiteralPath $Path -PathType Container) {
        Get-ChildItem -LiteralPath $Path -Directory -Recurse -Filter "bin" -ErrorAction SilentlyContinue |
            ForEach-Object { Add-UniquePath $List $_.FullName }
    }
}

function Get-OpenCvSearchDirs {
    $dirs = [System.Collections.Generic.List[string]]::new()

    Add-UniquePath $dirs $env:OPENCV_BIN_DIR
    Add-UniquePath $dirs $env:OPENCV_DIR

    if ($env:OPENCV_DIR) {
        Add-UniquePath $dirs (Join-Path $env:OPENCV_DIR "build\x64\vc16\bin")
        Add-UniquePath $dirs (Join-Path $env:OPENCV_DIR "build\x64\vc17\bin")
        Add-UniquePath $dirs (Join-Path $env:OPENCV_DIR "x64\vc16\bin")
        Add-UniquePath $dirs (Join-Path $env:OPENCV_DIR "x64\vc17\bin")
    }

    if ($env:OPENCV_LINK_PATHS) {
        foreach ($entry in ($env:OPENCV_LINK_PATHS -split ";")) {
            Add-UniquePath $dirs $entry
            Add-UniquePath $dirs (Join-Path (Split-Path -Parent $entry) "bin")
        }
    }

    foreach ($entry in ($env:Path -split ";")) {
        if ($entry -match "opencv") {
            Add-UniquePath $dirs $entry
        }
    }

    Add-UniquePath $dirs "C:\tools\opencv\build\x64\vc16\bin"
    Add-UniquePath $dirs "C:\tools\opencv\build\x64\vc17\bin"
    Add-UniquePath $dirs "C:\opencv\build\x64\vc16\bin"
    Add-UniquePath $dirs "C:\opencv\build\x64\vc17\bin"

    return $dirs
}

function Copy-OpenCvDlls {
    param([string]$Destination)

    $copied = 0
    foreach ($dir in (Get-OpenCvSearchDirs)) {
        $dlls = Get-ChildItem -LiteralPath $dir -Filter "opencv_world*.dll" -File -ErrorAction SilentlyContinue
        if (-not $dlls) {
            $dlls = Get-ChildItem -LiteralPath $dir -Filter "opencv_*.dll" -File -ErrorAction SilentlyContinue
        }

        foreach ($dll in $dlls) {
            if ($dll.BaseName -match "d$") {
                continue
            }
            Copy-Item -LiteralPath $dll.FullName -Destination $Destination -Force
            $copied++
        }
    }

    if ($copied -eq 0) {
        Write-Warning "OpenCV DLLs were not found. Set OPENCV_BIN_DIR, OPENCV_DIR, OPENCV_LINK_PATHS, or add OpenCV bin to PATH before packaging."
    }

    return $copied
}

function Get-CudaRuntimeSearchDirs {
    $dirs = [System.Collections.Generic.List[string]]::new()

    foreach ($dir in $CudaRuntimeDir) {
        Add-CudaRuntimeSearchRoot $dirs $dir
    }

    Add-CudaRuntimeSearchRoot $dirs $CudaRuntimeCacheDir
    Add-UniquePath $dirs $ReleaseDir
    Add-CudaRuntimeSearchRoot $dirs $env:CUDA_PATH
    if ($env:CUDA_PATH) {
        Add-UniquePath $dirs (Join-Path $env:CUDA_PATH "bin")
    }

    Get-ChildItem Env: -ErrorAction SilentlyContinue |
        Where-Object { $_.Name -like "CUDA_PATH_V12*" -and $_.Value } |
        ForEach-Object {
            Add-CudaRuntimeSearchRoot $dirs $_.Value
        }

    foreach ($entry in ($env:Path -split ";")) {
        if ($entry -match "CUDA|NVIDIA GPU Computing Toolkit|cudnn") {
            Add-UniquePath $dirs $entry
        }
    }

    $toolkitRoot = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA"
    if (Test-Path -LiteralPath $toolkitRoot -PathType Container) {
        Get-ChildItem -LiteralPath $toolkitRoot -Directory -ErrorAction SilentlyContinue |
            Where-Object { $_.Name -like "v12*" } |
            Sort-Object Name -Descending |
            ForEach-Object {
                Add-CudaRuntimeSearchRoot $dirs $_.FullName
            }
    }

    return $dirs
}

function Copy-CudaRuntimeDlls {
    param([string]$Destination)

    $requiredDlls = @(
        "cublasLt64_12.dll",
        "cublas64_12.dll",
        "cufft64_11.dll",
        "cudart64_12.dll",
        "cudnn64_9.dll"
    )
    $optionalDlls = @(
        "cudnn_adv64_9.dll",
        "cudnn_cnn64_9.dll",
        "cudnn_engines_precompiled64_9.dll",
        "cudnn_engines_runtime_compiled64_9.dll",
        "cudnn_graph64_9.dll",
        "cudnn_heuristic64_9.dll",
        "cudnn_ops64_9.dll",
        "nvrtc64_*.dll",
        "nvrtc-builtins64_*.dll",
        "nvJitLink64_*.dll"
    )

    $searchDirs = Get-CudaRuntimeSearchDirs
    $copied = 0
    $missing = [System.Collections.Generic.List[string]]::new()

    foreach ($dllName in $requiredDlls) {
        $source = $null
        foreach ($dir in $searchDirs) {
            $candidate = Join-Path $dir $dllName
            if (Test-Path -LiteralPath $candidate -PathType Leaf) {
                $source = $candidate
                break
            }
        }

        if ($source) {
            Copy-Item -LiteralPath $source -Destination $Destination -Force
            $copied++
        } else {
            [void]$missing.Add($dllName)
        }
    }

    foreach ($pattern in $optionalDlls) {
        foreach ($dir in $searchDirs) {
            Get-ChildItem -LiteralPath $dir -Filter $pattern -File -ErrorAction SilentlyContinue |
                Where-Object { -not $_.Name.StartsWith("._") } |
                ForEach-Object {
                    Copy-Item -LiteralPath $_.FullName -Destination $Destination -Force
                    $copied++
                }
        }
    }

    if ($missing.Count -gt 0) {
        $newline = [Environment]::NewLine
        $searched = ($searchDirs | ForEach-Object { "  - $_" }) -join $newline
        $missingText = ($missing | ForEach-Object { "  - $_" }) -join $newline
        throw @"
CUDA packaging was requested, but required CUDA runtime DLLs were not found.

Missing:
$missingText

Install NVIDIA CUDA Toolkit 12.x, add its bin directory to PATH, set CUDA_PATH, or pass:
  .\scripts\package-windows-portable.ps1 -Cuda -CudaRuntimeDir "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.x\bin"

Searched:
$searched
"@
    }

    return $copied
}

function Install-CudaRuntimeFromPython {
    Write-Step "Downloading CUDA runtime DLLs from NVIDIA PyPI wheels"
    if (Test-Path -LiteralPath $CudaRuntimeCacheDir) {
        $toolsDir = Resolve-Path (Join-Path $RepoRoot "..\tools") -ErrorAction SilentlyContinue
        $resolvedCache = Resolve-Path -LiteralPath $CudaRuntimeCacheDir -ErrorAction SilentlyContinue
        if ($toolsDir -and $resolvedCache -and -not $resolvedCache.Path.StartsWith($toolsDir.Path, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw "Refusing to remove CUDA runtime cache outside tools: $($resolvedCache.Path)"
        }
        Remove-Item -LiteralPath $CudaRuntimeCacheDir -Recurse -Force
    }
    New-Item -ItemType Directory -Path $CudaRuntimeCacheDir -Force | Out-Null
    $packages = @(
        "nvidia-cuda-runtime-cu12==12.4.127",
        "nvidia-cublas-cu12==12.4.5.8",
        "nvidia-cufft-cu12==11.2.1.3",
        "nvidia-cudnn-cu12==9.11.0.98"
    )
    & python -m pip install --upgrade --target $CudaRuntimeCacheDir @packages
    if ($LASTEXITCODE -ne 0) {
        throw "Failed to download CUDA runtime packages with pip."
    }
}

function Write-Launcher {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string[]]$Lines
    )

    $content = @(
        "@echo off",
        "setlocal",
        "cd /d ""%~dp0""",
        "set ""PATH=%CD%;%PATH%"""
    ) + $Lines + @(
        "endlocal"
    )

    Set-Content -LiteralPath $Path -Value $content -Encoding ASCII
}

Write-Step "Building release binary"
Initialize-LocalBuildEnvironment
$cargoArgs = @("build", "--release", "--package", "simple-runtime")
if ($Cuda) {
    $cargoArgs += @("--features", "cuda")
}
& cargo @cargoArgs
if ($LASTEXITCODE -ne 0) {
    throw "cargo build failed with exit code $LASTEXITCODE"
}

if (-not (Test-Path -LiteralPath $BinaryPath -PathType Leaf)) {
    throw "Expected binary was not found: $BinaryPath"
}

Write-Step "Preparing portable directory"
New-Item -ItemType Directory -Path $DistDir -Force | Out-Null
if (Test-Path -LiteralPath $PortableDir) {
    $resolvedDist = (Resolve-Path -LiteralPath $DistDir).Path
    $resolvedPortable = (Resolve-Path -LiteralPath $PortableDir).Path
    if (-not $resolvedPortable.StartsWith($resolvedDist, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Refusing to remove portable directory outside dist: $resolvedPortable"
    }

    Write-Step "Removing previous portable directory"
    Remove-Item -LiteralPath $PortableDir -Recurse -Force
}
New-Item -ItemType Directory -Path $PortableDir -Force | Out-Null

foreach ($dir in @("config", "models", "uploads", "results")) {
    New-Item -ItemType Directory -Path (Join-Path $PortableDir $dir) -Force | Out-Null
}

Copy-Item -LiteralPath $BinaryPath -Destination $PortableDir -Force

Write-Step "Collecting runtime DLLs from target/release"
$runtimeDllPatterns = @(
    "*.dll"
)
foreach ($pattern in $runtimeDllPatterns) {
    Get-ChildItem -LiteralPath $ReleaseDir -Filter $pattern -File -ErrorAction SilentlyContinue |
        Where-Object { -not $_.Name.StartsWith("._") } |
        ForEach-Object { Copy-Item -LiteralPath $_.FullName -Destination $PortableDir -Force }
}

Write-Step "Collecting OpenCV DLLs"
$openCvCopied = Copy-OpenCvDlls -Destination $PortableDir

$onnxDll = Join-Path $PortableDir "onnxruntime_providers_shared.dll"
if (-not (Test-Path -LiteralPath $onnxDll)) {
    Write-Warning "ONNX Runtime provider DLLs were not found in target/release. The portable build may need additional ONNX Runtime DLLs."
}

if ($Cuda) {
    $cudaDll = Join-Path $PortableDir "onnxruntime_providers_cuda.dll"
    if (-not (Test-Path -LiteralPath $cudaDll)) {
        throw "CUDA packaging was requested, but onnxruntime_providers_cuda.dll was not found in target/release."
    }
    if ($DownloadCudaRuntime) {
        Install-CudaRuntimeFromPython
    }
    Write-Step "Collecting CUDA runtime DLLs"
    $cudaRuntimeCopied = Copy-CudaRuntimeDlls -Destination $PortableDir
} else {
    $cudaRuntimeCopied = 0
}

Write-Step "Writing launchers"
Write-Launcher -Path (Join-Path $PortableDir "run-ui.bat") -Lines @(
    "powershell -NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -Command ""Start-Process -FilePath '%CD%\$BinaryName' -ArgumentList 'ui-webview' -WorkingDirectory '%CD%' -WindowStyle Hidden"""
)

Write-Launcher -Path (Join-Path $PortableDir "run-ui-debug.bat") -Lines @(
    """%CD%\$BinaryName"" -vv ui-webview",
    "pause"
)

Write-Launcher -Path (Join-Path $PortableDir "run-cli-example.bat") -Lines @(
    "if not exist ""uploads"" mkdir ""uploads""",
    "if not exist ""results"" mkdir ""results""",
    "echo Put input images in the uploads folder, then edit this example command if needed.",
    """%CD%\$BinaryName"" cli --input ""uploads"" --output ""results"" --config ""config\example.json"" --overwrite",
    "pause"
)

$readme = @"
Manga Image Translator Rust - Windows Portable
================================================

Contents:
- simple-runtime.exe
- Runtime DLLs copied from target\release
- OpenCV DLLs copied from OPENCV_BIN_DIR, OPENCV_DIR, OPENCV_LINK_PATHS, PATH, or common C:\tools\opencv paths when found
- CUDA runtime DLLs copied from CUDA_PATH, PATH, CUDA Toolkit 12.x, target\release, or -CudaRuntimeDir when this is a CUDA package
- config, models, uploads, and results directories

Launchers:
- run-ui.bat starts the WebView desktop UI.
- run-cli-example.bat runs the CLI against the uploads folder and writes to results.
  Add or edit config\example.json before using that example command.

Packaging notes:
- If OpenCV DLLs were not copied, install OpenCV and set OPENCV_BIN_DIR to the folder containing opencv_world*.dll.
- CUDA packages need compatible NVIDIA CUDA 12.x, cuDNN 9, and ONNX Runtime CUDA provider DLLs.
- If packaging fails with missing cublasLt64_12.dll/cublas64_12.dll/cufft64_11.dll/cudart64_12.dll, install CUDA Toolkit 12.x or pass -CudaRuntimeDir to the script.
- Use -DownloadCudaRuntime to fetch NVIDIA CUDA 12 runtime DLLs from PyPI into the local tools cache when CUDA Toolkit is not installed.
- Set MIT_REQUIRE_CUDA=1 before starting a launcher, or enable Require CUDA in WebView, to fail fast when CUDA is unavailable.
- All launchers cd to their own directory before starting simple-runtime.exe.
"@
Set-Content -LiteralPath (Join-Path $PortableDir "README-portable.txt") -Value $readme -Encoding ASCII

if (-not $NoZip) {
    Write-Step "Creating zip archive"
    $zipPath = Join-Path $DistDir "$PortableName.zip"
    if (Test-Path -LiteralPath $zipPath) {
        Remove-Item -LiteralPath $zipPath -Force
    }
    Compress-Archive -LiteralPath $PortableDir -DestinationPath $zipPath -Force
}

Write-Host ""
Write-Host "Portable package created: $PortableDir"
if (-not $NoZip) {
    Write-Host "Zip archive created: $(Join-Path $DistDir "$PortableName.zip")"
}
Write-Host "OpenCV DLLs copied: $openCvCopied"
if ($Cuda) {
    Write-Host "CUDA runtime DLLs copied: $cudaRuntimeCopied"
}
