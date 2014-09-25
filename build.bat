@echo off
rustc --version

::clean
rm -rf bin
mkdir bin

::compile
cd src/Mi

:: Release ->
rustc main.rs -O -o ../../bin/Mi.exe
:: Debig ->
::rustc main.rs -Z debug-info -o ../../bin/Mi.exe

::wait / handle errors
pause