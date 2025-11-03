@echo off
REM VI3 Demo - Complete Architecture Demonstration
REM Run with: run_vi3_demo.bat

echo.
echo ╔════════════════════════════════════════════════════════╗
echo ║  PROJECT VI V3 - Architecture Demonstration           ║
echo ╚════════════════════════════════════════════════════════╝
echo.

echo Building VI3 demo...
cargo build --release --example vi3_demo

if %errorlevel% neq 0 (
    echo.
    echo ❌ Build failed! Please check the errors above.
    pause
    exit /b %errorlevel%
)

echo.
echo ✅ Build successful!
echo.
echo Running VI3 architecture demonstration...
echo.

cargo run --release --example vi3_demo

echo.
echo Demo completed.
pause

