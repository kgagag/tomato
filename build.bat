@echo off
chcp 65001 >nul
setlocal

:: Create bin directory if not exists
if not exist "bin" (
    echo Creating bin directory...
    mkdir bin
)

:: Build Rust project with Cargo
echo Building with cargo...
cargo build
if errorlevel 1 goto :error

:: Copy compiled tomato.exe to bin directory
if exist "target\debug\tomato.exe" (
    echo Copying tomato.exe to bin directory...
    copy /Y "target\debug\tomato.exe" "bin\"
) else (
    echo Warning: tomato.exe not found in target\debug
)

:: Build JRE and copy related jar files if build.bat exists
if exist "jre\build.bat" (
    echo Building JRE environment...
    pushd jre
    call build.bat
    :: Copy jar files only if JRE build succeeds
    if %errorlevel% equ 0 (
        if exist "target\rt-mod.jar" (
            echo Copying rt-mod.jar to bin directory...
            copy /Y "target\rt-mod.jar" "..\bin\"
        )
        if exist "lib\rt.jar" (
            echo Copying rt.jar to bin directory...
            copy /Y "lib\rt.jar" "..\bin\"
        )
    ) else (
        echo Warning: JRE build failed - build.bat execution error
    )
    popd
) else (
    echo Warning: jre\build.bat not found in root directory
)

echo.
echo ==============================
echo Build completed successfully!
echo ==============================
goto :end

:error
echo.
echo ==============================
echo Build failed! Please check the error information above.
echo ==============================
pause
exit /b 1

:end
pause
exit /b 0