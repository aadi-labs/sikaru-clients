#!/usr/bin/env bash
# Call a Sikaru catalog model from the CLI.
# Install: curl -fsSL https://app.sikaru.ai/install.sh | sh
# Authenticate with `sikaru login`, or set SIKARU_API_KEY.
set -euo pipefail

MODEL="${SIKARU_MODEL:?Set SIKARU_MODEL to a model ID, for example moonshotai/kimi-k3}"

# List models, and describe one.
sikaru inference models
sikaru inference models "$MODEL"

# One-shot reply, then a streamed one.
sikaru inference chat "$MODEL" "Explain a rainbow in one sentence."
sikaru inference chat "$MODEL" "Explain a rainbow in one sentence." --stream

# Read the message from stdin and add a system prompt.
echo "Explain a rainbow in one sentence." | sikaru inference chat "$MODEL" --system "Answer in plain words."
