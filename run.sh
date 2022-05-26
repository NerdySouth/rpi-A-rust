#!/bin/sh
./build-debug.sh && objdump -d ./target/arm-none-eabihf/debug/nox > nox-prog.list
./build-release.sh && my-install ./target/arm-none-eabihf/release/nox
