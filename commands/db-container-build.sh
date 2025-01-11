#!/bin/bash

docker compose -f docker/db/compose.yml --env-file .env up --build -d