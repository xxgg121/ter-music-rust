@echo off
chcp 65001 >nul 2>&1
echo ==========================================
echo Ter-Music-Rust MacOS 交叉编译脚本
echo ==========================================
echo.

:::: 将 zig 添加到 PATH（根据实际安装路径修改）
set PATH=%USERPROFILE%\AppData\Local\Packages\PythonSoftwareFoundation.Python.3.13_qbz5n2kfra8p0\LocalCache\local-packages\Python313\site-packages\ziglang;%PATH%

:::: 项目目录（使用正斜杠避免 bindgen 转义问题）
set PROJECT_DIR=%~dp0
set PROJECT_DIR_FWD=%PROJECT_DIR:\=/%
set MACOS_SYSROOT=%PROJECT_DIR%macos-sysroot
set MACOS_SYSROOT_FWD=%PROJECT_DIR_FWD%macos-sysroot

cd /d "%PROJECT_DIR%"

:::: 确认 zig 和 cargo-zigbuild 可用
echo [1/3] 检查 zig 和 cargo-zigbuild ...
zig version
if errorlevel 1 (
    echo 错误: zig 未找到
    pause
    exit /b 1
)
cargo zigbuild --help >nul 2>&1
if errorlevel 1 (
    echo 错误: cargo-zigbuild 未找到
    echo 请先安装: cargo install cargo-zigbuild
    pause
    exit /b 1
)
echo.

:::: 确认 MacOS SDK 存在
echo [2/3] 检查 MacOS SDK ...
if not exist "%MACOS_SYSROOT%\System\Library\Frameworks\CoreAudio.framework" (
    echo 错误: 未找到 MacOS SDK
    echo.
    echo 交叉编译 MacOS 版本需要 MacOS SDK 头文件，请按以下步骤准备:
    echo.
    echo   方式 A: 从 GitHub 下载预打包 SDK（推荐）:
    echo     下载: https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
    echo     加速: https://ghfast.top/https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
    echo     curl -L -o MacOSX13.3.sdk.tar.xz https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
    echo     mkdir macos-sysroot
    echo     tar -xf MacOSX13.3.sdk.tar.xz -C macos-sysroot --strip-components=1
    echo     del MacOSX13.3.sdk.tar.xz
    echo.
    echo   方式 B: 从 MacOS 系统复制:
    echo     scp -r mac:/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk ./macos-sysroot
    echo.
    echo   SDK 来源: https://github.com/joseluisq/macosx-sdks
    echo.
    pause
    exit /b 1
)
echo   MacOS SDK 已就绪
echo.

:::: 配置环境变量
echo [3/3] 配置交叉编译环境并编译...

:::: 添加 MacOS 目标
rustup target add x86_64-apple-darwin aarch64-apple-darwin 2>nul

:::: 设置 libclang 路径（bindgen 需要）
if exist "C:\msys64\mingw64\bin\libclang.dll" (
    set LIBCLANG_PATH=C:\msys64\mingw64\bin
) else if exist "C:\Program Files\LLVM\bin\libclang.dll" (
    set LIBCLANG_PATH=C:\Program Files\LLVM\bin
) else (
    echo 警告: 未找到 libclang.dll，bindgen 可能失败
    echo   请安装 LLVM: winget install LLVM.LLVM
    echo   或通过 MSYS2: pacman -S mingw-w64-x86_64-clang
)

:::: 设置 MacOS SDK 路径
:::: 注意: 使用正斜杠路径，bindgen 会将反斜杠当作转义字符
set COREAUDIO_SDK_PATH=%MACOS_SYSROOT_FWD%
:::: SDKROOT: zig 链接器需要此变量找到 MacOS 系统库（libobjc.tbd 等）
set SDKROOT=%MACOS_SYSROOT%

:::: 编译 x86_64 版本（Intel Mac）
echo.
echo ------------------------------------------
echo 编译 x86_64 版本 (Intel Mac) ...
echo ------------------------------------------
set BINDGEN_EXTRA_CLANG_ARGS=--target=x86_64-apple-darwin -isysroot %MACOS_SYSROOT_FWD% -F %MACOS_SYSROOT_FWD%/System/Library/Frameworks -iframework %MACOS_SYSROOT_FWD%/System/Library/Frameworks -I %MACOS_SYSROOT_FWD%/usr/include
cargo zigbuild --release --target x86_64-apple-darwin

if errorlevel 1 (
    echo.
    echo ==========================================
    echo 编译失败！
    echo ==========================================
    echo.
    echo 常见问题:
    echo   1. libclang.dll 未找到 - 安装 LLVM 或 MSYS2 clang
    echo   2. MacOS SDK 头文件缺失 - 检查 macos-sysroot 目录
    echo   3. CoreAudio.framework 未找到 - 确保复制了完整 Frameworks
    echo.
    pause
    exit /b 1
)

echo.
echo ==========================================
echo 编译成功！
echo ==========================================
echo.
echo 输出文件位置:
echo   %PROJECT_DIR%target\x86_64-apple-darwin\release\ter-music-rust
echo.

:::: 编译 aarch64 版本（Apple Silicon Mac）
echo ------------------------------------------
echo 编译 aarch64 版本 (Apple Silicon Mac) ...
echo ------------------------------------------
set BINDGEN_EXTRA_CLANG_ARGS=--target=aarch64-apple-darwin -isysroot %MACOS_SYSROOT_FWD% -F %MACOS_SYSROOT_FWD%/System/Library/Frameworks -iframework %MACOS_SYSROOT_FWD%/System/Library/Frameworks -I %MACOS_SYSROOT_FWD%/usr/include
cargo zigbuild --release --target aarch64-apple-darwin

if errorlevel 1 (
    echo.
    echo ==========================================
    echo 编译失败！
    echo ==========================================
    pause
    exit /b 1
)

echo.
echo ==========================================
echo 编译成功！
echo ==========================================
echo.
echo 输出文件位置:
echo   %PROJECT_DIR%target\aarch64-apple-darwin\release\ter-music-rust
echo.

:::: 显示文件信息
dir "target\x86_64-apple-darwin\release\ter-music-rust" 2>nul
dir "target\aarch64-apple-darwin\release\ter-music-rust" 2>nul

echo.
echo 将文件复制到 MacOS 系统后，需要:
echo   1. chmod +x ter-music-rust
echo   2. MacOS 可能需要允许运行未知来源应用:
echo      系统设置 -^> 隐私与安全性 -^> 仍要打开
echo      或: xattr -cr ter-music-rust
echo   3. ./ter-music-rust -o /path/to/music
echo.
echo 说明:
echo   - x86_64 版本适用于 Intel Mac
echo   - aarch64 版本适用于 Apple Silicon (M1/M2/M3/M4) Mac
echo   - MacOS 无需额外安装音频库（使用系统 CoreAudio）
echo.
pause
