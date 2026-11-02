# Examples

Each example calls a model from the Sikaru catalog. Create a project API key in the
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

The Python and TypeScript examples use the OpenAI-shaped client, `Sikaru` and
`AsyncSikaru` in Python and `@sikaru/sdk/inference` in TypeScript. Passing
`stream=True` (`stream: true` in TypeScript) to `chat.completions.create` returns a
stream of chunks. The TypeScript example uses top-level `await`, so run it as an ES module: Node 22.6
or later, in a project with `"type": "module"`.
