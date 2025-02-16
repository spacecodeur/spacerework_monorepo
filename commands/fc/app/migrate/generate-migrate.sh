#!/bin/bash

show_help() {
    echo "Usage: $0 <migration>"
    echo "  Migration           : the migration's name"
}

if [[ $# -lt 1 ]]; then
    echo "Error: One required parameter is missing."
    show_help
    exit 1
fi

MIGRATION_NAME=$1

sea-orm-cli migrate -d src/migration generate "$MIGRATION_NAME"