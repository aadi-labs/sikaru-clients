#!/usr/bin/env bash
# Add credits to your organization from an agent wallet: create a top-up offer, pay its URL,
# then check it. Install: curl -fsSL https://app.sikaru.ai/install.sh | sh
# Authenticate with `sikaru login`, or set SIKARU_API_KEY.
set -euo pipefail

PROJECT="${SIKARU_PROJECT:?Set SIKARU_PROJECT to your project ID}"
AMOUNT="${SIKARU_TOP_UP_USD:-20}"

# Create an offer for AMOUNT US dollars (at least 5). It prints the pay URL and expiry.
OFFER="$(sikaru credits add --amount "$AMOUNT" -p "$PROJECT" -o json)"
echo "$OFFER"
PAY_URL="$(printf '%s' "$OFFER" | python3 -c 'import json, sys; print(json.load(sys.stdin)["payUrl"])')"
ID="$(printf '%s' "$OFFER" | python3 -c 'import json, sys; print(json.load(sys.stdin)["id"])')"

# Pay PAY_URL with an agent wallet that supports HTTP 402 machine payments; see
# https://docs.sikaru.ai/sdks-and-cli/credits for the supported wallet and its command.
echo "Pay this URL with your agent wallet: $PAY_URL"
read -r -p "Press Enter once the payment has gone through..."

# The status is `credited` once the credits have landed.
sikaru credits status "$ID" -p "$PROJECT"
