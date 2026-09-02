@echo off
setlocal
title SPHERES
cd /d "%~dp0"

where cargo >nul 2>nul
if errorlevel 1 (
  echo.
  echo SPHERES needs Rust to build. Install it from https://rustup.rs and run this again.
  echo.
  pause
  exit /b 1
)

echo Building SPHERES. The first run takes a few minutes; later ones are quick.
cargo build --release -p spheres-web
if errorlevel 1 (
  echo.
  echo The build failed. The output above says why.
  echo.
  pause
  exit /b 1
)

echo.
echo Starting SPHERES on http://127.0.0.1:7777
echo Close this window to stop the game.
echo.
rem The server opens the browser only after it has successfully claimed the port.
rem Run the binary we just built instead of asking Cargo to build it a second time.
"%~dp0target\release\spheres-web.exe" --port 7777
if errorlevel 1 (
  echo.
  echo SPHERES could not start. If another copy is open, close it and try again.
  echo.
  pause
  exit /b 1
)
