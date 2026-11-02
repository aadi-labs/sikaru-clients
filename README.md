# Sikaru clients

Sikaru runs managed agents, evaluations and trace analysis, and serves a catalog of
models over one API. This repository holds the public clients for that API: a Python
SDK, a TypeScript SDK and the `sikaru` command-line tool. All three ship at the same
version.

## Install

| Client | Install |
| --- | --- |
| Python SDK (`sikaru_api`) | `pip install sikaru_api` or `uv add sikaru_api` |
| TypeScript SDK (`@sikaru/sdk`) | `npm i @sikaru/sdk` |
| CLI (`sikaru`) | `curl -fsSL https://sikaru.ai/install.sh \| sh` or `npm i -g @sikaru/cli` |

Create a project API key in the Sikaru dashboard and set `SIKARU_API_KEY`. The CLI can
also sign you in with `sikaru login`.

## Quick start

```python
from sikaru_api import Sikaru

client = Sikaru()  # reads SIKARU_API_KEY
reply = client.chat.completions.create(
    model="moonshotai/kimi-k3",
    messages=[{"role": "user", "content": "Explain a rainbow in one sentence."}],
)
print(reply.choices[0].message.content)
```

```sh
sikaru inference chat moonshotai/kimi-k3 "Explain a rainbow in one sentence."
```

## What you can do

Sikaru has five product groups. Each row links to where that group lives in each
client: the generated reference for the API operations behind it, or a runnable example.

| Group | What it does | Python | TypeScript | CLI |
| --- | --- | --- | --- | --- |
| inference | Call catalog models: chat, completions and responses | [example](examples/python/inference.py), [reference](sdk/python/reference.md#chat-completions) | [example](examples/typescript/inference.ts), [reference](sdk/typescript/reference.md#chat-completions) | [example](examples/cli/inference.sh) |
| agent | Create, run and publish managed agents | [reference](sdk/python/reference.md#managedagents) | [reference](sdk/typescript/reference.md#managedagents) | [reference](cli/reference.md#sikaru-managed-agents) |
| eval | Run an agent against a dataset and read the results | [reference](sdk/python/reference.md#checks) | [reference](sdk/typescript/reference.md#checks) | [reference](cli/reference.md#sikaru-checks) |
| traces | Upload traces from other systems and read run trajectories | [reference](sdk/python/reference.md#traceimports) | [reference](sdk/typescript/reference.md#traceimports) | [reference](cli/reference.md#sikaru-trace-imports) |
| environment | Create and version the datasets and checks agents are evaluated in | [reference](sdk/python/reference.md#datasets) | [reference](sdk/typescript/reference.md#datasets) | [reference](cli/reference.md#sikaru-datasets) |

The CLI groups are `sikaru inference`, `sikaru agent`, `sikaru eval`, `sikaru traces`
and `sikaru environment`. Run `sikaru <group> --help` for the commands in each. The CLI
reference documents the API operation commands behind them, which are also callable
under each product.

## Repository map

| Path | Contents |
| --- | --- |
| [`sdk/python`](sdk/python) | Python SDK (`sikaru_api` on PyPI) |
| [`sdk/typescript`](sdk/typescript) | TypeScript SDK, published as `@sikaru/sdk` |
| [`cli`](cli) | The `sikaru` command-line tool |
| [`shared`](shared) | Authoring companions and cross-language contracts |
| [`examples`](examples) | Runnable examples for the Python SDK, the TypeScript SDK and the CLI |
| [`scripts`](scripts) | Release and CI checks |

`sdk/` and `cli/` are generated from the Sikaru API definition. Open an issue rather
than editing them by hand.

## Shared

[`shared/authoring`](shared/authoring/README.md) packages an agent's source files
(instructions, skills, assets, eval notes) from a `sikaru.json` manifest, in Python and
Rust, ready to send through the SDK or CLI.
[`shared/compute-contracts`](shared/compute-contracts/README.md) holds the contracts
that keep the Python, TypeScript and CLI clients consistent when you attach your own
compute.

## Releases

See [CHANGELOG.md](CHANGELOG.md) for what changed in each version and
[RELEASING.md](RELEASING.md) for how a release is cut.
