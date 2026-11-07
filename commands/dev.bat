@echo off
cd /d "%~dp0.."
rem dx serve を起動しつつ、styles\main.scss を一定間隔で「保存」(タイムスタンプ更新) する
rem 使い方: dev.bat [dx serve の引数...]
rem         set INTERVAL=3 && dev.bat   (間隔を秒で変更。デフォルト100秒)

set "FILE=styles\main.scss"
if "%INTERVAL%"=="" set "INTERVAL=20"

start "scss-touch" /min powershell -NoProfile -Command "$Host.UI.RawUI.WindowTitle='scss-touch'; while ($true) { Start-Sleep %INTERVAL%; (Get-Item '%FILE%').LastWriteTime = Get-Date }"

dx serve %*

taskkill /fi "WINDOWTITLE eq scss-touch*" /f >nul 2>&1