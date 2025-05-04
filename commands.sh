#!/bin/bash

COMMANDS_DIR="./commands"
ENV_FILE=".env"

if [[ ! -d "$COMMANDS_DIR" ]]; then
    echo "Error: The directory $COMMANDS_DIR does not exist."
    exit 1
fi

if [ -f "$ENV_FILE" ]; then
  APP_NAME=$(grep '^APP_NAME=' "$ENV_FILE" | cut -d '=' -f 2)
else
  echo "Erreur : le fichier .env est introuvable."
  exit 1
fi

export APP_NAME

show_help() {
    echo "Usage: \`$0 <command/subcommand/...>.sh\`"
    echo "Available commands:"
    find "$COMMANDS_DIR" -type f -name "*.sh" | sed "s|^$COMMANDS_DIR/||" | sed 's|.sh$||' | sort
    echo "Enable autocompletion: \`$0 --setup\`"
}

is_running_in_docker() {
  if [[ -f "/.dockerenv" ]]; then
    return 0
  else
    return 1
  fi
}

_commands_autocomplete() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local commands=$(find "$COMMANDS_DIR" -type f -name "*.sh" | sed "s|^$COMMANDS_DIR/||" | sed 's|.sh$||')
    COMPREPLY=($(compgen -W "$commands" -- "$cur"))
}

if [[ "$0" != "${BASH_SOURCE[0]}" ]]; then
    complete -F _commands_autocomplete ./commands.sh
else
    if [[ $1 == "--setup" ]]; then
        SCRIPT_PATH=$(realpath "$0")
        if ! grep -q "source $SCRIPT_PATH" ~/.bashrc; then
            echo "source $SCRIPT_PATH" >> ~/.bashrc
            echo "The script has been added to ~/.bashrc"
        else
            echo "The script is already present in ~/.bashrc"
        fi
        echo "Remember to run \`source ~/.bashrc\`!"
        exit 0
    fi

    if [[ $# -eq 0 ]]; then
        show_help
        exit 1
    fi

    COMMAND=$1
    COMMAND_FILE="$COMMANDS_DIR/$COMMAND.sh"

    # check if the command file exists
    if [[ ! -f "$COMMAND_FILE" ]]; then
        echo "Unknown command: $COMMAND"
        show_help
        exit 1
    fi

    if [[ "$COMMAND" == fc/* ]] && ! is_running_in_docker; then
        
        SERVICE=$(echo "$COMMAND" | cut -d '/' -f 2)

        CONTAINER_NAME="${APP_NAME}-${SERVICE}-container"

        # keep --tty and --interactive : if not, SIGINT (ctrl+c) won't be correctly passed from host to container
        docker exec --tty --interactive --workdir /app "$CONTAINER_NAME" ./commands.sh "$COMMAND" "${@:2}"

    else
        # execute the command with parameters starting from the second argument
        bash "$COMMAND_FILE" "${@:2}"
    fi
fi