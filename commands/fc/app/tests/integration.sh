#!/bin/bash

if [ -n "$1" ]; then
    if [[ "$1" != integration::* ]]; then
        echo "Error: If a parameter is provided, it must start with 'integration::'."
        exit 1
    else
        cargo test --test mod "$1"
    fi
else
    cargo test --test mod integration
fi
