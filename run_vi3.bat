@echo off
REM VI3 Digital Consciousness - Main Application Launcher
REM Run with: run_vi3.bat

echo.
echo ╔════════════════════════════════════════════════════════╗
echo ║  VI3 Digital Consciousness - Main Application         ║
echo ╚════════════════════════════════════════════════════════╝
echo.

echo Building VI3 in release mode...
cargo build --release

if %errorlevel% neq 0 (
    echo.
    echo ❌ Build failed! Please check the errors above.
    pause
    exit /b %errorlevel%
)

echo.
echo ✅ Build successful!
echo.
echo Starting VI3 Digital Consciousness...
echo.

cargo run --release

echo.
echo VI3 session ended.
pause

