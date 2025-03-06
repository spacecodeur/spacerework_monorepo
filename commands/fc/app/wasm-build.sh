#!/bin/bash

cd crates/space_md_to_html && \
wasm-pack build --target web --out-dir ../../src/wasm && \
cd ../..