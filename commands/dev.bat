@echo off
cd /d "%~dp0.."
rem dx serve を起動しつつ、styles\main.scss を一定間隔で「保存」(タイムスタンプ更新) する
rem 使い方: dev.bat [dx serve の引数...]
rem         set INTERVAL=3 && dev.bat   (間隔を秒で変更。デフォルト100秒)

set "FILE=styles\main.scss"
if "%INTERVAL%"=="" set "INTERVAL=100"

start "scss-touch" /min cmd /c "for /l %%i in (0,0,1) do (timeout /t %INTERVAL% /nobreak >nul & copy /b %FILE% +,, >nul)"

dx serve %*

taskkill /fi "WINDOWTITLE eq scss-touch*" /f >nul 2>&1