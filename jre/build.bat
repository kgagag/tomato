@echo off
chcp 65001 >nul  :: Force UTF-8 encoding to prevent charset issues
setlocal enabledelayedexpansion

:: Configuration
set "SOURCE_DIR=src"
set "TARGET_DIR=out"
set "CLASSPATH=."  :: Add additional dependencies here if needed

:: Verify source directory exists
if not exist "%SOURCE_DIR%" (
    echo Error: Source directory %SOURCE_DIR% not found
    exit /b 1
)

:: Create output directory if it doesn't exist
if not exist "%TARGET_DIR%" (
    mkdir "%TARGET_DIR%"
)

:: Compile all Java files (automatically handles package structure)
set "fileCount=0"
for /r "%SOURCE_DIR%" %%f in (*.java) do (
    set /a "fileCount+=1"
    echo Compiling: %%~nxf
    
    :: Extract package path (relative to src directory)
    set "filePath=%%~pf"
    set "filePath=!filePath:%SOURCE_DIR%=!"
    set "filePath=!filePath:~0,-1!"
    
    :: Create corresponding package directories
    if not "!filePath!"=="" (
        if not exist "%TARGET_DIR%!filePath!" (
            mkdir "%TARGET_DIR%!filePath!"
        )
    )
    
    :: Compile the file
    javac -encoding UTF-8 -d "%TARGET_DIR%" -cp "%CLASSPATH%" "%%f"
    if errorlevel 1 (
        echo Compilation failed for: %%f
        exit /b 1
    )
)

:: Output results
if %fileCount% equ 0 (
    echo Error: No .java files found. Please check:
    echo 1. Files are placed under %SOURCE_DIR% directory
    echo 2. Files have .java extension (not .txt or others)
    exit /b 1
)

echo Compilation successful! Processed