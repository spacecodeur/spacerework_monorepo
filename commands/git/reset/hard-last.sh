#!/bin/bash

echo "⚠️  WARNING: You are about to perform a HARD RESET to the last commit (HEAD)."
echo ""
echo "This will:"
echo "  Remove ALL uncommitted changes in your working directory."
echo "  Delete ALL staged changes (files in 'git add')."
echo "  Any unsaved work will be LOST permanently."
echo ""

# Prompt for confirmation
read -p "Are you sure you want to continue? [yes/NO]: " confirm

if [[ "$confirm" != "yes" ]]; then
  echo "Operation cancelled."
  exit 0
fi

echo "Performing hard reset..."
git reset --hard HEAD

if [ $? -eq 0 ]; then
  echo "Hard reset completed. Your working directory is now clean and matches the last commit."
else
  echo "❌ Hard reset failed."
  exit 1
fi