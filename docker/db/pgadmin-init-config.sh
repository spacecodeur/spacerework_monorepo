#!/bin/bash

# Charger les variables du fichier .env
set -a
source .env
set +a

# Générer le fichier servers.json
cat <<EOF > servers.json
{
  "Servers": {
    "1": {
      "Name": "${APP_NAME}-db",
      "Group": "Servers",
      "Host": "postgres",
      "Port": 5432,
      "Username": "${POSTGRES_USER}",
      "Password": "${POSTGRES_PW}",
      "SSLMode": "prefer",
      "MaintenanceDB": "${POSTGRES_DB}"
    }
  }
}
EOF
