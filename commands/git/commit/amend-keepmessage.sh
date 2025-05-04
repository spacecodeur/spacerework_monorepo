#!/bin/bash

# -------------------- What is git commit --amend --no-edit? --------------------
# This command:
#   - Adds any staged changes to the last commit
#   - Keeps the original commit message (no editor is opened)
# It rewrites the last commit with the new content, but the same message.
# ⚠️ If the last commit has already been pushed, use with caution,
#    as it rewrites history and may require a force push.
# -------------------------------------------------------------------------------

if git diff --cached --quiet; then
  echo "No staged changes detected. Nothing to amend."
  exit 0
else
  echo "Staged changes detected. They will be added to the last commit."
fi

echo "Performing git commit --ammend --no-edit..."
git commit --amend --no-edit

if [ $? -eq 0 ]; then
  echo "Commit amended successfully (message unchanged)."
else
  echo "❌ Failed to amend commit."
  exit 1
fi