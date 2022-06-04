#!/bin/sh
./build-debug.sh && objdump -d ./target/arm-none-eabihf/debug/nox > dump-files/nox-prog.list && \
cargo build --profile=release -Z build-std=core,compiler_builtins
