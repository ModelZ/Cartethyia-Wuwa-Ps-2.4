:: Author: ModelZ
:: Date: 13/04/2025
:: Description: Launches the Wicked Waifus servers in Windows Terminal.
:: Usage: Double-click this batch file to run it.
:: Requires: Windows Terminal installed and configured.

@echo off

:: Docker postgres volume cleanup and run 
docker compose down
docker volume rm wicked-waifus-ps_wicked-waifus-postgres-vol
docker-compose up -d

:: Sleep for 2 seconds to allow postgres to start
timeout /t 2 /nobreak >nul

:: Start the wuwa servers in Windows Terminal
start wt.exe ^
    new-tab cmd /k "cd /d ""%~dp0"" && target\release\wicked-waifus-config-server.exe" ^
    ; new-tab cmd /k "cd /d ""%~dp0"" && target\release\wicked-waifus-game-server.exe" ^
    ; new-tab cmd /k "cd /d ""%~dp0"" && target\release\wicked-waifus-gateway-server.exe" ^
    ; new-tab cmd /k "cd /d ""%~dp0"" && target\release\wicked-waifus-hotpatch-server.exe" ^
    ; new-tab cmd /k "cd /d ""%~dp0"" && target\release\wicked-waifus-login-server.exe"

:: Sleep for 2 seconds to allow postgres to start
timeout /t 2 /nobreak >nul

:: Start the wuwa client in Windows Terminal
start wt new-tab cmd /k "cd /d D:\wuwa_beta\2.4\wuwa-client\Client\Binaries\Win64 && launcher.exe -l info"


