#!/bin/bash

sea-orm-cli migrate -d src/migration down -n 1 && \
sea-orm-cli migrate -d src/migration up -n 1