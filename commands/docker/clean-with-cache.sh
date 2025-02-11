#!/bin/bash

docker ps -q | \
xargs -r docker stop && docker ps -aq | \
xargs -r docker rm -v && docker network prune -f && docker system prune -af --volumes && docker images -q | \
xargs -r docker rmi
docker network prune -f