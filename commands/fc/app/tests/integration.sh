#!/bin/bash

if [ -n "$1" ]; then
    cargo test --test integration -- "$1"
else
    cargo test --test integration
fi
