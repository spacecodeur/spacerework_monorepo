#!/bin/bash

cd crates/md_to_html && \
wasm-pack build --target web --out-dir ../../src/wasm && \
cd ../..