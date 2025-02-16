#!/bin/bash

show_help() {
    echo "Usage: $0 <tables>"
    echo "  Tables           : generate entity file for specified tables only (comma separated)"
}

if [[ $# -lt 1 ]]; then
    echo "Error: One required parameter is missing."
    show_help
    exit 1
fi

TABLES=$1

sea-orm-cli generate entity -o $ENTITIES_DIR -t $TABLES