#!/bin/bash

# This script updates the Tuono version in package.json, Cargo.toml, and docker/app/compose.yml
# Usage: ./tuono-update.sh [new_version]
# If no version is provided, the latest version from crates.io is used.
# Use -h for help.

set -e  # Exit immediately if a command exits with a non-zero status

PACKAGE_JSON="./package.json"
CARGO_TOML="./Cargo.toml"
COMPOSE_YML="./docker/app/compose.yml"

if [ "$1" == "-h" ]; then
    echo "Usage: ./tuono-update.sh [new_version]"
    echo "If no version is provided, the latest version from crates.io will be used."
    exit 0
fi

# Determine the new version of tuono
if [ -z "$1" ]; then
    echo "Fetching the latest Tuono version..."
    NEW_VERSION=$(curl -s https://crates.io/api/v1/crates/tuono | grep -o '"max_version":"[^"]*"' | sed 's/"max_version":"//;s/"//')
else
    NEW_VERSION=$1
fi

# Check if the required files exist
FILES=($PACKAGE_JSON $CARGO_TOML $COMPOSE_YML)
for FILE in "${FILES[@]}"; do
    if [ ! -f "$FILE" ]; then
        echo "Error: File $FILE not found. Update aborted."
        exit 1
    fi
done

# Get the current tuono's version
CURRENT_VERSION=$(grep -o 'tuono_lib = "[^"]*"' $CARGO_TOML | sed 's/tuono_lib = "//;s/"//')

if [ -z "$CURRENT_VERSION" ]; then
    echo "Error: Could not determine the current Tuono version."
    exit 1
fi

echo "Current Tuono version: $CURRENT_VERSION"
echo "New Tuono version: $NEW_VERSION"

if [ "$CURRENT_VERSION" == "$NEW_VERSION" ]; then
    echo "Tuono is already up to date. No update needed."
    exit 0
fi

read -p "Are you sure you want to update to Tuono $NEW_VERSION? (y/N): " CONFIRM
if [[ "$CONFIRM" != "y" && "$CONFIRM" != "Y" ]]; then
    echo "Update cancelled."
    exit 0
fi

# Update files
sed -i 's/\("tuono"[[:space:]]*:[[:space:]]*"\)[0-9]\+\.[0-9]\+\.[0-9]\+\("\)/\1'"$NEW_VERSION"'\2/' $PACKAGE_JSON
sed -i "s/\(tuono_lib = \)\"[0-9]\+\.[0-9]\+\.[0-9]\+\"/\1\"$NEW_VERSION\"/" $CARGO_TOML
sed -i "s/\(TUONO_VERSION: \)[0-9]\+\.[0-9]\+\.[0-9]\+/\1$NEW_VERSION/" $COMPOSE_YML

# Success message
echo "Tuono successfully updated to version $NEW_VERSION."