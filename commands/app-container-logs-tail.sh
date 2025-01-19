#!/bin/bash

container_id=$(docker inspect --format="{{.Id}}" "${APP_NAME}-app-container")
log_file="/var/lib/docker/containers/${container_id}/${container_id}-json.log"
sudo tail -f "$log_file"