#!/bin/bash

COMMANDS_DIR="./commands"

if [[ ! -d "$COMMANDS_DIR" ]]; then
    echo "Erreur : Le dossier $COMMANDS_DIR n'existe pas."
    exit 1
fi

show_help() {
    echo "Usage: \`$0 <commande>\`"
    echo "Commandes disponibles : ( fc-* = from (docker) container )"
    for cmd_file in "$COMMANDS_DIR"/*.sh; do
        cmd_name=$(basename "$cmd_file" .sh)
        echo "  $cmd_name"
    done
    echo "Activer l'autocomplétion : \`$0 --setup\`"
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
            echo "Le script a été ajouté à ~/.bashrc"
        else
            echo "Le script est déjà présent dans ~/.bashrc"
        fi
        echo "Penser à exécuter \`source ~/.bashrc\` ! "
        exit 0
    fi

    if [[ $# -eq 0 ]]; then
        show_help
        exit 1
    fi

    COMMAND=$1
    COMMAND_FILE="$COMMANDS_DIR/$COMMAND.sh"

    # Vérification si le fichier de commande existe
    if [[ ! -f "$COMMAND_FILE" ]]; then
        echo "Commande inconnue : $COMMAND"
        show_help
        exit 1
    fi

    if [[ "$COMMAND" == fc-* ]] && ! is_running_in_docker; then
        docker exec --workdir /app spacerework-container ./commands.sh $COMMAND  
    else
        # Exécuter la commande avec les paramètres à partir du 2e argument
        bash "$COMMAND_FILE" "${@:2}"
    fi
fi
