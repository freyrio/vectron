@echo off
echo Vectron Embedder Examples Runner

if "%1"=="" (
    echo Please specify which example to run:
    echo   basic    - Basic window example
    echo   multiple - Multiple windows example
    echo.
    echo Example: run_examples.bat basic
    exit /b 1
)

if "%1"=="basic" (
    echo Running Basic Window Example...
    cargo run --example basic_window
) else if "%1"=="multiple" (
    echo Running Multiple Windows Example...
    cargo run --example multiple_windows
) else (
    echo Unknown example: %1
    echo Available examples: basic, multiple
    exit /b 1
) 