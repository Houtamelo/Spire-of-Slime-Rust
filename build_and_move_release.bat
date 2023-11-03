START /B /wait cargo build --release

echo off

timeout /t 1

echo on

copy /Y /B "%~dp0target\release\spire.dll" "%~dp0godot\Bin\spire.dll"