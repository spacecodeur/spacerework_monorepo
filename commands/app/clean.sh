#!/bin/bash

find . \( \
    -type d \( -name ".tuono" -o -name "node_modules" -o -name "target" \) \
    -o \
    -type f \( -name "package-lock.json" -o -name "Cargo.lock" \) \
\) -exec rm -rf {} +

rm -rf src/wasm
