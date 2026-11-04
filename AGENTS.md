# AGENTS.md

Guidance for AI coding agents working with the Sikaru clients.

## What this repository is

The official Sikaru clients, released together under one version and one `vX.Y.Z` tag:

| Path | Package | Install |
| --- | --- | --- |
| `cli/` | `sikaru` CLI | `curl -fsSL https://app.sikaru.ai/install.sh \| sh` or `npm install -g @aadi-labs/sikaru-cli` |
| `sdk/python/` | `sikaru-sdk` on PyPI, imported as `sikaru_sdk` | `pip install sikaru-sdk` |
| `sdk/typescript/` | `@aadi-labs/sikaru-sdk` on npm | `npm install @aadi-labs/sikaru-sdk` |

`examples/` has runnable CLI and SDK examples. `shared/` holds the authoring companions and shared contracts.

## Using Sikaru from your project

- Install the published packages above. Do not vendor, copy or rebuild client code into another project.
- Sign in with `sikaru login`, or set `SIKARU_API_KEY` on the server for SDK code and CI. Never put a key in browser code, committed files or chat output.
- Use only operations in the API schema: https://docs.sikaru.ai/openapi.json.
- An accepted request is not a finished run. Read status until it completes, fails or is cancelled.
- Reuse the same idempotency key when retrying the same submission.

Start here: https://sikaru.ai/llms.txt, https://docs.sikaru.ai/llms.txt and https://sikaru.ai/agents.md. Skills for coding agents: `npx skills add aadi-labs/sikaru-plugins`.

## Changing this repository

- `sdk/` and `cli/` are generated from the Sikaru API definition. Do not hand-edit generated files; changes are overwritten by the next release.
- To report a bug or request a change, open an issue with the client version, operation name and a minimal reproduction. Redact keys and private data.
- Releases are cut by maintainers; see `RELEASING.md`.
