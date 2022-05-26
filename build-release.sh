#!/bin/sh
cargo build --profile=release -Z build-std=core,compiler_builtins
