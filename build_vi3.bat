@echo off
REM VI3 Build Script - Compile entire project
REM Run with: build_vi3.bat

echo.
echo ╔════════════════════════════════════════════════════════╗
echo ║  VI3 Project Builder                                   ║
echo ╚════════════════════════════════════════════════════════╝
echo.

echo Building VI3 in release mode...
echo This may take a few minutes...
echo.

cargo build --release --all-targets

if %errorlevel% neq 0 (
    echo.
    echo ❌ Build failed! Please check the errors above.
    pause
    exit /b %errorlevel%
)

echo.
echo ╔════════════════════════════════════════════════════════╗
echo ║  ✅ BUILD SUCCESSFUL                                   ║
echo ╚════════════════════════════════════════════════════════╝
echo.
echo Binary location: target\release\vi3.exe
echo.
echo Available commands:
echo   - run_vi3.bat                    : Run main application
echo   - run_vi3_demo.bat               : Run VI3 demo
echo   - run_suffering_metrics_demo.bat : Run metrics demo
echo   - run_all_tests.bat              : Run test suite
echo.
pause

