#!/bin/bash

npm install && \
./commands.sh fc/app/wasm-build && \
tuono dev # todo : replace it by another command who only build the rust app