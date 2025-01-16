#!/bin/bash

NETWORK_NAME="${APP_NAME}-docker-network"

if ! docker network ls | grep -q "$NETWORK_NAME"; then
  docker network create "$NETWORK_NAME"
fi

docker compose -f docker/db/compose.yml --env-file .env up --build -d