#!/bin/sh
./build-debug.sh && objdump -d ./target/arm-none-eabihf/debug/nox > dump-files/nox-prog.list
./build-release.sh && rust_install ./target/arm-none-eabihf/release/nox
