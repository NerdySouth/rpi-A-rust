#!/bin/sh
./build-debug.sh && objdump -d ./target/arm-none-eabihf/debug/nox > dump-files/nox-kernel.list && \
./build-release.sh && mv target/arm-none-eabihf/release/nox firmware/kernel.img && cp firmware/kernel.img /Volumes/NO\ NAME/ &&\
sync && diskutil eject /Volumes/NO\ NAME
