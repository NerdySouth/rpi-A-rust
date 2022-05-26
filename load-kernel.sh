#!/bin/sh
./build-debug.sh && objdump -d ./target/arm-none-eabihf/debug/nox > nox-kernel.list
./build-release.sh && mv target/arm-none-eabihf/release/nox kernel.img
cp kernel.img /Volumes/NO\ NAME
sync
diskutil eject /Volumes/NO\ NAME
