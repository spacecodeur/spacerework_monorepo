#!/bin/bash

cd /app

# The `npm install` is added here because the `docker/app/compose.yml` volume mapping (`../..:/app`) mounts the host project directory to the container's `/app`.
# Since `node_modules` doesn't exist on the host, it would be removed from the container once the volume is mounted.
# Running `npm install` ensures the dependencies are installed inside the container after the volume mount, without overwriting them.
npm install

tail -f /dev/null # keep the container running after build