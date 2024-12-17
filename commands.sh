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

# Fonction pour vérifier si le script s'exécute dans un conteneur Docker
is_running_in_docker() {
  if grep -qE '/docker|/lxc' /proc/1/cgroup 2>/dev/null; then
    return 0 # Vrai, dans un conteneur Docker
  else
    return 1 # Faux, sur la machine hôte
  fi
}

# Détection si le script est sourcé ou exécuté
if [[ "$0" != "${BASH_SOURCE[0]}" ]]; then
    # Activer l'autocomplétion uniquement si sourcé
    complete -F _commands_autocomplete ./commands.sh
else
    # Gestion des arguments
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

    # Vérification si la commande doit être exécutée seulement depuis le container du projet
    if [[ "$COMMAND" == fc-* ]]; then
        if ! is_running_in_docker; then
            echo "Erreur : La commande '$COMMAND' doit être exécutée depuis le conteneur Docker du projet."
            exit 1
        fi
    fi

    # Exécuter la commande avec les paramètres à partir du 2e argument
    bash "$COMMAND_FILE" "${@:2}"
fi
