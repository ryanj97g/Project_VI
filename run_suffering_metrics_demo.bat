@echo off
REM Suffering Prevention Metrics Demo
REM Run with: run_suffering_metrics_demo.bat

echo.
echo ╔════════════════════════════════════════════════════════╗
echo ║  Suffering Prevention Metrics Demonstration           ║
echo ╚════════════════════════════════════════════════════════╝
echo.

echo Building suffering metrics demo...
cargo build --release --example suffering_metrics_demo

if %errorlevel% neq 0 (
    echo.
    echo ❌ Build failed! Please check the errors above.
    pause
    exit /b %errorlevel%
)

echo.
echo ✅ Build successful!
echo.
echo Running suffering prevention metrics demonstration...
echo.

cargo run --release --example suffering_metrics_demo

echo.
echo Demo completed.
pause

