#!/bin/bash

COMMANDS_DIR="./commands"

if [[ ! -d "$COMMANDS_DIR" ]]; then
    echo "Error: The directory $COMMANDS_DIR does not exist."
    exit 1
fi

show_help() {
    echo "Usage: \`$0 <command>\`"
    echo "Available commands: (fc-* = from (docker) container)"
    for cmd_file in "$COMMANDS_DIR"/*.sh; do
        cmd_name=$(basename "$cmd_file" .sh)
        echo "  $cmd_name"
    done
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
    local commands=$(find "$COMMANDS_DIR" -name "*.sh" -exec basename {} .sh \;)
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

    # Check if the command file exists
    if [[ ! -f "$COMMAND_FILE" ]]; then
        echo "Unknown command: $COMMAND"
        show_help
        exit 1
    fi

    if [[ "$COMMAND" == fc-* ]] && ! is_running_in_docker; then
        # keep --tty and --interactive : if not, SIGINT (ctrl+c) won't be correctly pass from host to container
        docker exec --tty --interactive --workdir /app spacerework-container ./commands.sh $COMMAND
    else
        # Execute the command with parameters starting from the second argument
        bash "$COMMAND_FILE" "${@:2}"
    fi
fi