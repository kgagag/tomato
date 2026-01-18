@echo off
chcp 65001 >nul
title RT-MOD 构建工具
echo ========================================
echo    RT-MOD 项目构建工具
echo ========================================
echo.

echo 1. 清理并构建项目...
call mvn clean package

echo.
echo 2. 处理依赖库...
if exist "lib" (
    if not exist "target\lib" mkdir target\lib
    echo 正在复制 lib 目录到 target...
    xcopy /E /Y lib target\lib\
    echo lib 目录复制完成！
) else (
    echo 警告：未找到 lib 目录，跳过依赖库复制
)

echo.
echo ========================================
echo 构建成功完成！
echo.
echo 输出文件：target\rt-mod.jar
echo 依赖目录：target\lib\
echo.
echo 按任意键退出...
pause >nul