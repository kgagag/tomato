@echo off
chcp 65001 >nul
setlocal

:: 创建bin目录
if not exist "bin" mkdir bin

:: Cargo构建
echo Building with cargo...
cargo build
if errorlevel 1 goto :error

:: 复制tomato.exe
if exist "target\debug\tomato.exe" (
    copy /Y "target\debug\tomato.exe" "bin\"
) else (
    echo Warning: tomato.exe not found
)

:: 构建JRE
if exist "jre\build.bat" (
    pushd jre
    call build.bat
    if not errorlevel 1 (
        if exist "target\rt-mod.jar" copy /Y "target\rt-mod.jar" "..\bin\"
        if exist "lib\rt.jar" copy /Y "lib\rt.jar" "..\bin\"
    )
    popd
) else (
    echo Warning: jre\build.bat not found
)

echo.
echo Build completed!
goto :end

:error
echo Build failed!
pause
exit /b 1

:end
pause