#!/bin/bash

show_help() {
    echo "Usage: $0 <username> <ip_address> [path_to_ssh_key]"
    echo "  username           : Username on the remote server"
    echo "  ip_address         : IP address of the remote server (must be valid)"
    echo "  path_to_ssh_key    : (optional) Path to the private SSH key (.pem)"
    echo "  If the SSH key is not specified, the command will use the standard SSH connection method."
}

if [[ $# -lt 2 ]]; then
    echo "Error: Two required parameters are missing."
    show_help
    exit 1
fi

USER=$1
SERVER=$2
KEY_PATH=$3

# Check if the IP address is valid (IPv4 format)
if ! [[ "$SERVER" =~ ^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: The IP address '$SERVER' is not valid."
    show_help
    exit 1
fi

# If a key path is provided, validate its existence
if [[ -n "$KEY_PATH" ]]; then
    if [[ ! -f "$KEY_PATH" ]]; then
        echo "Error: The SSH key file '$KEY_PATH' does not exist."
        show_help
        exit 1
    fi
    SSH_CMD="ssh -i $KEY_PATH $USER@$SERVER"
else
    SSH_CMD="ssh $USER@$SERVER"
fi

# Compile and create the archive
tuono build && \
cargo build --release && \
[ -f release.tar.gz ] && rm release.tar.gz
tar -czf release.tar.gz ./out ./target/release/tuono ./target/release/tuono.d

# copy the archive to the remote server
if [[ -n "$KEY_PATH" ]]; then
    # If an SSH key is specified
    scp -i "$KEY_PATH" release.tar.gz "$USER@$SERVER:/home/$USER/"
else
    scp release.tar.gz "$USER@$SERVER:/home/$USER/"
fi

# Check if the SCP command failed
if [[ $? -ne 0 ]]; then
    echo "Error: Failed to copy the archive to the remote server."
    exit 1
fi

rm release.tar.gz

# Commands to execute on the remote server

# 1. Kill the process using port 3000 on the remote server
$SSH_CMD "lsof -t -i :3000 | xargs kill -9"

# 2. Wait for port 3000 to be freed (max 10 seconds)
echo "Waiting for port 3000 to be freed..."
for i in {1..10}; do
    if ! $SSH_CMD "lsof -i :3000"; then
        echo "Port 3000 freed."
        break
    fi
    if [ $i -eq 10 ]; then
        echo "Error: Port 3000 could not be freed after 10 seconds."
        exit 1
    fi
    sleep 1
done

# 3. Remove the "release" directory if it exists
$SSH_CMD "rm -rf /home/$USER/release"

# 4. Create a "release" directory and extract the archive into it
$SSH_CMD "mkdir -p /home/$USER/release"
$SSH_CMD "tar -xzf /home/$USER/release.tar.gz -C /home/$USER/release"

# 5. Remove the archive on the remote server after extraction
$SSH_CMD "rm /home/$USER/release.tar.gz"

# 6. Run the binary "./target/release/tuono" in the background using nohup
$SSH_CMD "cd /home/$USER/release && bash -c './target/release/tuono > /home/$USER/release/tuono.log 2>&1 &' && exit"

# Close the SSH session
$SSH_CMD "exit"

# End !
echo "The server has been successfully updated and the process has started."