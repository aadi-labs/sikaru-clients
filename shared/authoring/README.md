# Sikaru authoring companions

The generated clients (`sdk/` and `cli/`) expose the Sikaru API contract and are
unmodified generator output. The authoring companions add customer-source
packaging on top of them. They are installed separately and built from this
repository.

| Implementation | Package | Built on |
| --- | --- | --- |
| [Python](python/) | `sikaru-authoring` (`sikaru_authoring`) | the generated Python SDK |
| [Rust](rust/) | `sikaru-authoring` binary | the generated CLI library in `cli/` |

Both compile the same source manifest the same way.

## Source manifest

Only files named in `sikaru.json` are packaged. Paths must be relative; symlinks,
traversal, unsupported source kinds and oversized bundles are rejected. A
definition is limited to 1 MB and 100 files. Keep sensitive files out of the
manifest.

```json
{
  "name": "my-agent",
  "sources": [
    {"path": "instructions.md", "kind": "agent_md"},
    {"path": "skills/research/SKILL.md", "kind": "agent_skill"},
    {"path": "skills/research/assets/input.csv", "kind": "skill_asset"},
    {"path": "evals/acceptance.md", "kind": "eval_md"}
  ]
}
```

The manifest may also declare optional `web`, `tools` and `setup` sections. Both
implementations validate them locally and include them in the definition as
written. Unknown manifest keys are rejected. The gateway validates the public
source contract before publication.

## Python

Install from the repository root:

```sh
python -m pip install ./shared/authoring/python
```

```python
from sikaru_authoring import compile_directory

bundle = compile_directory("support-agent")
# Pass bundle.definition and bundle.content_digest to the generated SDK.
```

Compilation makes no network requests and includes no execution runtime.

### Markdown agent documents

Document parsing and access validation happen on the server. The document
helpers preserve UTF-8 bytes, including CRLF line endings, through pull and push:

```python
from sikaru_api import SikaruApi
from sikaru_authoring.documents import pull_document, push_document, publish_document

client = SikaruApi()  # Reads SIKARU_API_KEY from the environment.
draft = pull_document(client, "project-id", "support", "agent.md")
# Edit agent.md, then save against the revision you read.
saved = push_document(client, "project-id", "support", "agent.md",
                      expected_revision=draft.revision)
```

A stale save raises the generated `ApiError` with status 409 and the newer draft
in `body["detail"]["draft"]`. Review that draft before resubmitting. Publication
requires its revision and the expected live version, with explicit
acknowledgement of access changes. Use `client.agent_documents.compare` to review
the current live version and `client.agent_documents.snippets` for hosted session
examples. The CLI exposes the same workflow under `sikaru agent`; use each
command's `--help` for arguments.

## Rust

Install from the repository root:

```sh
cargo install --locked --path shared/authoring/rust
sikaru-authoring init support-agent --name support
sikaru-authoring init research-agent --name research --capabilities
sikaru-authoring check support-agent
sikaru-authoring check support-agent --ceilings ceilings.json
sikaru-authoring dev support --revision 7 --project PROJECT --tenant TENANT --user USER --prompt 'Help this customer.'
```

`init DIRECTORY` creates a new source directory with `sikaru.json` and
`instructions.md`, and fails if the directory already exists; `--name` sets the
agent name and `--capabilities` also scaffolds the optional manifest sections.
`check [DIRECTORY]` reads only the manifest-listed files and prints the agent name,
its definition and the canonical `contentDigest`. It makes no API request.
`--ceilings` reports every field that exceeds the project's capability ceilings.

`dev AGENT --revision N --project P --tenant T --user U [--prompt TEXT]
[--acknowledge-widening]` tests an existing agent at its saved document revision
without changing Live. It creates a session in the draft environment pinned to
that revision, using the same generated API executor, credentials and base URL as
the `sikaru` CLI. `--prompt` queues one turn and returns its IDs without waiting.
`--acknowledge-widening` acknowledges an access widening in that revision.
`--dry-run` prints the request without contacting the server.

Authenticate with `SIKARU_API_KEY` or `sikaru-authoring auth login`.

## Verify

```sh
RUST_MIN_STACK=16777216 cargo test --locked --manifest-path shared/authoring/rust/Cargo.toml
python -m pip install ./shared/authoring/python
python -c 'from sikaru_authoring import compile_directory'
```
