#!/bin/bash

echo -e "\n\033[1;34mIMAGES\033[0m"
docker images

echo -e "\n\033[1;34mCONTAINERS\033[0m"
docker ps -a --size

echo -e "\n\033[1;34mNETWORKS\033[0m"
# docker network ls | awk 'NR==1 || ($2 != "bridge" && $2 != "host" && $2 != "none")'
docker network ls