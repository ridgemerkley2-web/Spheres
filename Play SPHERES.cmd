@echo off
title SPHERES
cd /d "%~dp0"

rem Was: "%~dp0dist\spheres-web.exe" --port 7777
rem `dist/` is gitignored and nothing in the repo ever builds it, so this file --
rem the one thing a player is meant to double-click -- worked on no branch.

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
rem Opened only after the build, so the browser does not land on nothing.
start "" http://127.0.0.1:7777
cargo run --release -p spheres-web -- --port 7777
