@echo off
chcp 65001 >nul
title RT-MOD Build Tool
echo ========================================
echo    RT-MOD Project Build Tool
echo ========================================
echo.

echo 1. Cleaning and building the project...
call mvn clean package

echo.
echo 2. Processing dependency libraries...
if exist "lib" (
    if not exist "target\lib" mkdir target\lib
    echo Copying lib directory to target...
    xcopy /E /Y lib target\lib\
    echo lib directory copied successfully!
) else (
    echo Warning: lib directory not found, skipping dependency copy.
)

echo.
echo ========================================
echo Build completed successfully!
echo.
echo Output file: target\rt-mod.jar
echo Dependencies directory: target\lib\
echo.