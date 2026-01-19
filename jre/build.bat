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
echo ========================================
echo Build completed successfully!
echo.
echo Output file: target\rt-mod.jar
echo Dependencies directory: target\lib\
echo.