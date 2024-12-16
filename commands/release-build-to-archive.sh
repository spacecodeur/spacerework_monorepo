#!/bin/bash

# Fonction pour afficher l'aide
show_help() {
    echo "Usage: $0 <nom_utilisateur> <adresse_ip> [chemin_vers_la_clé_ssh]"
    echo "  nom_utilisateur    : Nom d'utilisateur sur le serveur distant"
    echo "  adresse_ip         : Adresse IP du serveur distant (doit être valide)"
    echo "  chemin_vers_la_clé_ssh : (optionnel) Chemin vers la clé SSH privée (.pem)"
    echo "  Si la clé SSH n'est pas spécifiée, la commande utilisera la méthode classique de connexion SSH."
}

# Vérification du nombre de paramètres
if [[ $# -lt 2 ]]; then
    echo "Erreur : Deux paramètres obligatoires sont manquants."
    show_help
    exit 1
fi

# Récupération des paramètres
USER=$1
SERVER=$2
KEY_PATH=$3

# Vérification de l'adresse IP (format IPv4)
if ! [[ "$SERVER" =~ ^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Erreur : L'adresse IP '$SERVER' n'est pas valide."
    show_help
    exit 1
fi

# Si un chemin de clé est fourni, vérifier sa validité
if [[ -n "$KEY_PATH" ]]; then
    if [[ ! -f "$KEY_PATH" ]]; then
        echo "Erreur : Le fichier de clé SSH '$KEY_PATH' n'existe pas."
        show_help
        exit 1
    fi
    SSH_CMD="ssh -i $KEY_PATH $USER@$SERVER"
else
    SSH_CMD="ssh $USER@$SERVER"
fi

# Compilation et création de l'archive
tuono build && \
cargo build --release && \
[ -f release.tar.gz ] && rm release.tar.gz
tar -czf release.tar.gz ./out ./target/release/tuono ./target/release/tuono.d

# Commande SCP pour copier l'archive sur le serveur distant
if [[ -n "$KEY_PATH" ]]; then
    # Si une clé SSH est spécifiée
    scp -i "$KEY_PATH" release.tar.gz "$USER@$SERVER:/home/$USER/"
else
    # Sinon, utiliser la méthode classique de connexion SSH
    scp release.tar.gz "$USER@$SERVER:/home/$USER/"
fi

# Vérification de l'échec de la commande SCP
if [[ $? -ne 0 ]]; then
    echo "Erreur : Échec de la copie de l'archive sur le serveur distant."
    exit 1
fi

# Suppression de l'archive locale
rm release.tar.gz

# Commandes à exécuter sur le serveur distant

# 1. Tuer le processus utilisant le port 3000 sur le serveur distant
$SSH_CMD "lsof -t -i :3000 | xargs kill -9"

# 2. Attendre que le port 3000 soit libéré (max 10 secondes)
echo "Attente de la libération du port 3000..."
for i in {1..10}; do
    if ! $SSH_CMD "lsof -i :3000"; then
        echo "Port 3000 libéré."
        break
    fi
    if [ $i -eq 10 ]; then
        echo "Erreur : Le port 3000 n'a pas pu être libéré après 10 secondes."
        exit 1
    fi
    sleep 1
done

# 3. Supprimer le dossier "release" s'il existe
$SSH_CMD "rm -rf /home/$USER/release"

# 4. Créer un dossier "release" et y dézipper l'archive
$SSH_CMD "mkdir -p /home/$USER/release"
$SSH_CMD "tar -xzf /home/$USER/release.tar.gz -C /home/$USER/release"

# 5. Supprimer l'archive sur le serveur distant après extraction
$SSH_CMD "rm /home/$USER/release.tar.gz"

# 6. Exécuter le binaire "./target/release/tuono" en arrière-plan avec nohup
$SSH_CMD "cd /home/$USER/release && bash -c './target/release/tuono > /home/$USER/release/tuono.log 2>&1 &' && exit"

# Fermer la session SSH
$SSH_CMD "exit"

# Fin du script
echo "Le serveur a été mis à jour avec succès et le processus a démarré."
