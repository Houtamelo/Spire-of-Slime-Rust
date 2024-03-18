START /B /wait cargo build

echo off

timeout /t 1

echo on

copy /Y /B "%~dp0target\debug\spire.dll" "%~dp0godot\Bin\spire.dll"

pause
