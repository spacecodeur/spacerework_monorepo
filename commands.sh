#!/bin/bash

# Dossier contenant les scripts de commandes
COMMANDS_DIR="./commands"

# Vérifie si le dossier des commandes existe
if [[ ! -d "$COMMANDS_DIR" ]]; then
    echo "Erreur : Le dossier $COMMANDS_DIR n'existe pas."
    exit 1
fi

# Fonction pour afficher l'aide
show_help() {
    echo "Usage: $0 <commande>"
    echo "Commandes disponibles :"
    for cmd_file in "$COMMANDS_DIR"/*.sh; do
        cmd_name=$(basename "$cmd_file" .sh)
        echo "  $cmd_name"
    done
}

# Fonction pour l'autocomplétion
_commands_autocomplete() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local commands=$(find "$COMMANDS_DIR" -name "*.sh" -exec basename {} .sh \;)
    COMPREPLY=($(compgen -W "$commands" -- "$cur"))
}

# Détection si le script est sourcé ou exécuté
if [[ "$0" != "${BASH_SOURCE[0]}" ]]; then
    # Activer l'autocomplétion uniquement si sourcé
    complete -F _commands_autocomplete ./commands.sh
else
    # Si exécuté directement, rappeler de sourcer pour l'autocomplétion
    if [[ $1 == "--setup" ]]; then
        echo "Ajout automatique de source dans ~/.bashrc"
        SCRIPT_PATH=$(realpath "$0")
        if ! grep -q "source $SCRIPT_PATH" ~/.bashrc; then
            echo "source $SCRIPT_PATH" >> ~/.bashrc
            echo "Le script a été ajouté à ~/.bashrc. Veuillez exécuter : source ~/.bashrc"
        else
            echo "Le script est déjà présent dans ~/.bashrc"
        fi
        exit 0
    fi

    # Gestion des arguments
    if [[ $# -eq 0 ]]; then
        show_help
        exit 1
    fi

    COMMAND=$1
    COMMAND_FILE="$COMMANDS_DIR/$COMMAND.sh"

    # Vérification et exécution de la commande avec les paramètres à partir du 2e argument
    if [[ -f "$COMMAND_FILE" ]]; then
        bash "$COMMAND_FILE" "${@:2}"  # On passe tous les paramètres sauf le premier (commande)
    else
        echo "Commande inconnue : $COMMAND"
        show_help
        exit 1
    fi
fi
