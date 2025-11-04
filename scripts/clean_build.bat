@echo off
REM Clean VI3 Build Artifacts
REM Run with: clean_build.bat

echo.
echo ╔════════════════════════════════════════════════════════╗
echo ║  VI3 Clean Build                                       ║
echo ╚════════════════════════════════════════════════════════╝
echo.

echo Cleaning all build artifacts...
echo.

cargo clean

if %errorlevel% neq 0 (
    echo.
    echo ❌ Clean failed!
    pause
    exit /b %errorlevel%
)

echo.
echo ✅ Build artifacts cleaned successfully!
echo.
echo Run build_vi3.bat to rebuild the project.
echo.
pause

