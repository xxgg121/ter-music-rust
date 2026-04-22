@echo off
chcp 65001 >nul
echo.
echo ========================================
echo   Ter-Music Windows 构建脚本
echo ========================================
echo.

REM 检查 Rust 是否安装
rustc --version >nul 2>&1
if errorlevel 1 (
    echo [错误] 未检测到 Rust 安装
    echo.
    echo 请先安装 Rust:
    echo   1. 访问 https://rustup.rs/
    echo   2. 下载并运行 rustup-init.exe
    echo   3. 或使用 winget: winget install Rustlang.Rustup
    echo.
    pause
    exit /b 1
)

REM 显示 Rust 版本
echo [信息] 检测到 Rust 版本:
rustc --version
cargo --version
echo.

REM 构建项目
echo [步骤] 开始构建项目...
echo.

REM 清理旧的构建文件
if exist "target\release\ter-music-rust.exe" (
    echo [信息] 发现旧的构建文件，正在清理...
    cargo clean
)

REM 构建发布版本
echo [信息] 正在编译发布版本（这可能需要几分钟）...
cargo build --release

if errorlevel 1 (
    echo.
    echo [错误] 编译失败！
    echo.
    echo 可能的原因:
    echo   1. 网络连接问题（Cargo 需要下载依赖）
    echo   2. Rust 版本过低（需要 1.70 或更高）
    echo   3. 代码编译错误
    echo.
    echo 解决方法:
    echo   1. 检查网络连接
    echo   2. 运行: rustup update
    echo   3. 查看上方错误信息
    echo.
    pause
    exit /b 1
)

echo.
echo ========================================
echo   构建成功！
echo ========================================
echo.

REM 显示可执行文件位置
echo [信息] 可执行文件位置:
echo   %CD%\target\release\ter-music-rust.exe
echo.

REM 询问是否运行
set /p RUN="是否立即运行？(y/n): "
if /i "%RUN%"=="y" (
    echo.
    echo [信息] 启动程序...
    echo.
    target\release\ter-music-rust.exe
)

pause
