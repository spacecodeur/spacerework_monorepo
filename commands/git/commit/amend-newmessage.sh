#!/bin/bash

# -------------------- What is git commit --amend? --------------------
# This command lets you modify the most recent commit.
# It can be used to:
#   - Add new staged changes to the previous commit
#   - Edit the commit message of the previous commit
# It replaces the last commit with a new one.
# ⚠️ If the last commit has already been pushed, use with caution,
#    as it rewrites history and may require a force push.
# ---------------------------------------------------------------------

echo "Performing git commit --ammend..."
git commit --amend

if [ $? -eq 0 ]; then
  echo "Commit amended successfully."
else
  echo "❌ Failed to amend commit."
  exit 1
fi