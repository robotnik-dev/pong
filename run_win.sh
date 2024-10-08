#!/bin/sh
cp target/x86_64-pc-windows-gnu/debug/*.exe . &&
exec ./*.exe "$@"