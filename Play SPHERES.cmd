@echo off
title SPHERES
echo Starting SPHERES on http://127.0.0.1:7777 ...
echo Close this window to stop the game.
start "" http://127.0.0.1:7777
"%~dp0dist\spheres-web.exe" --port 7777
