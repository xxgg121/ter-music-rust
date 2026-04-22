@echo off
chcp 65001 >nul 2>&1
echo ==========================================
echo Ter-Music-Rust Linux 交叉编译脚本
echo ==========================================
echo.

::: 将 zig 添加到 PATH
set PATH=%USERPROFILE%\AppData\Local\Packages\PythonSoftwareFoundation.Python.3.13_qbz5n2kfra8p0\LocalCache\local-packages\Python313\site-packages\ziglang;%PATH%

::: 项目目录
set PROJECT_DIR=%~dp0
set SYSROOT=%PROJECT_DIR%linux-sysroot

cd /d "%PROJECT_DIR%"

::: 确认 zig 和 cargo-zigbuild 可用
echo [1/2] 检查 zig 和 cargo-zigbuild ...
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

::: 设置交叉编译环境变量
echo [2/2] 配置交叉编译环境并编译...

::: ALSA 的 pkg-config 配置
set PKG_CONFIG_PATH=%SYSROOT%\usr\lib\x86_64-linux-gnu\pkgconfig
set PKG_CONFIG_ALLOW_CROSS=1
set PKG_CONFIG_SYSROOT_DIR=%SYSROOT%

::: ALSA 库路径（让链接器找到 libasound.so）
set ALSA_LIB_DIR=%SYSROOT%\usr\lib\x86_64-linux-gnu
set ALSA_INC_DIR=%SYSROOT%\usr\include

::: 设置额外的库搜索路径
set RUSTFLAGS=-L %SYSROOT%\usr\lib\x86_64-linux-gnu

cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.34

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
echo   %PROJECT_DIR%target\x86_64-unknown-linux-gnu\release\ter-music-rust
echo.

::: 显示文件信息
dir "target\x86_64-unknown-linux-gnu\release\ter-music-rust" 2>nul

echo.
echo 将此文件复制到 Linux 系统后，需要:
echo   1. chmod +x ter-music-rust
echo   2. 确保系统已安装 ALSA 库 (sudo apt install libasound2)
echo   3. ./ter-music-rust -o /path/to/music
echo.
pause
