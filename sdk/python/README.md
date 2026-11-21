# Sikaru Python Library

[![fern shield](https://img.shields.io/badge/%F0%9F%8C%BF-Built%20with%20Fern-brightgreen)](https://buildwithfern.com?utm_source=github&utm_medium=github&utm_campaign=readme&utm_source=Sikaru%2FPython)
[![pypi](https://img.shields.io/pypi/v/sikaru_sdk)](https://pypi.python.org/pypi/sikaru_sdk)

[Sikaru](https://sikaru.ai) gives you three things behind one API: inference on a catalog of models through OpenAI-compatible endpoints, managed agents that Sikaru runs and keeps state for, and a lab that grades, evaluates and improves those agents from their traces. This package is the official client for that API.

Documentation: [docs.sikaru.ai](https://docs.sikaru.ai) · [API reference](https://docs.sikaru.ai/api-reference) · [Release notes](https://docs.sikaru.ai/sdks-and-cli/release-notes) · [Docs index for AI agents](https://docs.sikaru.ai/llms.txt)


## Table of Contents

- [Documentation](#documentation)
- [Installation](#installation)
- [Reference](#reference)
- [Quickstart](#quickstart)
- [Run An Agent](#run-an-agent)
- [Tool Providers](#tool-providers)
- [Streaming](#streaming)
- [Async Client](#async-client)
- [Errors](#errors)
- [Retries and Timeouts](#retries-and-timeouts)
- [For Coding Agents](#for-coding-agents)
- [Learn More](#learn-more)
- [Advanced](#advanced)
  - [Access Raw Response Data](#access-raw-response-data)
  - [Custom Client](#custom-client)

## Documentation

API reference documentation is available [here](https://docs.sikaru.ai/api-reference).

## Installation

```sh
pip install sikaru_sdk
```

## Reference

A full reference for this library is available [here](./reference.md).

## Quickstart

Set `SIKARU_API_KEY` to a project API key from the Sikaru dashboard, then call a model. Model IDs take the `lab/model` form. Requires Python 3.10 or later; with uv, install with `uv add sikaru-sdk`.

```python
from sikaru_sdk import Sikaru

client = Sikaru()  # reads SIKARU_API_KEY

completion = client.chat.completions.create(
    model="z-ai/glm-5.3-flash",
    messages=[{"role": "user", "content": "Explain a rainbow in one sentence."}],
)
print(completion.choices[0].message.content)
```

`Sikaru` takes the same request shape as the OpenAI SDK on `chat.completions`, `completions` and `responses`. List the catalog with `client.models.list()`, or omit `model` to use your project's default. Every other API resource, such as `runs` and `memory`, is on the same client. See [Inference](https://docs.sikaru.ai/inference/inference-api) for how each request field is handled.


## Run an agent

Start a run of a published agent and follow its events. Set `SIKARU_PROJECT_ID` to your project ID.

```python
import os
import uuid

from sikaru_sdk import Sikaru

client = Sikaru()
project = os.environ["SIKARU_PROJECT_ID"]

run = client.runs.start(
    project,
    "support-quickstart",  # the agent's slug
    tenant_id="demo-customer",
    user_id="demo-user",
    idempotency_key=str(uuid.uuid4()),
    input={"text": "Summarize this case: an order has two charges and no transaction IDs."},
    product_context={},
    policy={},
)

for event in client.runs.stream_events(project, run.run_id):
    print(event.event_type, event.payload)

print(client.runs.get(project, run.run_id).status)
```

`runs.start` confirms that the run was accepted, not that it finished. A terminal event ends the stream, but an idle stream can also close, so read the run's status before treating a closed stream as done. To retry a start that timed out, send the same idempotency key and input. The [quickstart](https://docs.sikaru.ai/get-started/quickstart) creates the agent and walks through the full loop.


## Tool providers

A tool provider is your backend's registration as the executor of product-owned tools: Sikaru sends it each tool request an agent makes, signed, and your backend performs the action. The full guide, including Terraform secrets and the request lifecycle, is [Tool providers](https://docs.sikaru.ai/build/tool-providers).

**Declare it on every deploy.** A provider is addressed by its capability prefix, a constant in your code, so your pipeline never stores an ID. The call creates the provider or updates it in place and returns the same `id` each time; an identical declaration changes nothing.

```python
import os

from sikaru_sdk import Sikaru

client = Sikaru()

provider = client.tool_providers.apply_tool_provider(
    os.environ["SIKARU_PROJECT_ID"],
    "crm",  # capability prefix
    provider_type="custom",
    display_name="CRM",
    tool_catalog_ref="custom://crm/tools",
    broker_endpoint_ref="https://tools.example.com/sikaru",
    secret=os.environ["CRM_DELIVERY_SECRET"],  # whsec_ + base64 of 24-64 random bytes
)
print(provider.id, provider.delivery_mode, provider.secret_state.fingerprint)
```

Generate the secret yourself (`echo "whsec_$(openssl rand -base64 32)"`, or `random_bytes` in Terraform) so every deploy can send it. Omit `secret` and Sikaru generates one, returned only in `provider.secret` on that first call. Reads never return a secret: compare `secret_state.fingerprint` with your copy instead.

**Verify every delivery before acting on it.** Pass the raw body exactly as received, the request headers, and every secret you currently accept. `verify` checks the signature in constant time and the timestamp within five minutes, then returns the parsed body:

```python
import os

from fastapi import FastAPI, Request, Response
from sikaru_sdk.webhooks import WebhookVerificationError, verify

app = FastAPI()
accepted = [s for s in (os.environ["CRM_DELIVERY_SECRET"], os.environ.get("CRM_DELIVERY_SECRET_PREVIOUS")) if s]


@app.post("/sikaru")
async def receive(request: Request) -> Response:
    try:
        delivery = verify(await request.body(), request.headers, accepted)
    except WebhookVerificationError:
        return Response(status_code=401)  # unsigned, tampered, stale or unknown secret
    # Deduplicate on request.headers["webhook-id"]: retries reuse it.
    # Store the delivery durably, return 2xx, then run the tool and submit its result.
    return Response(status_code=200)
```

Because the receiver accepts a list, rotating never drops a delivery: add the new secret to the list, send it in the next declaration (the old one stays valid for 24 hours), then remove the old one. `client.tool_providers.list_tool_providers(project)` and `get_tool_provider(project, "crm")` read providers with a `tool_providers:read` key. See [Rotate without downtime](https://docs.sikaru.ai/build/tool-providers#rotate-without-downtime).


## Streaming

Pass `stream=True` to get an iterator of chunks. Streamed chat and completions responses end with a usage chunk.

```python
stream = client.chat.completions.create(
    model="z-ai/glm-5.3-flash",
    messages=[{"role": "user", "content": "Write a haiku about rain."}],
    stream=True,
)
for chunk in stream:
    if chunk.choices and chunk.choices[0].delta:
        print(chunk.choices[0].delta.content or "", end="")
    if chunk.usage:
        print("\ntokens:", chunk.usage.total_tokens)
```

Run events stream the same way with `client.runs.stream_events(project, run_id)`.


## Async client

`AsyncSikaru` has the same methods. Await a call to get its result, or a stream when you pass `stream=True`.

```python
import asyncio

from sikaru_sdk import AsyncSikaru


async def main() -> None:
    client = AsyncSikaru()
    completion = await client.chat.completions.create(
        model="z-ai/glm-5.3-flash",
        messages=[{"role": "user", "content": "Explain a rainbow in one sentence."}],
    )
    print(completion.choices[0].message.content)


asyncio.run(main())
```


## Errors

Every non-success response raises `ApiError`, or a typed subclass such as `PaymentRequiredError` or `TooManyRequestsError` exported from `sikaru_sdk`.

```python
from sikaru_sdk.core.api_error import ApiError

try:
    client.chat.completions.create(
        model="z-ai/glm-5.3-flash",
        messages=[{"role": "user", "content": "Hello"}],
    )
except ApiError as error:
    print(error.status_code, error.body)
```

Inference errors use the OpenAI envelope, `{"error": {"message", "type", "param", "code"}}`, and `param` names the field at fault. Other endpoints return `{"detail", "code"}`. Branch on `code`. A `429` or `503` response carries `Retry-After` in seconds. A failed run is not an error response: read `error.code` on the run itself.


## Retries and timeouts

Only reads (GET and HEAD) retry automatically: up to 2 times by default on rate limits, timeouts and server errors, with exponential backoff that honors `Retry-After`. Requests that change state, including inference calls and `runs.start`, are never replayed for you. Retry them yourself with the same idempotency key where the operation takes one.

Requests time out after 60 seconds by default. Set `timeout` (seconds) and `max_retries` on the client, or override them per request:

```python
client = Sikaru(timeout=20.0, max_retries=0)
run = client.runs.get(project, run_id, request_options={"timeout": 10})
```

A request timeout limits one HTTP call. It does not cancel a hosted run.


## For coding agents

- Read [docs.sikaru.ai/llms.txt](https://docs.sikaru.ai/llms.txt) for an index of every guide and API operation. Append `.md` to any docs page URL for plain Markdown, and add `?lang=python` to `llms.txt` to keep only Python examples.
- Exact request and response fields are in the [OpenAPI schema](https://docs.sikaru.ai/openapi.json). Use only operations listed there.
- The `sikaru` CLI covers the same API from a shell. `sikaru inference chat z-ai/glm-5.3-flash "Hello" --output json` prints one structured result, and `--help` lists the commands at every level. Install it with `curl -fsSL https://app.sikaru.ai/install.sh | sh`.
- Address a tool provider by its capability prefix, never a stored ID, and run every delivery through `sikaru_sdk.webhooks.verify` before acting on it. Never log a delivery secret.
- Keep `SIKARU_API_KEY` out of code, commits and logs. An accepted run is not a finished run: read its status until it completes, fails or is cancelled.


## Learn more

- [Documentation](https://docs.sikaru.ai): guides for inference, managed agents, memory, graders and evaluations.
- [SDK configuration](https://docs.sikaru.ai/sdks-and-cli/sdk-configuration): timeouts, errors, async usage and pagination.
- [Method reference](https://github.com/aadi-labs/sikaru-clients/blob/main/sdk/python/reference.md): every client method with its parameters.
- [Release notes](https://docs.sikaru.ai/sdks-and-cli/release-notes): what changed in each version. The SDKs and the CLI ship together at one version.
- [Issues](https://github.com/aadi-labs/sikaru-clients/issues): report a bug with the client version, the operation name and a minimal reproduction. Redact keys and private data.


## Advanced

### Access Raw Response Data

The SDK provides access to raw response data, including headers, through the `.with_raw_response` property.
The `.with_raw_response` property returns a "raw" client that can be used to access the `.headers` and `.data` attributes.

```python
from sikaru_sdk import SikaruApi

client = SikaruApi(...)
response = client.chat.completions.with_raw_response.create(...)
print(response.headers)  # access the response headers
print(response.status_code)  # access the response status code
print(response.data)  # access the underlying object
```

### Custom Client

You can override the `httpx` client to customize it for your use-case. Some common use-cases include support for proxies
and transports.

```python
import httpx
from sikaru_sdk import SikaruApi

client = SikaruApi(
    ...,
    httpx_client=httpx.Client(
        proxy="http://my.test.proxy.example.com",
        transport=httpx.HTTPTransport(local_address="0.0.0.0"),
    ),
)
```

