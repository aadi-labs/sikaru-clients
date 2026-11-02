#!/usr/bin/env bash
# Create, publish and run a managed agent from an agent document.
# Install: curl -fsSL https://sikaru.ai/install.sh | sh
# Authenticate with `sikaru login`, or set SIKARU_API_KEY.
set -euo pipefail

PROJECT="${SIKARU_PROJECT:?Set SIKARU_PROJECT to your project ID}"
AGENT="${SIKARU_AGENT:-data-helper}"
FILE="$(dirname "$0")/../agents/data-helper.md"

# Check the document, then create the agent with it as the first draft.
sikaru agent validate "$AGENT" -p "$PROJECT" --file "$FILE"
sikaru agent create "$AGENT" -p "$PROJECT" --file "$FILE" --name "Data helper"

# Review the draft's access, then publish exactly what was reviewed.
REVISION="$(sikaru agent pull "$AGENT" -p "$PROJECT" --file agent.md --query revision)"
DIGEST="$(sikaru agent review "$AGENT" -p "$PROJECT" --revision "$REVISION" --query accessDigest | tr -d '"')"
sikaru agent publish "$AGENT" -p "$PROJECT" --revision "$REVISION" --access-digest "$DIGEST"

# Run one task. Omit the task to start a conversation.
sikaru agent run "$AGENT" -p "$PROJECT" "Which pandas function reads a CSV file?"
