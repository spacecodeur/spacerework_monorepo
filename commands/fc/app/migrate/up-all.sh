#!/bin/bash

sea-orm-cli migrate -d src/migration up && \
echo -e "\n\033[1;34mRegenerating models...\n\033[0m" && \
./commands.sh fc/app/migrate/generate-all-database-entities