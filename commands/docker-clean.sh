#!/bin/bash

docker ps -q | \
xargs -r docker stop && docker ps -aq | \
xargs -r docker rm -v && docker network prune -f && docker images -q | \
xargs -r docker rmi