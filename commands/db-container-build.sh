#!/bin/bash

NETWORK_NAME="${APP_NAME}-docker-network"

! docker network ls | grep -q "$NETWORK_NAME" && docker network create "$NETWORK_NAME"

[ -f docker/db/servers.json ] && rm docker/db/servers.json

# [ -f docker/db/.pgpass ] && rm docker/db/.pgpass

cat > docker/db/servers.json <<EOL
{
  "Servers": {
    "1": {
      "Name": "${APP_NAME}-db",
      "Group": "Servers",
      "Host": "postgres",
      "Port": 5432,
      "MaintenanceDB": "${POSTGRES_DB}",
      "Username": "${POSTGRES_USER}",
      "ConnectionParameters": {
        "sslmode": "prefer",
        "connect_timeout": 10,
        "sslcompression": 0,
        "passfile": "/tmp/pgpass"
      }
    }
  }
}
EOL

# echo "Création du fichier docker/db/pgpass..."
# #       host:     port:db_name: user_name:password
# # ex :  localhost:5432:postgres:myadmin:  Str0ngP@ssw0rd
# cat > docker/db/.pgpass <<EOL
# postgres:5432:${POSTGRES_DB}:${POSTGRES_USER}:${POSTGRES_PW}
# EOL
# chmod 600 docker/db/pgpass

docker compose -f docker/db/compose.yml up --build -d