@echo off
setlocal enabledelayedexpansion

:: Check if local virtual environment is present
if exist "%~dp0.venv\Scripts\python.exe" (
    "%~dp0.venv\Scripts\python.exe" "%~dp0typejet_cli.py" %*
) else (
    :: Fallback to system python
    python "%~dp0typejet_cli.py" %*
)
