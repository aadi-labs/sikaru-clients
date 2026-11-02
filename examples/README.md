# Examples

The inference examples call a model from the Sikaru catalog, and the agent example
creates and runs a managed agent. Create a project API key in the
Sikaru dashboard and set it in your environment, along with a model ID:

```sh
export SIKARU_API_KEY=...
export SIKARU_MODEL=moonshotai/kimi-k3   # any ID from the models list
```

List the models available to your project with `sikaru inference models`.

| Example | What it does | Run it |
| --- | --- | --- |
| [python/inference.py](python/inference.py) | Lists models, then makes a plain, a streamed and an async chat call | `pip install sikaru_api && python examples/python/inference.py` |
| [typescript/inference.ts](typescript/inference.ts) | Lists models, then makes a plain and a streamed chat call | `npm i @sikaru/sdk && node --experimental-strip-types examples/typescript/inference.ts` |
| [cli/inference.sh](cli/inference.sh) | Lists and describes models, then chats with one | `bash examples/cli/inference.sh` |
| [cli/decision.sh](cli/decision.sh) | Asks System One yes-or-no, choice and score questions about a support ticket | `bash examples/cli/decision.sh` |
| [cli/agent.sh](cli/agent.sh) | Creates, publishes and runs the agent in [agents/data-helper.md](agents/data-helper.md) | `SIKARU_PROJECT=... bash examples/cli/agent.sh` |
| [cli/credits.sh](cli/credits.sh) | Creates a credit top-up offer, waits while an agent wallet pays its URL, then checks it | `SIKARU_PROJECT=... bash examples/cli/credits.sh` |

The Python and TypeScript examples use the OpenAI-shaped client, `Sikaru` and
`AsyncSikaru` in Python and `@sikaru/sdk/inference` in TypeScript. Passing
`stream=True` (`stream: true` in TypeScript) to `chat.completions.create` returns a
stream of chunks. The TypeScript example uses top-level `await`, so run it as an ES module: Node 22.6
or later, in a project with `"type": "module"`.

## Agent documents

An agent is one Markdown file: YAML frontmatter for settings, then the
instructions. [agents/data-helper.md](agents/data-helper.md) turns web search off,
asks before running shell commands, limits web fetches to one domain and installs
pandas. The frontmatter accepts `model`, `tools`, `web`, `setup`, `access`, `budget`
and `sources`:

- `tools` turns the built-in tools on or off and sets their policy (`allow`,
  `require_approval` or `deny`): `bash`, `workspace`, `agents`, `memory`,
  `web_search` and `web_fetch`. All are on by default.
- `web` sets `allow_domains` or `block_domains`.
- `setup` installs `pip` and `npm` packages, runs `commands` and clones `repos`.
- `model` takes an ID from `sikaru inference models`; omit it to use the project
  default.

To give an agent an app or MCP server, connect it to the project first (in the
dashboard, or with `sikaru agent connections`), then mention it in the
instructions, for example `@app/github` or `@mcp/research`. An optional `access`
entry narrows what the mention grants:

```yaml
access:
  "@app/github":
    tools: [GITHUB_CREATE_ISSUE]
    policy: require_approval
```

`sikaru agent validate` reports any mention that does not match a connection. The
full reference is the [agent guide](https://sikaru.ai/agent-guide.md), with a JSON
schema at [sikaru.ai/schemas/agent-document.json](https://sikaru.ai/schemas/agent-document.json).
