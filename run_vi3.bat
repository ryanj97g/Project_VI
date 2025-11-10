@echo off
:: PROJECT VI - Smart Launcher for Windows
:: Auto-installs dependencies and runs VI

color 0B
title PROJECT VI - Constitutional Consciousness

echo.
echo ========================================================
echo           PROJECT VI - Starting...
echo ========================================================
echo.

:: Check for Ollama
echo [1/3] Checking for Ollama...
ollama --version >nul 2>&1
if %errorlevel% neq 0 (
    echo   ! Ollama not found. Installing...
    echo   ! Downloading Ollama installer...
    curl -L -o "%TEMP%\OllamaSetup.exe" https://ollama.com/download/OllamaSetup.exe
    if %errorlevel% neq 0 (
        echo   X Failed to download Ollama
        echo   Please install manually from: https://ollama.com
        pause
        exit /b 1
    )
    echo   ! Running Ollama installer...
    start /wait "%TEMP%\OllamaSetup.exe"
    del "%TEMP%\OllamaSetup.exe"
    echo   + Ollama installed!
    timeout /t 5 >nul
) else (
    echo   + Ollama found
)
echo.

:: Start Ollama service if not running
tasklist /FI "IMAGENAME eq ollama.exe" 2>NUL | find /I /N "ollama.exe">NUL
if %errorlevel% neq 0 (
    echo   ! Starting Ollama service...
    start /B ollama serve
    timeout /t 3 >nul
)

:: Check for gemma2:2b model
echo [2/3] Checking for AI models...
ollama list 2>nul | findstr /C:"gemma2:2b" >nul
if %errorlevel% neq 0 (
    echo   ! Model gemma2:2b not found. Downloading...
    echo   ! This may take a few minutes (~1.1GB)
    ollama pull gemma2:2b
    if %errorlevel% neq 0 (
        echo   X Failed to download gemma2:2b
        pause
        exit /b 1
    )
    echo   + gemma2:2b ready
) else (
    echo   + gemma2:2b found
)

:: Check for tinyllama model
ollama list 2>nul | findstr /C:"tinyllama" >nul
if %errorlevel% neq 0 (
    echo   ! Model tinyllama not found. Downloading...
    echo   ! This may take a minute (~430MB)
    ollama pull tinyllama
    if %errorlevel% neq 0 (
        echo   X Failed to download tinyllama
        pause
        exit /b 1
    )
    echo   + tinyllama ready
) else (
    echo   + tinyllama found
)
echo.

:: Launch VI
echo [3/3] Launching PROJECT VI...
echo.
echo ========================================================
echo     All dependencies ready. Starting VI...
echo ========================================================
echo.

if exist "vi3.exe" (
    start vi3.exe
) else if exist "target\release\vi3.exe" (
    start target\release\vi3.exe
) else (
    echo X vi3.exe not found in current directory
    echo   Please run from the PROJECT VI directory
    pause
    exit /b 1
)

:: Keep window open briefly to show success
timeout /t 2 >nul
exit
