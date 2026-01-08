#!/bin/sh
# Set up local git to use the repository's .githooks directory and make the pre-commit executable

set -e

echo "Configuring git to use .githooks..."
git config core.hooksPath .githooks

if [ -f .githooks/pre-commit ]; then
  chmod +x .githooks/pre-commit
  echo "Pre-commit hook installed and made executable."
else
  echo "No .githooks/pre-commit found."
fi

echo "Done."
