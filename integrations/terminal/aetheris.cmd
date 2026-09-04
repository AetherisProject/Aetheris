@echo off
REM Aetheris CLI wrapper for Windows
WHERE aeth >nul 2>nul
IF %ERRORLEVEL% NEQ 0 (
    ECHO aeth not found in PATH. Please install Aetheris first.
    EXIT /B 1
)
aeth %*
