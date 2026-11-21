# Sikaru API CLI Reference

Full command reference for `sikaru`.

## Commands

- [`sikaru activation`](#sikaru-activation)
- [`sikaru agent-documents`](#sikaru-agent-documents)
- [`sikaru agent-imports`](#sikaru-agent-imports)
- [`sikaru agents`](#sikaru-agents)
- [`sikaru auth`](#sikaru-auth)
- [`sikaru capability-ceilings`](#sikaru-capability-ceilings)
- [`sikaru changesets`](#sikaru-changesets)
- [`sikaru channels`](#sikaru-channels)
- [`sikaru chat completions`](#sikaru-chat-completions)
- [`sikaru checks`](#sikaru-checks)
- [`sikaru completions`](#sikaru-completions)
- [`sikaru compute-attachments`](#sikaru-compute-attachments)
- [`sikaru compute-credentials`](#sikaru-compute-credentials)
- [`sikaru compute-environments`](#sikaru-compute-environments)
- [`sikaru compute-operations`](#sikaru-compute-operations)
- [`sikaru compute-workspaces`](#sikaru-compute-workspaces)
- [`sikaru connections`](#sikaru-connections)
- [`sikaru context-registry`](#sikaru-context-registry)
- [`sikaru conversations`](#sikaru-conversations)
- [`sikaru credit-top-ups`](#sikaru-credit-top-ups)
- [`sikaru datasets`](#sikaru-datasets)
- [`sikaru decisions`](#sikaru-decisions)
- [`sikaru deployments`](#sikaru-deployments)
- [`sikaru end-users`](#sikaru-end-users)
- [`sikaru environments`](#sikaru-environments)
- [`sikaru evaluation-comparisons`](#sikaru-evaluation-comparisons)
- [`sikaru evaluation-criteria`](#sikaru-evaluation-criteria)
- [`sikaru evaluation-jobs`](#sikaru-evaluation-jobs)
- [`sikaru evaluation-results`](#sikaru-evaluation-results)
- [`sikaru evaluator-runs`](#sikaru-evaluator-runs)
- [`sikaru execution-objectives`](#sikaru-execution-objectives)
- [`sikaru execution-sessions`](#sikaru-execution-sessions)
- [`sikaru executions`](#sikaru-executions)
- [`sikaru feedback`](#sikaru-feedback)
- [`sikaru git-credentials`](#sikaru-git-credentials)
- [`sikaru graders`](#sikaru-graders)
- [`sikaru harnesses`](#sikaru-harnesses)
- [`sikaru http-channels`](#sikaru-http-channels)
- [`sikaru import-sessions`](#sikaru-import-sessions)
- [`sikaru issue-clusters`](#sikaru-issue-clusters)
- [`sikaru managed-agents`](#sikaru-managed-agents)
- [`sikaru memory`](#sikaru-memory)
- [`sikaru model-gateway`](#sikaru-model-gateway)
- [`sikaru model-settings`](#sikaru-model-settings)
- [`sikaru models`](#sikaru-models)
- [`sikaru personal-channels`](#sikaru-personal-channels)
- [`sikaru projects`](#sikaru-projects)
- [`sikaru release-watches`](#sikaru-release-watches)
- [`sikaru responses`](#sikaru-responses)
- [`sikaru retention-policies`](#sikaru-retention-policies)
- [`sikaru review-queue`](#sikaru-review-queue)
- [`sikaru run-references`](#sikaru-run-references)
- [`sikaru run-schedules`](#sikaru-run-schedules)
- [`sikaru run-webhooks`](#sikaru-run-webhooks)
- [`sikaru runs`](#sikaru-runs)
- [`sikaru sessions`](#sikaru-sessions)
- [`sikaru tasks`](#sikaru-tasks)
- [`sikaru tool-providers`](#sikaru-tool-providers)
- [`sikaru trace-import-connections`](#sikaru-trace-import-connections)
- [`sikaru trace-imports`](#sikaru-trace-imports)
- [`sikaru trace-streams`](#sikaru-trace-streams)
- [`sikaru workflow-intents`](#sikaru-workflow-intents)
- [`sikaru workflow-runs`](#sikaru-workflow-runs)
- [`sikaru workflows`](#sikaru-workflows)

---

### `sikaru activation`

#### `sikaru activation project-activation-status`

Project Activation Status

`GET /v1/projects/{project_id}/activation`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

---

### `sikaru agent-documents`

#### `sikaru agent-documents adopt-suggestion`

Adopt Suggestion

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/document/suggestions/{suggestion_id}/adopt`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--suggestion-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru agent-documents compare`

Compare

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/document/compare`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |

#### `sikaru agent-documents discard`

Discard Document

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/document/discard`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru agent-documents dismiss-suggestion`

Dismiss Suggestion

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/document/suggestions/{suggestion_id}/dismiss`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--suggestion-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |

#### `sikaru agent-documents draft`

Draft a first agent document, starter checks and suggested apps from a description or past conversations.

`POST /v1/projects/{project_id}/agent-documents/draft`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru agent-documents edit-setting`

Set or remove one setting without rewriting the rest of the document.

`POST /v1/projects/{project_id}/agent-documents/settings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru agent-documents get`

Get Document

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/document`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |

#### `sikaru agent-documents get-publication`

A publication's state, read without publishing again: follow a pending publish here until its release check
settles.

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/document/publications/{publication_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--publication-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |

#### `sikaru agent-documents import-files`

Import Document Files

`POST /v1/projects/{project_id}/agent-documents/import`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru agent-documents list-resources`

Document Resources

`GET /v1/projects/{project_id}/agent-documents/resources`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

#### `sikaru agent-documents list-suggestions`

Suggestions

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/document/suggestions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |

#### `sikaru agent-documents list-templates`

Document Templates

`GET /v1/projects/{project_id}/agent-documents/templates`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

#### `sikaru agent-documents list-versions`

Versions

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/document/versions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |

#### `sikaru agent-documents publish`

Publish

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/document/publish`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru agent-documents publish-anyway`

Release a publication its release check blocked or couldn't verify, recording who, when and why (R10, R33).
Only a release check is overridden: an agent's measured evaluation still runs (R34).

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/document/publications/{publication_id}/publish-anyway`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--publication-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru agent-documents revert`

Revert

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/document/revert`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru agent-documents review`

Review Document

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/document/review`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru agent-documents save`

Save Document

`PUT /v1/projects/{project_id}/managed-agents/{agent_slug}/document`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru agent-documents snippets`

Document Snippets

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/document/snippets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |

#### `sikaru agent-documents validate`

Validate Agent Document

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/document/validate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru agent-documents validate-text`

Validate Project Document

`POST /v1/projects/{project_id}/agent-documents/validate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru agent-imports`

#### `sikaru agent-imports create-agent-import`

Create Agent Import

`POST /v1/projects/{project_id}/agent-imports`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru agent-imports list-agent-imports`

List Agent Imports

`GET /v1/projects/{project_id}/agent-imports`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

---

### `sikaru agents`

#### `sikaru agents create-managed-session`

Create Managed Session

`POST /v1/projects/{project_id}/agents/{agent_id}/sessions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru auth`

#### `sikaru auth get-device-configuration`

Device Configuration

`GET /v1/auth/device/config`

---

### `sikaru capability-ceilings`

#### `sikaru capability-ceilings get`

Get Capability Ceilings

`GET /v1/projects/{project_id}/capability-ceilings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

#### `sikaru capability-ceilings update`

Update Capability Ceilings

`PUT /v1/projects/{project_id}/capability-ceilings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru changesets`

#### `sikaru changesets approve-changeset`

Approve Changeset

`POST /v1/projects/{project_id}/changesets/{changeset_id}/approve`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `sikaru changesets create-changeset`

Create Changeset

`POST /v1/projects/{project_id}/changesets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru changesets get-changeset`

Get Changeset

`GET /v1/projects/{project_id}/changesets/{changeset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |

#### `sikaru changesets list-changeset-diffs`

List Changeset Diffs

`GET /v1/projects/{project_id}/changesets/{changeset_id}/diffs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |

#### `sikaru changesets list-changeset-evidence`

List Changeset Evidence

`GET /v1/projects/{project_id}/changesets/{changeset_id}/evidence`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |

#### `sikaru changesets list-changesets`

List Changesets

`GET /v1/projects/{project_id}/changesets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--status` | `string` | No |  |
| `--improvement` | `string` | No |  |

#### `sikaru changesets promote-changeset`

Promote Changeset

`POST /v1/projects/{project_id}/changesets/{changeset_id}/promote`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `sikaru changesets publish-changeset-anyway`

Promote a Try a Fix changeset its agent's release check blocked or couldn't verify, recording who, when and
why (grader release check R10, R33, R36). The contract of an agent document's Publish Anyway: owner credentials
and a reason, refused while the check still runs or when the live version or check settings changed since.

`POST /v1/projects/{project_id}/changesets/{changeset_id}/publications/{publication_id}/publish-anyway`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |
| `--publication-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru changesets reject-changeset`

Reject Changeset

`POST /v1/projects/{project_id}/changesets/{changeset_id}/reject`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `sikaru changesets rollback-changeset`

Rollback Changeset

`POST /v1/projects/{project_id}/changesets/{changeset_id}/rollback`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `sikaru changesets stage-changeset`

Stage Changeset

`POST /v1/projects/{project_id}/changesets/{changeset_id}/stage`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `sikaru channels`

#### `sikaru channels availability`

Availability

`GET /v1/projects/{project_id}/channels/availability`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

#### `sikaru channels bindings`

Bindings

`GET /v1/projects/{project_id}/channels/bindings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-id` | `string` | Yes |  |

#### `sikaru channels complete`

Complete

`POST /v1/projects/{project_id}/channels/slack/oauth/complete`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru channels configure-feedback`

Configure Feedback

`PATCH /v1/projects/{project_id}/channels/bindings/{binding_id}/feedback`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru channels configure-http-binding`

Configure Http Binding

`PATCH /v1/projects/{project_id}/channels/bindings/{binding_id}/configuration`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru channels configure-personal-access`

Configure

`PUT /v1/projects/{project_id}/channels/bindings/{binding_id}/personal-access`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru channels create-binding`

Create Binding

`POST /v1/projects/{project_id}/channels/bindings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru channels create-identity-app`

Create

`POST /v1/projects/{project_id}/channels/identity-apps`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru channels create-personal-slack-binding`

Customer Dm

`POST /v1/projects/{project_id}/channels/personal-slack-bindings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru channels delete-http-binding`

Delete Http Binding

`DELETE /v1/projects/{project_id}/channels/bindings/{binding_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru channels delivery`

Delivery

`GET /v1/projects/{project_id}/channels/runs/{run_id}/delivery`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--run-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru channels disconnect`

Disconnect

`POST /v1/projects/{project_id}/channels/slack/installations/{installation_id}/disconnect`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--installation-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru channels dm-status`

Dm Status

`GET /v1/projects/{project_id}/channels/slack/dm-verifications/{verification_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--verification-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru channels get-creation-resume`

Get Creation Resume

`GET /v1/projects/{project_id}/channels/creation-resumes/{resume_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--resume-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru channels get-member-slack-link`

Member Link Status

`GET /v1/projects/{project_id}/channels/bindings/{binding_id}/slack-link-verifications/{verification_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--verification-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru channels installations`

Installations

`GET /v1/projects/{project_id}/channels/slack/installations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

#### `sikaru channels issue-http-credential`

Issue Http Credential

`POST /v1/projects/{project_id}/channels/bindings/{binding_id}/credential`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru channels list-identity-apps`

List Apps

`GET /v1/projects/{project_id}/channels/identity-apps`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

#### `sikaru channels recent-receipts`

Recent Receipts

`GET /v1/projects/{project_id}/channels/bindings/{binding_id}/receipts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru channels resend-delivery`

Resend Delivery

`POST /v1/projects/{project_id}/channels/runs/{run_id}/delivery/resend`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--run-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru channels revoke-identity-app`

Revoke

`DELETE /v1/projects/{project_id}/channels/identity-apps/{app_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--app-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru channels revoke-subject`

Revoke Subject

`POST /v1/projects/{project_id}/channels/identity-apps/{app_id}/subjects/revoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--app-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru channels rooms`

Rooms

`GET /v1/projects/{project_id}/channels/slack/installations/{installation_id}/rooms`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--installation-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru channels save-creation-resume`

Save Creation Resume

`POST /v1/projects/{project_id}/channels/creation-resumes`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru channels start`

Start

`POST /v1/projects/{project_id}/channels/slack/oauth/start`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru channels start-dm`

Start Dm

`POST /v1/projects/{project_id}/channels/slack/installations/{installation_id}/dm-verifications`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--installation-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru channels start-member-slack-link`

Link

`POST /v1/projects/{project_id}/channels/bindings/{binding_id}/slack-links`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru channels status`

Status

`PATCH /v1/projects/{project_id}/channels/bindings/{binding_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru channels unlink-member-slack-identity`

Member Unlink

`DELETE /v1/projects/{project_id}/channels/bindings/{binding_id}/slack-links/{installation_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--installation-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru channels update-identity-app-urls`

Update Urls

`PATCH /v1/projects/{project_id}/channels/identity-apps/{app_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--app-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru chat completions`

#### `sikaru chat completions create`

Chat

`POST /v1/chat/completions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru checks`

#### `sikaru checks create`

Same key and definition return the same check; a changed definition conflicts.

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/checks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru checks list`

List Checks

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/checks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |

#### `sikaru checks list-results`

List Check Results

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/checks/{check_id}/results`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--check-id` | `string` | Yes |  |

#### `sikaru checks run`

Start the check's task as a real run of the agent's active release; the result settles later.

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/checks/{check_id}/runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--check-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru completions`

#### `sikaru completions create`

Completions

`POST /v1/completions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru compute-attachments`

#### `sikaru compute-attachments abandon`

Irreversibly abandon execution authority, preserving unknown effects. Cleanup is still required.

`POST /v1/projects/{project_id}/compute-attachments/{attachment_id}/abandon`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru compute-attachments cancel`

Idempotently request stopping. This never asserts child cleanup.

`POST /v1/projects/{project_id}/compute-attachments/{attachment_id}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |

#### `sikaru compute-attachments claim`

Same attachment/key returns the same launch identity, never another launch.

`POST /v1/projects/{project_id}/compute-attachments/{attachment_id}/claim`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru compute-attachments cleanup`

Cleanup Compute Executor

`POST /v1/projects/{project_id}/compute-attachments/{attachment_id}/cleanup`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru compute-attachments connect`

Attest original workspace/journal. Reconnection never replays uncertain effects.

`POST /v1/projects/{project_id}/compute-attachments/{attachment_id}/connect`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru compute-attachments create`

Same session/key/workspace returns the original binding; changed workspace conflicts.

`POST /v1/projects/{project_id}/execution-sessions/{session_id}/compute-attachments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru compute-attachments get`

Get Compute Attachment

`GET /v1/projects/{project_id}/compute-attachments/{attachment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |

#### `sikaru compute-attachments heartbeat`

Heartbeat Compute Executor

`POST /v1/projects/{project_id}/compute-attachments/{attachment_id}/heartbeat`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |

#### `sikaru compute-attachments issue-credential`

Issue Compute Executor Credential

`POST /v1/projects/{project_id}/compute-attachments/{attachment_id}/credentials`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru compute-attachments ready`

Ready Compute Executor

`POST /v1/projects/{project_id}/compute-attachments/{attachment_id}/ready`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru compute-attachments reconcile`

Reconcile Compute Executor

`POST /v1/projects/{project_id}/compute-attachments/{attachment_id}/reconcile`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru compute-attachments status`

Compute Executor Status

`GET /v1/projects/{project_id}/compute-attachments/{attachment_id}/status`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |

#### `sikaru compute-attachments stop`

Executor can stop its own attachment and cancel its runs; cleanup is separately reported.

`POST /v1/projects/{project_id}/compute-attachments/{attachment_id}/stop`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |

#### `sikaru compute-attachments teardown`

Controller attests sandbox teardown; clean parked turns stay resumable.

`POST /v1/projects/{project_id}/compute-attachments/{attachment_id}/teardown`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru compute-credentials`

#### `sikaru compute-credentials renew`

Renew Compute Credential

`POST /v1/projects/{project_id}/compute-credentials/renew`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

#### `sikaru compute-credentials revoke`

Revoke Compute Credential

`POST /v1/projects/{project_id}/compute-credentials/{credential_id}/revoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--credential-id` | `string` | Yes |  |

---

### `sikaru compute-environments`

#### `sikaru compute-environments create`

Same key + same configuration returns the same environment; changed input conflicts.

`POST /v1/projects/{project_id}/compute-environments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru compute-environments disable`

Idempotently disable future claims and credentials; existing cleanup remains required.

`POST /v1/projects/{project_id}/compute-environments/{environment_id}/disable`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--environment-id` | `string` | Yes |  |

#### `sikaru compute-environments get`

Get Compute Environment

`GET /v1/projects/{project_id}/compute-environments/{environment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--environment-id` | `string` | Yes |  |

---

### `sikaru compute-operations`

#### `sikaru compute-operations poll`

Poll Compute Work

`GET /v1/projects/{project_id}/compute-attachments/{attachment_id}/work`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |
| `--wait-seconds` | `integer` | No |  |
| `--limit` | `integer` | No |  |

#### `sikaru compute-operations submit-receipt`

Only exact immutable receipt retries are idempotent; changed content conflicts.

A 409 is final for this receipt: it can never be recorded as sent (for example a
receipt a waiting harness already took). Executors settle delivery and keep the
receipt to answer a later issue of the same operation; they never retry a 409.
Network failures, 429 and 5xx are transient and may be retried.

`POST /v1/projects/{project_id}/compute-attachments/{attachment_id}/receipts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru compute-workspaces`

#### `sikaru compute-workspaces policy`

Read the pinned publication rules before capturing workspace files.

`GET /v1/projects/{project_id}/compute-attachments/{attachment_id}/workspace-policy`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |
| `--run-id` | `string` | No |  |

#### `sikaru compute-workspaces record`

Record a verified uploaded manifest as this session's workspace checkpoint.

`POST /v1/projects/{project_id}/compute-attachments/{attachment_id}/workspace-checkpoints`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru compute-workspaces upload`

Upload file or manifest bytes before acknowledging a workspace checkpoint.

`PUT /v1/projects/{project_id}/compute-attachments/{attachment_id}/workspace-content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--attachment-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru connections`

#### `sikaru connections authorize`

Authorize

`POST /v1/projects/{project_id}/connections/{connection_id}/authorize`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru connections complete`

Complete

`POST /v1/projects/{project_id}/connections/{connection_id}/complete`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru connections create-connection`

Create Connection

`POST /v1/projects/{project_id}/connections`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru connections credentials`

Credentials

`PUT /v1/projects/{project_id}/connections/{connection_id}/credentials`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru connections disable`

Disable

`POST /v1/projects/{project_id}/connections/{connection_id}/disable`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru connections disconnect`

Disconnect

`POST /v1/projects/{project_id}/connections/{connection_id}/disconnect`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru connections discover`

Retry loading tools. Tools load automatically after sign-in and on creation; the outcome is recorded in `tool_load`.

`POST /v1/projects/{project_id}/connections/{connection_id}/discover`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru connections enable`

Enable

`POST /v1/projects/{project_id}/connections/{connection_id}/enable`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru connections events`

Events

`GET /v1/projects/{project_id}/connections/{connection_id}/events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--cursor` | `string` | No |  |
| `--limit` | `integer` | No |  |

#### `sikaru connections get-connection`

Get Connection

`GET /v1/projects/{project_id}/connections/{connection_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru connections grant`

Grant

`POST /v1/projects/{project_id}/connections/{connection_id}/grants`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru connections list-apps`

Catalog Apps

`GET /v1/projects/{project_id}/connections/catalog/apps`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--search` | `string` | No |  |
| `--category` | `string` | No |  |
| `--cursor` | `string` | No |  |
| `--limit` | `integer` | No |  |

#### `sikaru connections list-connections`

List Connections

`GET /v1/projects/{project_id}/connections`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

#### `sikaru connections revoke`

Revoke

`POST /v1/projects/{project_id}/connections/{connection_id}/revoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru connections revoke-grant`

Revoke Grant

`DELETE /v1/projects/{project_id}/connections/{connection_id}/grants/{grant_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--grant-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

#### `sikaru connections update-connection`

Update Connection

`PATCH /v1/projects/{project_id}/connections/{connection_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru connections usage`

Usage

`GET /v1/projects/{project_id}/connections/{connection_id}/usage`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |

---

### `sikaru context-registry`

#### `sikaru context-registry create-context-registry-change`

Create Context Registry Change

`POST /v1/projects/{project_id}/context-registry`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru conversations`

#### `sikaru conversations list-messages`

List Messages

`GET /v1/projects/{project_id}/conversations/{conversation_id}/messages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--conversation-id` | `string` | Yes |  |
| `--account-id` | `string` | Yes |  |
| `--limit` | `integer` | No |  |
| `--cursor` | `string` | No |  |

#### `sikaru conversations record-message`

Record Message

`POST /v1/projects/{project_id}/conversations/{conversation_id}/messages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--conversation-id` | `string` | Yes |  |
| `--account-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru credit-top-ups`

#### `sikaru credit-top-ups create`

Offer a credit top-up for this project's organization; an agent pays `payUrl` from its own wallet.

`POST /v1/projects/{project_id}/credit-top-ups`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru credit-top-ups get`

Read a top-up offer's status.

`GET /v1/projects/{project_id}/credit-top-ups/{top_up_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--top-up-id` | `string` | Yes |  |

#### `sikaru credit-top-ups get-balance`

Read the credits available to this project's organization and the largest top-up an agent can offer.

`GET /v1/projects/{project_id}/credits`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

---

### `sikaru datasets`

#### `sikaru datasets capture-into-dataset`

Capture Into Dataset

`POST /v1/projects/{project_id}/datasets/{dataset_id}/capture`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--dataset-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru datasets capture-into-new-dataset`

Creates the named dataset and adds the runs to it; a retry of the key reuses both.

`POST /v1/projects/{project_id}/datasets/capture`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru datasets create-dataset`

Create Dataset

`POST /v1/projects/{project_id}/datasets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru datasets delete-dataset`

Delete Dataset

`DELETE /v1/projects/{project_id}/datasets/{dataset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--dataset-id` | `string` | Yes |  |

#### `sikaru datasets delete-example`

Permanently removes every copy of the example, including from older versions.

`DELETE /v1/projects/{project_id}/datasets/{dataset_id}/examples/{example_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--dataset-id` | `string` | Yes |  |
| `--example-id` | `string` | Yes |  |

#### `sikaru datasets export-dataset`

Export Dataset

`GET /v1/projects/{project_id}/datasets/{dataset_id}/export`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--dataset-id` | `string` | Yes |  |
| `--version` | `string` | No |  |

#### `sikaru datasets get-dataset`

Get Dataset

`GET /v1/projects/{project_id}/datasets/{dataset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--dataset-id` | `string` | Yes |  |

#### `sikaru datasets list-dataset-check-results`

Pass rate per agent version for each dataset version run as Checks; failures name their example and run.

`GET /v1/projects/{project_id}/datasets/{dataset_id}/checks/results`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--dataset-id` | `string` | Yes |  |
| `--version` | `string` | No |  |

#### `sikaru datasets list-datasets`

List Datasets

`GET /v1/projects/{project_id}/datasets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--purpose` | `string` | No |  |

#### `sikaru datasets list-examples`

List Examples

`GET /v1/projects/{project_id}/datasets/{dataset_id}/examples`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--dataset-id` | `string` | Yes |  |
| `--version` | `string` | No |  |
| `--limit` | `integer` | No |  |
| `--offset` | `integer` | No |  |

#### `sikaru datasets list-versions`

List Versions

`GET /v1/projects/{project_id}/datasets/{dataset_id}/versions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--dataset-id` | `string` | Yes |  |

#### `sikaru datasets preview-dataset-checks`

How many examples can run as Checks, why the rest cannot, and the estimated model cost on ``agent``.

`GET /v1/projects/{project_id}/datasets/{dataset_id}/checks/preview`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--dataset-id` | `string` | Yes |  |
| `--agent` | `string` | No |  |
| `--version` | `string` | No |  |

#### `sikaru datasets start-dataset-checks`

One Check per eligible example, each started as a production run of the agent's live version.

Credits are admitted before anything is created. A retry of the same key
returns the same checks and starts only what did not start before.

`POST /v1/projects/{project_id}/datasets/{dataset_id}/checks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--dataset-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru datasets update-dataset`

Update Dataset

`PATCH /v1/projects/{project_id}/datasets/{dataset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--dataset-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru datasets update-example`

Editing the expected answer or tags creates a version; edited fields are human-written.

Sikaru's own tags survive a tag edit unless it comes from the signed-in dashboard, which shows them.

`PATCH /v1/projects/{project_id}/datasets/{dataset_id}/examples/{example_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--dataset-id` | `string` | Yes |  |
| `--example-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru datasets upload-examples`

Upload Examples

`POST /v1/projects/{project_id}/datasets/{dataset_id}/uploads`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--dataset-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru decisions`

#### `sikaru decisions create`

Answer questions about a state

`POST /v1/systemone`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru deployments`

#### `sikaru deployments list-console-deployments`

List Console Deployments

`GET /v1/projects/{project_id}/deployments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

---

### `sikaru end-users`

#### `sikaru end-users correct-persona-summary`

Correct the persona summary. The correction applies from the next turn and is never relearned away.

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/end-users/{tenant_id}/{user_id}/persona-summary/corrections`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--tenant-id` | `string` | Yes |  |
| `--user-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru end-users delete`

Purge this user from every agent of the product, with every trace or eval case they contributed.

`DELETE /v1/projects/{project_id}/managed-agents/{agent_slug}/end-users/{tenant_id}/{user_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--tenant-id` | `string` | Yes |  |
| `--user-id` | `string` | Yes |  |
| `--terminal` | `boolean` | No | Permanently reject cell creation and new work for this end user. |

#### `sikaru end-users get`

What the agent knows about this end user: the persona summary, never the persona prompt.

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/end-users/{tenant_id}/{user_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--tenant-id` | `string` | Yes |  |
| `--user-id` | `string` | Yes |  |

#### `sikaru end-users get-learning-run`

Get End User Learning

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/end-users/{tenant_id}/{user_id}/learning-runs/{invocation_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--tenant-id` | `string` | Yes |  |
| `--user-id` | `string` | Yes |  |
| `--invocation-id` | `string` | Yes |  |

#### `sikaru end-users import-state`

Import a versioned product snapshot atomically, preserving correction provenance.

The cell must already exist. Every file requires expectedEtag (null for an
absent destination). Reusing an importId requires the identical manifest.
The digest is SHA-256 of canonical JSON {sourceRevision, files}, with sorted
keys, compact separators and the supplied file order. Private layers are
accepted as input but are never returned in the receipt or end-user view.

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/end-users/{tenant_id}/{user_id}/imports`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--tenant-id` | `string` | Yes |  |
| `--user-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru end-users revoke-learning-source`

Revoke End User Learning Source

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/end-users/{tenant_id}/{user_id}/learning-sources/{source_id}/revoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--tenant-id` | `string` | Yes |  |
| `--user-id` | `string` | Yes |  |
| `--source-id` | `string` | Yes |  |

#### `sikaru end-users start-learning-run`

Start End User Learning

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/end-users/{tenant_id}/{user_id}/learning-runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--tenant-id` | `string` | Yes |  |
| `--user-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru end-users upsert`

Create this end user's private cell, or update its timezone and population opt-in.

`PUT /v1/projects/{project_id}/managed-agents/{agent_slug}/end-users/{tenant_id}/{user_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--tenant-id` | `string` | Yes |  |
| `--user-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru environments`

#### `sikaru environments create-managed-environment`

Create Managed Environment

`POST /v1/projects/{project_id}/environments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru environments list-managed-environments`

List Managed Environments

`GET /v1/projects/{project_id}/environments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

---

### `sikaru evaluation-comparisons`

#### `sikaru evaluation-comparisons cancel-comparison`

Cancel Comparison

`POST /v1/projects/{project_id}/evaluation-comparisons/{comparison_id}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--comparison-id` | `string` | Yes |  |

#### `sikaru evaluation-comparisons create-comparison`

Create Comparison

`POST /v1/projects/{project_id}/evaluation-comparisons`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru evaluation-comparisons get-comparison`

Get Comparison

`GET /v1/projects/{project_id}/evaluation-comparisons/{comparison_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--comparison-id` | `string` | Yes |  |

#### `sikaru evaluation-comparisons list-comparisons`

List Comparisons

`GET /v1/projects/{project_id}/evaluation-comparisons`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--after` | `string` | No |  |

---

### `sikaru evaluation-criteria`

#### `sikaru evaluation-criteria list-criteria`

List Criteria

`GET /v1/projects/{project_id}/evaluation-criteria`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--after` | `string` | No |  |

---

### `sikaru evaluation-jobs`

#### `sikaru evaluation-jobs cancel-job`

Cancel Job

`POST /v1/projects/{project_id}/evaluation-jobs/{job_id}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--job-id` | `string` | Yes |  |

#### `sikaru evaluation-jobs create-job`

Create Job

`POST /v1/projects/{project_id}/evaluation-jobs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru evaluation-jobs get-job`

Get Job

`GET /v1/projects/{project_id}/evaluation-jobs/{job_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--job-id` | `string` | Yes |  |

#### `sikaru evaluation-jobs list-jobs`

List Jobs

`GET /v1/projects/{project_id}/evaluation-jobs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--cursor` | `string` | No |  |

---

### `sikaru evaluation-results`

#### `sikaru evaluation-results list-results`

List Results

`GET /v1/projects/{project_id}/evaluation-results`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--environment` | `production | staging | development` | No |  |
| `--evaluator` | `string` | No |  |
| `--verdict` | `string` | No |  |
| `--limit` | `integer` | No |  |
| `--cursor` | `string` | No |  |

#### `sikaru evaluation-results record-result`

Record Result

`POST /v1/projects/{project_id}/evaluation-results`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru evaluator-runs`

#### `sikaru evaluator-runs create-evaluator-run`

Create Evaluator Run

`POST /v1/projects/{project_id}/evaluator-runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru execution-objectives`

#### `sikaru execution-objectives cancel`

Cancel

`POST /v1/projects/{project_id}/execution-objectives/{objective_id}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--objective-id` | `string` | Yes |  |

#### `sikaru execution-objectives create`

Create

`POST /v1/projects/{project_id}/execution-objectives`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru execution-objectives get`

Get

`GET /v1/projects/{project_id}/execution-objectives/{objective_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--objective-id` | `string` | Yes |  |

#### `sikaru execution-objectives list-objectives`

List Objectives

`GET /v1/projects/{project_id}/execution-objectives`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | No |  |
| `--status` | `string` | No |  |
| `--after` | `string` | No |  |
| `--limit` | `integer` | No |  |

#### `sikaru execution-objectives pause`

Pause

`POST /v1/projects/{project_id}/execution-objectives/{objective_id}/pause`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--objective-id` | `string` | Yes |  |

#### `sikaru execution-objectives resume`

Resume

`POST /v1/projects/{project_id}/execution-objectives/{objective_id}/resume`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--objective-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `sikaru execution-sessions`

#### `sikaru execution-sessions append-turn`

Append Turn

`POST /v1/projects/{project_id}/execution-sessions/{session_id}/turns`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru execution-sessions branch`

Branch Session

`POST /v1/projects/{project_id}/execution-sessions/{session_id}/branches`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru execution-sessions create`

Create Session

`POST /v1/projects/{project_id}/harnesses/{harness_id}/execution-sessions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru execution-sessions delete-file`

Delete File

`DELETE /v1/projects/{project_id}/execution-sessions/{session_id}/files/{file_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--file-id` | `string` | Yes |  |

#### `sikaru execution-sessions download-file`

Download File

`GET /v1/projects/{project_id}/execution-sessions/{session_id}/files/{file_id}/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--file-id` | `string` | Yes |  |

#### `sikaru execution-sessions get`

Get Session

`GET /v1/projects/{project_id}/execution-sessions/{session_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |

#### `sikaru execution-sessions list`

List Sessions

`GET /v1/projects/{project_id}/execution-sessions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | No |  |
| `--after` | `string` | No |  |
| `--limit` | `integer` | No |  |
| `--agent-slug` | `string` | No |  |

#### `sikaru execution-sessions list-eligible-releases`

List release identities eligible for an optional session pin.

`GET /v1/projects/{project_id}/harnesses/{harness_id}/releases`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |
| `--environment` | `production | staging` | No |  |

#### `sikaru execution-sessions list-files`

List Files

`GET /v1/projects/{project_id}/execution-sessions/{session_id}/files`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |

#### `sikaru execution-sessions list-session-inputs`

List Session Inputs

`GET /v1/projects/{project_id}/execution-sessions/{session_id}/inputs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |

#### `sikaru execution-sessions spend`

Session Spend

`GET /v1/projects/{project_id}/execution-sessions/{session_id}/spend`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |

#### `sikaru execution-sessions upload-file`

Upload File

`POST /v1/projects/{project_id}/execution-sessions/{session_id}/files`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--filename` | `string` | Yes |  |
| `--idempotency-key` | `string` | Yes |  |
| `--content-type` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru executions`

#### `sikaru executions execution-runtime-lineage`

Execution Runtime Lineage

`GET /v1/projects/{project_id}/executions/{trace_id}/runtime`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--trace-id` | `string` | Yes |  |
| `--account-id` | `string` | Yes |  |
| `--inference-after` | `string` | No |  |

---

### `sikaru feedback`

#### `sikaru feedback create-feedback`

Create Feedback

`POST /v1/projects/{project_id}/feedback`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru git-credentials`

#### `sikaru git-credentials create`

Store a git credential for one host. The value is write-only.

`POST /v1/projects/{project_id}/git-credentials`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru git-credentials grant`

Let an agent use a git credential for its declared repos.

`POST /v1/projects/{project_id}/git-credentials/{credential_id}/grants`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--credential-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru git-credentials list`

The project's git credentials and their grants, without values.

`GET /v1/projects/{project_id}/git-credentials`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

---

### `sikaru graders`

#### `sikaru graders estimate-cost`

What judging every conversation with the LLM judge or a project agent is expected to cost (R4).

`GET /v1/projects/{project_id}/agents/{agent_id}/graders/{grader_id}/cost-estimate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--grader-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--agent-id` | `string` | Yes |  |
| `--judge` | `llm | agent` | Yes |  |
| `--judge-agent-id` | `string` | No |  |

#### `sikaru graders get`

One Grader: its status, criteria, judge, release check, versions and label counts.

`GET /v1/projects/{project_id}/agents/{agent_id}/graders/{grader_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--grader-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--agent-id` | `string` | Yes |  |

#### `sikaru graders get-run`

Follow a recent-production re-score to completion or a useful failure message.

`GET /v1/projects/{project_id}/agents/{agent_id}/graders/{grader_id}/runs/{run_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--grader-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--agent-id` | `string` | Yes |  |

#### `sikaru graders get-test`

Read a pending, completed or failed unsaved test. Reports expire after one day.

`GET /v1/projects/{project_id}/agents/{agent_id}/graders/tests/{test_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--test-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--agent-id` | `string` | Yes |  |

#### `sikaru graders label`

Mark one conversation Flagged or Looks Fine with an optional reason; labeling it again replaces the label.
The response then shows what the Grader itself said.

`PUT /v1/projects/{project_id}/agents/{agent_id}/graders/{grader_id}/labels`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--grader-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--agent-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru graders label-queue`

Conversations the Grader is least sure about, unlabeled, each with an excerpt and without its verdict.

`GET /v1/projects/{project_id}/agents/{agent_id}/graders/{grader_id}/label-queue`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--grader-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--agent-id` | `string` | Yes |  |
| `--limit` | `integer` | No |  |

#### `sikaru graders list`

The agent's Graders, presets included, and whether the agent is graded at all.

`GET /v1/projects/{project_id}/agents/{agent_id}/graders`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-id` | `string` | Yes |  |

#### `sikaru graders run`

Re-score recent production conversations using this Grader version's own judge.

`POST /v1/projects/{project_id}/agents/{agent_id}/graders/{grader_id}/run`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--grader-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--agent-id` | `string` | Yes |  |
| `--idempotency-key` | `string` | No |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `sikaru graders samples`

The Grader's recent verdicts, flagged conversations first.

`GET /v1/projects/{project_id}/agents/{agent_id}/graders/{grader_id}/samples`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--grader-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--agent-id` | `string` | Yes |  |
| `--limit` | `integer` | No |  |

#### `sikaru graders test`

Score an unsaved file with its own judge. Poll a pending test; no Grader or verdict is saved.

`POST /v1/projects/{project_id}/agents/{agent_id}/graders/test`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-id` | `string` | Yes |  |
| `--idempotency-key` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru graders update`

Change the description, judge, escalation or release check. A judge or escalation change saves a provisional
version that goes live only once it clears the promotion rule; the other settings change no verdict.

`PATCH /v1/projects/{project_id}/agents/{agent_id}/graders/{grader_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--grader-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--agent-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru harnesses`

#### `sikaru harnesses get-improvement`

Get Improvement

`GET /v1/projects/{project_id}/harnesses/{harness_id}/improvements/{job_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |
| `--job-id` | `string` | Yes |  |

#### `sikaru harnesses get-invoice-budget`

Get Invoice Budget

`GET /v1/projects/{project_id}/harnesses/{harness_id}/budget/invoice`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |

#### `sikaru harnesses improvement-options`

Improvement Options

`GET /v1/projects/{project_id}/harnesses/{harness_id}/improvement-options`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |

#### `sikaru harnesses list-improvements`

List Improvements

`GET /v1/projects/{project_id}/harnesses/{harness_id}/improvements`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |
| `--after` | `string` | No |  |
| `--limit` | `integer` | No |  |

#### `sikaru harnesses resume-improvement`

Resume Improvement

`POST /v1/projects/{project_id}/harnesses/{harness_id}/improvements/{job_id}/resume`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |
| `--job-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `sikaru harnesses start-improvement`

Start Improvement

`POST /v1/projects/{project_id}/harnesses/{harness_id}/improvements`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru harnesses train-model-stub`

Reserved, unavailable model-training step; no learning job is submitted.

`POST /v1/projects/{project_id}/harnesses/{harness_id}/training`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |

---

### `sikaru http-channels`

#### `sikaru http-channels invoke`

Invoke

`POST /v1/channel-bindings/{binding_id}/messages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru http-channels poll`

Poll

`GET /v1/channel-bindings/{binding_id}/messages/{receipt_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

#### `sikaru http-channels start-slack-link`

Link

`POST /v1/channel-bindings/{binding_id}/slack-links`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru http-channels unlink-slack-identity`

Unlink

`DELETE /v1/channel-bindings/{binding_id}/slack-links/{installation_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--installation-id` | `string` | Yes |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

---

### `sikaru import-sessions`

#### `sikaru import-sessions create-compatibility-profile`

Create Compatibility Profile

`POST /v1/projects/{project_id}/import-sessions/{import_session_id}/compatibility-profile`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru import-sessions create-import-session`

Create Import Session

`POST /v1/projects/{project_id}/import-sessions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru import-sessions create-replay-run`

Create Replay Run

`POST /v1/projects/{project_id}/import-sessions/{import_session_id}/replay-runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru import-sessions create-source-artifact`

Create Source Artifact

`POST /v1/projects/{project_id}/import-sessions/{import_session_id}/source-artifacts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru import-sessions create-staging-run`

Create Staging Run

`POST /v1/projects/{project_id}/import-sessions/{import_session_id}/staging-runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru import-sessions get-import-session`

Get Import Session

`GET /v1/projects/{project_id}/import-sessions/{import_session_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |

#### `sikaru import-sessions get-parity-report`

Get Parity Report

`GET /v1/projects/{project_id}/import-sessions/{import_session_id}/parity-report`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |

#### `sikaru import-sessions list-import-session-diffs`

List Import Session Diffs

`GET /v1/projects/{project_id}/import-sessions/{import_session_id}/diffs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |

#### `sikaru import-sessions list-import-sessions`

List Import Sessions

`GET /v1/projects/{project_id}/import-sessions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

#### `sikaru import-sessions list-source-artifacts`

List Source Artifacts

`GET /v1/projects/{project_id}/import-sessions/{import_session_id}/source-artifacts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |

#### `sikaru import-sessions promote-import-session`

Promote Import Session

`POST /v1/projects/{project_id}/import-sessions/{import_session_id}/promote`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru issue-clusters`

#### `sikaru issue-clusters get-issue-cluster`

Get Issue Cluster

`GET /v1/projects/{project_id}/issue-clusters/{cluster_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--cluster-id` | `string` | Yes |  |

#### `sikaru issue-clusters list-issue-clusters`

List Issue Clusters

`GET /v1/projects/{project_id}/issue-clusters`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--status` | `string` | No |  |
| `--severity` | `string` | No |  |
| `--agent-id` | `string` | No |  |

#### `sikaru issue-clusters propose-issue-cluster-fix`

Propose Issue Cluster Fix

`POST /v1/projects/{project_id}/issue-clusters/{cluster_id}/propose-fix`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--cluster-id` | `string` | Yes |  |

#### `sikaru issue-clusters update-issue-cluster-status`

Update Issue Cluster Status

`PATCH /v1/projects/{project_id}/issue-clusters/{cluster_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--cluster-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru issue-clusters upsert-issue-cluster`

Upsert Issue Cluster

`POST /v1/projects/{project_id}/issue-clusters`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru managed-agents`

#### `sikaru managed-agents add-agent-login`

An operator signs the agent in to a company account once; every run then uses it.

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/browser-logins`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru managed-agents create-managed-agent`

Create Managed Agent

`POST /v1/projects/{project_id}/managed-agents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru managed-agents delete-agent-login`

Delete Agent Login

`DELETE /v1/projects/{project_id}/managed-agents/{agent_slug}/browser-logins/{login_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--login-id` | `string` | Yes |  |

#### `sikaru managed-agents delete-managed-agent`

Delete an agent that has never gone live, with its drafts, checks and schedules.

`DELETE /v1/projects/{project_id}/managed-agents/{agent_slug}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |

#### `sikaru managed-agents delete-playbook`

Delete Playbook

`DELETE /v1/projects/{project_id}/managed-agents/{agent_slug}/browser-playbooks/{playbook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--playbook-id` | `string` | Yes |  |

#### `sikaru managed-agents delete-rule`

Delete Rule

`DELETE /v1/projects/{project_id}/managed-agents/{agent_slug}/browser-rules/{rule_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--rule-id` | `string` | Yes |  |

#### `sikaru managed-agents get`

One managed agent in the project: the same public summary the list returns.

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |

#### `sikaru managed-agents get-definition-revision`

Get Definition Revision

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/definition-revisions/{changeset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |

#### `sikaru managed-agents list-agent-logins`

List Agent Logins

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/browser-logins`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |

#### `sikaru managed-agents list-managed-agents`

List Managed Agents

`GET /v1/projects/{project_id}/managed-agents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

#### `sikaru managed-agents list-playbooks`

List Playbooks

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/browser-playbooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |

#### `sikaru managed-agents list-rules`

Every always-allow rule on the agent; a rule personal to one end user does not say whose.

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/browser-rules`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |

#### `sikaru managed-agents update`

Change the agent's display name, turn capacity or memory dreaming. The slug and every reference to the agent stay the same.

Personal and end-user-cell choices are fixed at creation.

`PATCH /v1/projects/{project_id}/managed-agents/{agent_slug}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru managed-agents update-end-user-skill-source`

Replace or enable a customer-authored source using its own revision CAS.

Private optimized methodology and optimizer digests are never returned.
Replaying an operation returns its original receipt without reverting later work.

`PUT /v1/projects/{project_id}/managed-agents/{agent_slug}/end-users/{tenant_id}/{user_id}/skill-sources/{skill_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--tenant-id` | `string` | Yes |  |
| `--user-id` | `string` | Yes |  |
| `--skill-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru memory`

#### `sikaru memory delete-file`

Delete Memory File

`DELETE /v1/projects/{project_id}/managed-agents/{agent_slug}/memory/files/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--path` | `string` | Yes | Path inside the directory, such as MEMORY.md or notes/trip.md. |
| `--base` | `string` | Yes | The head the deletion was made from. |
| `--directory` | `agent | user` | No | The shared agent directory, or one end user's directory. |
| `--environment` | `published | draft` | No |  |
| `--tenant-id` | `string` | No |  |
| `--user-id` | `string` | No |  |

#### `sikaru memory dream`

Dream now: consolidate this directory without waiting for idle time, even with automatic dreaming off.

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/memory/dreams`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--directory` | `agent | user` | No | The shared agent directory, or one end user's directory. |
| `--environment` | `published | draft` | No |  |
| `--tenant-id` | `string` | No |  |
| `--user-id` | `string` | No |  |

#### `sikaru memory forget`

Remove something from this user's memory, its retained history and past-session search, in the background.

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/memory/forgets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--directory` | `agent | user` | No | The shared agent directory, or one end user's directory. |
| `--environment` | `published | draft` | No |  |
| `--tenant-id` | `string` | No |  |
| `--user-id` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru memory get-dream`

The directory's latest dream job.

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/memory/dreams`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--directory` | `agent | user` | No | The shared agent directory, or one end user's directory. |
| `--environment` | `published | draft` | No |  |
| `--tenant-id` | `string` | No |  |
| `--user-id` | `string` | No |  |

#### `sikaru memory get-forget`

Get Memory Forget

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/memory/forgets/{job_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--job-id` | `string` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--directory` | `agent | user` | No | The shared agent directory, or one end user's directory. |
| `--environment` | `published | draft` | No |  |
| `--tenant-id` | `string` | No |  |
| `--user-id` | `string` | No |  |

#### `sikaru memory list-files`

The directory's files and its current head.

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/memory/files`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--directory` | `agent | user` | No | The shared agent directory, or one end user's directory. |
| `--environment` | `published | draft` | No |  |
| `--tenant-id` | `string` | No |  |
| `--user-id` | `string` | No |  |

#### `sikaru memory list-forgets`

List Memory Forgets

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/memory/forgets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--directory` | `agent | user` | No | The shared agent directory, or one end user's directory. |
| `--environment` | `published | draft` | No |  |
| `--tenant-id` | `string` | No |  |
| `--user-id` | `string` | No |  |

#### `sikaru memory list-versions`

List Memory Versions

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/memory/versions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--directory` | `agent | user` | No | The shared agent directory, or one end user's directory. |
| `--environment` | `published | draft` | No |  |
| `--tenant-id` | `string` | No |  |
| `--user-id` | `string` | No |  |

#### `sikaru memory move-file`

Move or rename one file; the destination must not exist.

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/memory/files/move`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--directory` | `agent | user` | No | The shared agent directory, or one end user's directory. |
| `--environment` | `published | draft` | No |  |
| `--tenant-id` | `string` | No |  |
| `--user-id` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru memory read-file`

Read Memory File

`GET /v1/projects/{project_id}/managed-agents/{agent_slug}/memory/files/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--path` | `string` | Yes | Path inside the directory, such as MEMORY.md or notes/trip.md. |
| `--directory` | `agent | user` | No | The shared agent directory, or one end user's directory. |
| `--environment` | `published | draft` | No |  |
| `--tenant-id` | `string` | No |  |
| `--user-id` | `string` | No |  |

#### `sikaru memory remember`

Remember again: finished forgets of this target stop hiding it from future search and dreams.

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/memory/remember`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--directory` | `agent | user` | No | The shared agent directory, or one end user's directory. |
| `--environment` | `published | draft` | No |  |
| `--tenant-id` | `string` | No |  |
| `--user-id` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru memory rollback`

Make a retained version the head again; the rollback is kept as the newest version.

`POST /v1/projects/{project_id}/managed-agents/{agent_slug}/memory/versions/{version}/rollback`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--version` | `integer` | Yes |  |
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--directory` | `agent | user` | No | The shared agent directory, or one end user's directory. |
| `--environment` | `published | draft` | No |  |
| `--tenant-id` | `string` | No |  |
| `--user-id` | `string` | No |  |

#### `sikaru memory write-file`

Create or replace one file.

`PUT /v1/projects/{project_id}/managed-agents/{agent_slug}/memory/files/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-slug` | `string` | Yes |  |
| `--directory` | `agent | user` | No | The shared agent directory, or one end user's directory. |
| `--environment` | `published | draft` | No |  |
| `--tenant-id` | `string` | No |  |
| `--user-id` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru model-gateway`

#### `sikaru model-gateway capture-model-gateway-chat-completion`

Capture Model Gateway Chat Completion

`POST /v1/projects/{project_id}/model-gateway/{provider}/chat/completions/capture`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--provider` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru model-settings`

#### `sikaru model-settings get-model-settings`

Get Model Settings

`GET /v1/projects/{project_id}/model-settings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

#### `sikaru model-settings update-model-settings`

Update Model Settings

`PUT /v1/projects/{project_id}/model-settings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru models`

#### `sikaru models list`

Models

`GET /v1/models`

#### `sikaru models retrieve`

Model

`GET /v1/models/{model}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--model` | `string` | Yes |  |

---

### `sikaru personal-channels`

#### `sikaru personal-channels authorize-connection`

Authorize

`POST /v1/personal-channel-bindings/{binding_id}/messages/{receipt_id}/connections/{connection_id}/authorize`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

#### `sikaru personal-channels complete-connection`

Complete

`POST /v1/personal-channel-bindings/{binding_id}/messages/{receipt_id}/connections/{connection_id}/complete`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru personal-channels create-browser-handoff`

A one-time link to sign in, in a live browser, to the sites a parked browser call needs.

`POST /v1/personal-channel-bindings/{binding_id}/messages/{receipt_id}/tool-calls/{tool_call_id}/browser-handoff`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--tool-call-id` | `string` | Yes |  |
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

#### `sikaru personal-channels decide-approval`

Approve

`POST /v1/personal-channel-bindings/{binding_id}/messages/{receipt_id}/tool-calls/{tool_call_id}/approval`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--tool-call-id` | `string` | Yes |  |
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru personal-channels delete-browser-login`

Delete Browser Login

`DELETE /v1/personal-channel-bindings/{binding_id}/messages/{receipt_id}/browser-logins/{login_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--login-id` | `string` | Yes |  |
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

#### `sikaru personal-channels delete-browser-rule`

Delete Browser Rule

`DELETE /v1/personal-channel-bindings/{binding_id}/messages/{receipt_id}/browser-rules/{rule_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--rule-id` | `string` | Yes |  |
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

#### `sikaru personal-channels download-file`

Download

`GET /v1/personal-channel-bindings/{binding_id}/messages/{receipt_id}/files/{file_id}/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--file-id` | `string` | Yes |  |
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

#### `sikaru personal-channels get-browser-preview`

Everything this user is asked to approve, typed values included.

`GET /v1/personal-channel-bindings/{binding_id}/messages/{receipt_id}/browser-previews/{digest}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--digest` | `string` | Yes |  |
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

#### `sikaru personal-channels get-browser-preview-screenshot`

The page this user is asked to approve, password fields masked.

`GET /v1/personal-channel-bindings/{binding_id}/messages/{receipt_id}/browser-previews/{digest}/screenshot`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--digest` | `string` | Yes |  |
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

#### `sikaru personal-channels get-browser-receipt`

What this run's browser did for this user, with their own pages' screenshots.

`GET /v1/personal-channel-bindings/{binding_id}/messages/{receipt_id}/browser-receipt`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

#### `sikaru personal-channels get-browser-receipt-screenshot`

Browser Receipt Screenshot

`GET /v1/personal-channel-bindings/{binding_id}/messages/{receipt_id}/browser-receipt/screenshots/{digest}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--digest` | `string` | Yes |  |
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

#### `sikaru personal-channels get-message`

Message

`GET /v1/personal-channel-bindings/{binding_id}/messages/{receipt_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

#### `sikaru personal-channels get-slack-link`

Status

`GET /v1/personal-channel-bindings/{binding_id}/slack-link-verifications/{verification_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--verification-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

#### `sikaru personal-channels list-browser-logins`

The sites this user's agents may open signed in as them.

`GET /v1/personal-channel-bindings/{binding_id}/messages/{receipt_id}/browser-logins`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

#### `sikaru personal-channels list-browser-rules`

The browser actions this user always allows this agent to take for them.

`GET /v1/personal-channel-bindings/{binding_id}/messages/{receipt_id}/browser-rules`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

#### `sikaru personal-channels list-files`

Files

`GET /v1/personal-channel-bindings/{binding_id}/messages/{receipt_id}/files`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

#### `sikaru personal-channels replace-connection-credentials`

Credentials

`PUT /v1/personal-channel-bindings/{binding_id}/messages/{receipt_id}/connections/{connection_id}/credentials`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--connection-id` | `string` | Yes |  |
| `--binding-id` | `string` | Yes |  |
| `--receipt-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru personal-channels start-slack-link`

Start

`POST /v1/personal-channel-bindings/{binding_id}/slack-links`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru personal-channels unlink-slack-identity`

Unlink

`DELETE /v1/personal-channel-bindings/{binding_id}/slack-links/{installation_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--binding-id` | `string` | Yes |  |
| `--installation-id` | `string` | Yes |  |
| `--authorization` | `string` | No |  |
| `--x-sikaru-user-assertion` | `string` | No |  |
| `--x-sikaru-member-authorization` | `string` | No |  |

---

### `sikaru projects`

#### `sikaru projects list-projects`

The caller's organization and projects; a signed-in bearer's first call sets them up.

`GET /v1/projects`

---

### `sikaru release-watches`

#### `sikaru release-watches create-release-watch`

Create Release Watch

`POST /v1/projects/{project_id}/release-watches`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru responses`

#### `sikaru responses create`

Responses

`POST /v1/responses`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru retention-policies`

#### `sikaru retention-policies create-retention-policy-update`

Create Retention Policy Update

`POST /v1/projects/{project_id}/retention-policies`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru review-queue`

#### `sikaru review-queue create-review-queue-item`

Create Review Queue Item

`POST /v1/projects/{project_id}/review-queue`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru run-references`

#### `sikaru run-references resolve-run-reference`

Resolve Run Reference

`GET /v1/projects/{project_id}/run-references/{reference}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--reference` | `string` | Yes |  |
| `--as-of` | `string` | No |  |

---

### `sikaru run-schedules`

#### `sikaru run-schedules create-schedule`

Create Schedule

`POST /v1/projects/{project_id}/run-schedules`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru run-schedules delete-schedule`

Delete Schedule

`DELETE /v1/projects/{project_id}/run-schedules/{schedule_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--schedule-id` | `string` | Yes |  |
| `--expected-revision` | `string` | No |  |
| `--idempotency-key` | `string` | No |  |

#### `sikaru run-schedules inspect-schedule`

Inspect Schedule

`GET /v1/projects/{project_id}/run-schedules/{schedule_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--schedule-id` | `string` | Yes |  |

#### `sikaru run-schedules list-schedules`

List Schedules

`GET /v1/projects/{project_id}/run-schedules`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | No |  |
| `--agent-slug` | `string` | No |  |

#### `sikaru run-schedules pause-schedule`

Pause Schedule

`PATCH /v1/projects/{project_id}/run-schedules/{schedule_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--schedule-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru run-schedules schedule-history`

Schedule History

`GET /v1/projects/{project_id}/run-schedules/{schedule_id}/occurrences`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--schedule-id` | `string` | Yes |  |
| `--before` | `string` | No |  |
| `--limit` | `integer` | No |  |

#### `sikaru run-schedules schedule-notices`

Schedule Notices

`GET /v1/projects/{project_id}/run-schedules/{schedule_id}/notices`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--schedule-id` | `string` | Yes |  |

#### `sikaru run-schedules update-schedule`

Update Schedule

`PUT /v1/projects/{project_id}/run-schedules/{schedule_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--schedule-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru run-webhooks`

#### `sikaru run-webhooks create-webhook`

Create Webhook

`POST /v1/projects/{project_id}/run-webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru run-webhooks delete-webhook`

Delete Webhook

`DELETE /v1/projects/{project_id}/run-webhooks/{webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--webhook-id` | `string` | Yes |  |

#### `sikaru run-webhooks list-webhooks`

List Webhooks

`GET /v1/projects/{project_id}/run-webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

---

### `sikaru runs`

#### `sikaru runs cancel`

Cancel Run

`POST /v1/projects/{project_id}/runs/{run_id}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |

#### `sikaru runs create-handoff`

Create Handoff

`POST /v1/projects/{project_id}/runs/{run_id}/browser/handoffs/{tool_call_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--tool-call-id` | `string` | Yes |  |

#### `sikaru runs decide-approval`

Decide Approval

`POST /v1/projects/{project_id}/runs/{run_id}/tool-calls/{tool_call_id}/approval`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--tool-call-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru runs end-take-over`

Withdraw a take over, or hand back the browser the run is waiting on.

`DELETE /v1/projects/{project_id}/runs/{run_id}/browser/takeover`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |

#### `sikaru runs events`

Managed Run Events

`GET /v1/projects/{project_id}/runs/{run_id}/events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--after` | `string` | No |  |
| `--limit` | `string` | No |  |

#### `sikaru runs get`

Managed Run Summary

`GET /v1/projects/{project_id}/runs/{run_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |

#### `sikaru runs get-preview`

Everything a browser approval asks about, typed values included, for its owner only.

`GET /v1/projects/{project_id}/runs/{run_id}/browser/previews/{digest}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--digest` | `string` | Yes |  |

#### `sikaru runs get-receipt`

What the run's browser did: pages visited, actions committed, and sign-ins by people.

`GET /v1/projects/{project_id}/runs/{run_id}/browser/receipt`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |

#### `sikaru runs get-receipt-screenshot`

Get Receipt Screenshot

`GET /v1/projects/{project_id}/runs/{run_id}/browser/receipt/screenshots/{digest}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--digest` | `string` | Yes |  |

#### `sikaru runs get-trajectory`

Read retained ATIF structure and usage with private content redacted.

This is a committed snapshot and can be partial while a run is active or
interrupted. Messages, reasoning, tool payloads and provider metadata are
omitted. No trajectory is synthesized when retained evidence is unavailable.

`GET /v1/projects/{project_id}/runs/{run_id}/trajectory`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |

#### `sikaru runs get-transcript`

Managed Run Transcript

`GET /v1/projects/{project_id}/runs/{run_id}/transcript`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |

#### `sikaru runs pending-actions`

Pending Actions

`GET /v1/projects/{project_id}/runs/{run_id}/actions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |

#### `sikaru runs preview-screenshot`

The page an approver is asked about, password fields masked.

`GET /v1/projects/{project_id}/runs/{run_id}/browser/previews/{digest}/screenshot`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--digest` | `string` | Yes |  |

#### `sikaru runs record`

Record input and output, optionally deduplicating retries with Idempotency-Key.

Reusing a key with different content returns 409. No execution timing is
inferred; the run is ordered by when Sikaru receives it.

`POST /v1/projects/{project_id}/runs/record`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--idempotency-key` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru runs recover`

Recover Managed Run

`POST /v1/projects/{project_id}/runs/{run_id}/recover`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru runs resume`

Resume the same retained turn. A refused request does not change its state.

`POST /v1/projects/{project_id}/runs/{run_id}/resume`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |

#### `sikaru runs resume-all`

Recheck up to 100 authorized needs-action runs and jobs; only cleared stops resume.

`POST /v1/projects/{project_id}/runs/resume`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru runs resume-count`

Count needs-action work the caller can control, grouped by funding reason.

`GET /v1/projects/{project_id}/runs/resume/count`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

#### `sikaru runs retry`

Retry a failed turn as a new turn with its own run ID; the failed run stays failed. The retry
starts from the session's last completed turn, and external effects the failed turn already
completed are not repeated. Repeating the request returns the same retry. A run that did not
fail, whose `error.recoverable` is false, or that failed too long ago is refused with 409
`not_retryable`.

`POST /v1/projects/{project_id}/runs/{run_id}/retry`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |

#### `sikaru runs start`

Start Managed Harness Run

`POST /v1/projects/{project_id}/harnesses/{harness_id}/runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru runs stream-events`

Replay and stream durable run events; resume using after or Last-Event-ID.

A terminal run event ends the stream. Idle streams may also close: callers
can resume from their last delivered event without restarting the run.

`GET /v1/projects/{project_id}/runs/{run_id}/events/stream`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--after` | `string` | No |  |
| `--last-event-id` | `string` | No |  |

#### `sikaru runs stream-transcript-events`

Managed Transcript Stream

`GET /v1/projects/{project_id}/runs/{run_id}/transcript/events/stream`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--after` | `integer` | No |  |
| `--last-event-id` | `string` | No |  |

#### `sikaru runs submit-tool-result`

Submit Tool Result

`POST /v1/projects/{project_id}/runs/{run_id}/tool-results`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru runs take-over`

Take over a run's browser after the agent's current action; the link drives it until handed back.

`POST /v1/projects/{project_id}/runs/{run_id}/browser/takeover`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `sikaru runs watch-run`

Watch a run's browser read-only. Pages signed in with an end user's own login stay hidden
from everyone but that user.

`POST /v1/projects/{project_id}/runs/{run_id}/browser/watch`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |

---

### `sikaru sessions`

#### `sikaru sessions get-managed-session`

Get Managed Session

`GET /v1/projects/{project_id}/sessions/{session_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |

#### `sikaru sessions list-managed-session-events`

List Managed Session Events

`GET /v1/projects/{project_id}/sessions/{session_id}/events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--after` | `string` | No |  |
| `--limit` | `string` | No |  |

#### `sikaru sessions list-managed-session-files`

List Managed Session Files

`GET /v1/projects/{project_id}/sessions/{session_id}/files`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |

#### `sikaru sessions list-managed-session-plan`

List Managed Session Plan

`GET /v1/projects/{project_id}/sessions/{session_id}/plan`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |

---

### `sikaru tasks`

#### `sikaru tasks actions`

Task Actions

`GET /v1/projects/{project_id}/execution-sessions/{session_id}/tasks/{task_id}/actions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--task-id` | `string` | Yes |  |

#### `sikaru tasks approve-action`

Approve Task Action

`POST /v1/projects/{project_id}/execution-sessions/{session_id}/tasks/{task_id}/actions/{action_id}/approval`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--task-id` | `string` | Yes |  |
| `--action-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru tasks cancel`

Cancel Task

`POST /v1/projects/{project_id}/execution-sessions/{session_id}/tasks/{task_id}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--task-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru tasks changes`

Task Changes

`GET /v1/projects/{project_id}/execution-sessions/{session_id}/tasks/{task_id}/changes`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--task-id` | `string` | Yes |  |

#### `sikaru tasks get`

Get Task

`GET /v1/projects/{project_id}/execution-sessions/{session_id}/tasks/{task_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--task-id` | `string` | Yes |  |

#### `sikaru tasks integrate`

Integrate Task

`POST /v1/projects/{project_id}/execution-sessions/{session_id}/tasks/{task_id}/integrations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--task-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru tasks integrations`

Task Integrations

`GET /v1/projects/{project_id}/execution-sessions/{session_id}/tasks/{task_id}/integrations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--task-id` | `string` | Yes |  |

#### `sikaru tasks list`

List Tasks

`GET /v1/projects/{project_id}/execution-sessions/{session_id}/tasks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |

#### `sikaru tasks message`

Message Task

`POST /v1/projects/{project_id}/execution-sessions/{session_id}/tasks/{task_id}/messages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--task-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru tasks stop-all`

Stop Background Tasks

`POST /v1/projects/{project_id}/execution-sessions/{session_id}/tasks/stop-all`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru tool-providers`

#### `sikaru tool-providers apply-tool-provider`

Create the provider for this capability prefix, or converge it to the request.

Re-sending the same declaration changes nothing. Changing the provider type or
credential owner is refused with `state_conflict`.

`PUT /v1/projects/{project_id}/tool-providers/{capability_prefix}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--capability-prefix` | `string` | Yes | The provider's capability prefix; it never changes. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru tool-providers attach-source-tool-skill`

Attach Source Tool Skill

`POST /v1/projects/{project_id}/tool-providers/{capability_prefix}/skills`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--capability-prefix` | `string` | Yes | The provider's capability prefix; it never changes. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru tool-providers expire-previous-tool-provider-secret`

Stop accepting the previous delivery secret now. Without one this changes nothing.

`DELETE /v1/projects/{project_id}/tool-providers/{capability_prefix}/secret/previous`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--capability-prefix` | `string` | Yes | The provider's capability prefix; it never changes. |

#### `sikaru tool-providers get-tool-provider`

One tool provider with its delivery mode and secret state.

`GET /v1/projects/{project_id}/tool-providers/{capability_prefix}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--capability-prefix` | `string` | Yes | The provider's capability prefix; it never changes. |

#### `sikaru tool-providers list-tool-providers`

The project's tool providers, optionally narrowed to one capability prefix or status.

`GET /v1/projects/{project_id}/tool-providers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--capability-prefix` | `string` | No |  |
| `--status` | `string` | No |  |

#### `sikaru tool-providers rotate-tool-provider-secret`

Make a new delivery secret current; the old one stays valid for the overlap.

`POST /v1/projects/{project_id}/tool-providers/{capability_prefix}/secret/rotate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--capability-prefix` | `string` | Yes | The provider's capability prefix; it never changes. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru trace-import-connections`

#### `sikaru trace-import-connections list-trace-import-connections`

List Trace Import Connections

`GET /v1/projects/{project_id}/trace-import-connections`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

---

### `sikaru trace-imports`

#### `sikaru trace-imports cancel-trace-import`

Cancel Trace Import

`POST /v1/projects/{project_id}/trace-imports/{trace_import_id}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--trace-import-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru trace-imports create-trace-import`

Create Trace Import

`POST /v1/projects/{project_id}/trace-imports`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru trace-imports get-trace-import`

Get Trace Import

`GET /v1/projects/{project_id}/trace-imports/{trace_import_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--trace-import-id` | `string` | Yes |  |

#### `sikaru trace-imports get-trace-import-receipt`

Get Trace Import Receipt

`GET /v1/projects/{project_id}/trace-imports/{trace_import_id}/receipt`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--trace-import-id` | `string` | Yes |  |

#### `sikaru trace-imports list-trace-imports`

List Trace Imports

`GET /v1/projects/{project_id}/trace-imports`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

#### `sikaru trace-imports plan-trace-import`

Plan Trace Import

`POST /v1/projects/{project_id}/trace-imports/plan`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru trace-imports retry-trace-import`

Retry Trace Import

`POST /v1/projects/{project_id}/trace-imports/{trace_import_id}/retry`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--trace-import-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru trace-streams`

#### `sikaru trace-streams stream-openinference-spans`

Stream Openinference Spans

`POST /v1/trace-streams`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--idempotency-key` | `string` | No |  |
| `--x-sikaru-client-id` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru workflow-intents`

#### `sikaru workflow-intents compile-project-workflow-intent`

Compile Project Workflow Intent

`POST /v1/projects/{project_id}/workflow-intents/{intent_id}/compile`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--intent-id` | `string` | Yes |  |

#### `sikaru workflow-intents create-project-workflow-intent`

Create Project Workflow Intent

`POST /v1/projects/{project_id}/workflow-intents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru workflow-runs`

#### `sikaru workflow-runs project-workflow-run-events`

Project Workflow Run Events

`GET /v1/projects/{project_id}/workflow-runs/{run_id}/events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |

#### `sikaru workflow-runs recover-project-workflow-run`

Recover Project Workflow Run

`POST /v1/projects/{project_id}/workflow-runs/{run_id}/recover`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru workflows`

#### `sikaru workflows create-project-workflow-version`

Create Project Workflow Version

`POST /v1/projects/{project_id}/workflows/{workflow_id}/versions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--workflow-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru workflows export-product-workflow`

Export Product Workflow

`GET /v1/projects/{project_id}/workflows/{workflow_id}/export`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--workflow-id` | `string` | Yes |  |

#### `sikaru workflows import-workflow`

Import Workflow

`POST /v1/projects/{project_id}/workflows/import`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru workflows start-project-workflow-run`

Start Project Workflow Run

`POST /v1/projects/{project_id}/workflows/{workflow_id}/runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--workflow-id` | `string` | Yes |  |
| `--idempotency-key` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

## Global flags

These flags are available on every command:

| Flag | Description |
|------|-------------|
| `--dry-run` | Print the HTTP request without sending it |
| `--json <JSON\|->` | Supply the request body as JSON (or `-` for stdin) |
| `--params <JSON>` | Merge extra parameters as JSON |
| `--format <json\|table\|yaml\|csv>` | Output format (default: `json`) |
| `--output <PATH>` | Write binary responses to a file |
| `--base-url <URL>` | Override the API base URL |
| `--page-all` | Auto-paginate and stream all results |
| `--page-limit <N>` | Max pages to fetch (default: `10`) |
| `-q, --quiet` | Suppress stdout on success |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

