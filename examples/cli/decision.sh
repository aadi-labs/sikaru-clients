#!/usr/bin/env bash
# Ask System One questions about a text state from the CLI.
# Install: curl -fsSL https://sikaru.ai/install.sh | sh
# Authenticate with `sikaru login` and `sikaru config set project PROJECT`, or set SIKARU_API_KEY.
set -euo pipefail

TICKET="I was charged twice for my subscription. Please fix this ASAP."

# Yes-or-no questions inline.
sikaru decision infer "$TICKET" --noul urgent="Is this urgent?" --noul refund="Does the customer want a refund?"

# Choice and score questions from a file, printed as JSON.
QUESTIONS="$(mktemp)"
trap 'rm -f "$QUESTIONS"' EXIT
cat > "$QUESTIONS" <<'JSON'
{
  "category": {"type": "choice", "instructions": "What is this ticket about?",
               "criteria": {"billing": null, "technical": null, "other": null}},
  "severity": {"type": "score", "instructions": "How severe is it?", "criteria": ["low", "medium", "high"]}
}
JSON
sikaru decision infer "$TICKET" --questions "$QUESTIONS" -o json
