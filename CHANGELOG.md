# Changelog

## 0.0.1

First release from `sikaru-clients`. The Python SDK, TypeScript SDK and CLI live in one repository and release together at the same version.

### SDK

- Inference is OpenAI-shaped: chat completions, completions, responses and models, so existing OpenAI-style code can call Sikaru catalog models.
- Models are identified as `lab/model`, and each carries a catalog card with its specs, pricing and published reasoning settings.
- Request reasoning output with `include_reasoning`.

### CLI

- Commands are organized into six product groups: `inference`, `decision`, `agent`, `eval`, `traces` and `environment`. Each has a small verb set with positional arguments, short flags, `--follow` and `--output`. Account commands such as `login` and `config` stay at the top level.
- Chat with a catalog model and browse the catalog, including each model's card, with `sikaru inference`.
- Ask System One choice, yes-or-no and score questions about a text or JSON state with `sikaru decision infer`, or `decisions.create` in the SDKs.
- `sikaru agent run <agent>` runs an agent in a managed sandbox by default, or in your own sandbox with `--compute local`.
- `sikaru login` signs in through the browser; `--with-token` reads a token from standard input, and `sikaru logout` signs out.
