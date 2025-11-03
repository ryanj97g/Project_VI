@echo off
REM VI3 Test Suite Runner
REM Run with: run_all_tests.bat

echo.
echo ╔════════════════════════════════════════════════════════╗
echo ║  VI3 Complete Test Suite                              ║
echo ╚════════════════════════════════════════════════════════╝
echo.

echo Running all tests...
echo.

cargo test --release

if %errorlevel% neq 0 (
    echo.
    echo ❌ Some tests failed! Please check the output above.
    pause
    exit /b %errorlevel%
)

echo.
echo ✅ All tests passed!
echo.
pause

