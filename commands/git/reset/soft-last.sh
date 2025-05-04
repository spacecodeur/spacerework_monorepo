#!/bin/bash

echo "🔄 Performing soft reset..."
git reset --soft HEAD~1

if [ $? -eq 0 ]; then
  echo "✅ Soft reset successful. Changes from the last commit are now staged."
else
  echo "❌ Soft reset failed."
  exit 1
fi