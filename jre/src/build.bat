@echo off
setlocal enabledelayedexpansion

:: 配置参数
set "SOURCE_DIR=src"
set "TARGET_DIR=bin"  :: 编译输出目录
set "CLASSPATH=."     :: 类路径（可添加其他依赖）

:: 创建输出目录
if not exist "%TARGET_DIR%" (
    mkdir "%TARGET_DIR%"
)

:: 查找所有java文件并编译
set "fileCount=0"
for /r "%SOURCE_DIR%" %%f in (*.java) do (
    set /a "fileCount+=1"
    echo 正在编译 %%f...
    
    :: 提取包路径（相对于src目录）
    set "filePath=%%~pf"
    set "filePath=!filePath:%SOURCE_DIR%=!"
    set "filePath=!filePath:~0,-1!"
    
    :: 创建对应的包目录
    if not "!filePath!"=="" (
        if not exist "%TARGET_DIR%!filePath!" (
            mkdir "%TARGET_DIR%!filePath!"
        )
    )
    
    :: 编译单个文件
    javac -encoding UTF-8 -d "%TARGET_DIR%" -cp "%CLASSPATH%" "%%f"
    
    if errorlevel 1 (
        echo 编译失败: %%f
        exit /b 1
    )
)

if %fileCount% gtr 0 (
    echo 成功编译 %fileCount% 个Java文件到 %TARGET_DIR%
) else (
    echo 错误: %SOURCE_DIR% 目录下未找到任何.java文件
    exit /b 1
)

endlocal