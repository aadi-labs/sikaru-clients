# Changelog

Release notes are also published at https://docs.sikaru.ai/sdks-and-cli/release-notes.

## 0.0.3

The Python SDK, TypeScript SDK and CLI release together at 0.0.3. Upgrade all three to the same version.

### Breaking changes

- `tool_providers.register_tool_provider` is removed. Declare a provider with `tool_providers.apply_tool_provider`, which creates it or updates it in place by capability prefix and always returns the same ID.
- Attaching a source tool skill now addresses the provider by capability prefix instead of provider ID.
- Tool-request pushes and schedule callbacks are signed in the Standard Webhooks format (`webhook-id`, `webhook-timestamp`, `webhook-signature`) with a per-provider secret. Verify deliveries with the new `webhooks.verify` helper.

### SDK

- `tool_providers` adds `list_tool_providers` (filter by capability prefix and status), `get_tool_provider`, `rotate_tool_provider_secret` and `expire_previous_tool_provider_secret`. Reads work with a product API key that has `tool_providers:read`.
- Supply your own delivery secret when you apply or rotate, or let Sikaru generate one and return it once. Rotation keeps the previous secret valid for an overlap (24 hours by default, up to 7 days, or 0 for immediate).
- `webhooks.verify` in the Python SDK (`sikaru_sdk.webhooks`) and the TypeScript SDK (`@aadi-labs/sikaru-sdk/webhooks`) checks a delivery against one or more secrets, with timestamp tolerance and constant-time comparison.

### CLI

- `sikaru agent tool-providers` adds `apply_tool_provider`, `list_tool_providers`, `get_tool_provider`, `rotate_tool_provider_secret` and `expire_previous_tool_provider_secret`, and removes `register_tool_provider`.

## 0.0.2

The Python SDK, TypeScript SDK and CLI release together at 0.0.2. Upgrade all three to the same version.

### Breaking changes

- The `specialists` resource is now `tasks`, addressed by one `task_id`. `tasks.list`, `tasks.get`, `tasks.message` and `tasks.cancel` replace the `specialists` methods.
- `online_evaluations` and `judge_alignment` are removed. Use `graders` to ask questions about production conversations and hold releases that make things worse.
- `memory_registry` and the end-user fact methods `end_users.update_fact` and `end_users.delete_fact` are removed. Agent memory is now plain files, managed with `memory`.
- `managed_agents.create_definition_revision` and CLI authoring from `sikaru.json` or hand-written definitions are removed. An agent is now a folder: start one with `sikaru agent init`.
- `sikaru traces get` is now `sikaru trajectories get`. `sikaru traces` keeps `upload`.
- Prebuilt CLI binaries now cover Apple Silicon macOS, x86_64 Linux (glibc) and x86_64 Windows. Intel macOS, ARM Linux and musl Linux builds are no longer published.

### SDK

- `memory` reads, writes, moves and deletes files in an agent's memory folders, lists versions and rolls back, and runs `remember`, `forget` and `dream`.
- `graders` lists and updates Graders, tests one on recent runs without saving, takes labels from a label queue, re-scores recent conversations and estimates cost.
- Release checks: `agent_documents.get_publication` reads a publication and its check, and `agent_documents.publish_anyway` and `changesets.publish_changeset_anyway` publish past a held check with a reason.
- `tasks` adds `actions`, `approve_action`, `changes`, `integrate`, `integrations` and `stop_all` for background tasks.
- Hosted browser: `managed_agents` manages an agent's saved browser logins, playbooks and allow rules. `runs` adds live previews, receipts with screenshots, `take_over`, `end_take_over` and `create_handoff`, and `personal_channels` adds the same controls for personal agents.
- `end_users` adds `import_state`, learning runs (`start_learning_run`, `get_learning_run`) and `revoke_learning_source`.
- `run_schedules` adds `inspect_schedule` and `update_schedule`.
- Inference: omit `model` to use your project's default model. Each model card now includes `specs.sampling_defaults`, the lab's recommended sampling, which applies when a request leaves those fields out.

### CLI

- Author an agent as a folder: `sikaru agent init`, `sikaru agent add`, `sikaru agent info`, `sikaru agent run` and `sikaru agent publish` work with `agent.md` plus its subagents, schedules, channels and skills.
- `sikaru agent grader test NAME` scores a folder's Grader on recent runs and saves nothing; `sikaru agent grader run NAME` re-scores recent production conversations.
- `sikaru agent publish` waits for release checks, exits 1 when a release is held, and prints the command to publish anyway with a reason. `--no-wait` returns while checks are pending.
- `sikaru agent memory` manages memory files, versions, forgets and dreams.
- `sikaru traces upload` detects a file's format from its rows: OTLP spans, ATIF trajectories, OpenInference spans, or a trace export from another observability tool, which Sikaru turns into trajectories. Large exports are sent in batches that never split a trace.
- With `--compute local`, a `402` response keeps its billing advice and exit code, and `429` and `5xx` responses stay retryable.

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
