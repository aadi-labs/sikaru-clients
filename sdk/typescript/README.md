# Sikaru TypeScript Library

[![fern shield](https://img.shields.io/badge/%F0%9F%8C%BF-Built%20with%20Fern-brightgreen)](https://buildwithfern.com?utm_source=github&utm_medium=github&utm_campaign=readme&utm_source=Sikaru%2FTypeScript)
[![npm shield](https://img.shields.io/npm/v/@aadi-labs/sikaru-sdk)](https://www.npmjs.com/package/@aadi-labs/sikaru-sdk)

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
- [Errors](#errors)
- [Retries and Timeouts](#retries-and-timeouts)
- [For Coding Agents](#for-coding-agents)
- [Learn More](#learn-more)
- [Advanced](#advanced)
  - [Additional Headers](#additional-headers)
  - [Additional Query String Parameters](#additional-query-string-parameters)
  - [Aborting Requests](#aborting-requests)
  - [Access Raw Response Data](#access-raw-response-data)
  - [Logging](#logging)
  - [Custom Fetch](#custom-fetch)

## Documentation

API reference documentation is available [here](https://docs.sikaru.ai/api-reference).

## Installation

```sh
npm i -s @aadi-labs/sikaru-sdk
```

## Reference

A full reference for this library is available [here](./reference.md).

## Quickstart

Set `SIKARU_API_KEY` to a project API key from the Sikaru dashboard, then call a model. Model IDs take the `lab/model` form. Requires Node.js 18 or later. Keep the key in server-side code; never ship it in a browser bundle.

```typescript
import { Sikaru } from "@aadi-labs/sikaru-sdk/inference";

const client = new Sikaru(); // reads SIKARU_API_KEY

const completion = await client.chat.completions.create({
    model: "z-ai/glm-5.3-flash",
    messages: [{ role: "user", content: "Explain a rainbow in one sentence." }],
});
console.log(completion.choices?.[0]?.message?.content);
```

`Sikaru` takes the same request shape as the OpenAI SDK on `chat.completions`, `completions` and `responses`. List the catalog with `client.models.list()`, or omit `model` to use your project's default. Every other API resource, such as `runs` and `memory`, is on the same client. See [Inference](https://docs.sikaru.ai/inference/inference-api) for how each request field is handled.


## Run an agent

Start a run of a published agent and follow its events. Set `SIKARU_PROJECT_ID` to your project ID.

```typescript
import { randomUUID } from "node:crypto";
import { Sikaru } from "@aadi-labs/sikaru-sdk/inference";

const client = new Sikaru();
const project = process.env.SIKARU_PROJECT_ID!;

const run = await client.runs.start(project, "support-quickstart", {
    tenant_id: "demo-customer",
    user_id: "demo-user",
    idempotency_key: randomUUID(),
    input: { text: "Summarize this case: an order has two charges and no transaction IDs." },
    product_context: {},
    policy: {},
});

for await (const event of await client.runs.streamEvents(project, run.runId)) {
    console.log(event.eventType, event.payload);
}

console.log((await client.runs.get(project, run.runId)).status);
```

`runs.start` confirms that the run was accepted, not that it finished. A terminal event ends the stream, but an idle stream can also close, so read the run's status before treating a closed stream as done. To retry a start that timed out, send the same idempotency key and input. The [quickstart](https://docs.sikaru.ai/get-started/quickstart) creates the agent and walks through the full loop.


## Tool providers

A tool provider is your backend's registration as the executor of product-owned tools: Sikaru sends it each tool request an agent makes, signed, and your backend performs the action. The full guide, including Terraform secrets and the request lifecycle, is [Tool providers](https://docs.sikaru.ai/build/tool-providers).

**Declare it on every deploy.** A provider is addressed by its capability prefix, a constant in your code, so your pipeline never stores an ID. The call creates the provider or updates it in place and returns the same `id` each time; an identical declaration changes nothing.

```typescript
import { Sikaru } from "@aadi-labs/sikaru-sdk/inference";

const client = new Sikaru();

const provider = await client.toolProviders.applyToolProvider(process.env.SIKARU_PROJECT_ID!, "crm", {
    provider_type: "custom",
    display_name: "CRM",
    tool_catalog_ref: "custom://crm/tools",
    broker_endpoint_ref: "https://tools.example.com/sikaru",
    secret: process.env.CRM_DELIVERY_SECRET, // whsec_ + base64 of 24-64 random bytes
});
console.log(provider.id, provider.deliveryMode, provider.secretState?.fingerprint);
```

Generate the secret yourself (`echo "whsec_$(openssl rand -base64 32)"`, or `random_bytes` in Terraform) so every deploy can send it. Omit `secret` and Sikaru generates one, returned only in `provider.secret` on that first call. Reads never return a secret: compare `secretState.fingerprint` with your copy instead.

**Verify every delivery before acting on it.** Pass the raw body exactly as received, the request headers, and every secret you currently accept. `verify` checks the signature in constant time and the timestamp within five minutes, then resolves to the parsed body. It uses Web Crypto, so the same handler runs on Node.js 20+, Bun, Deno and edge runtimes:

```typescript
import { verify, WebhookVerificationError } from "@aadi-labs/sikaru-sdk/webhooks";

const accepted = [process.env.CRM_DELIVERY_SECRET, process.env.CRM_DELIVERY_SECRET_PREVIOUS]
    .filter((secret): secret is string => Boolean(secret));

export async function POST(request: Request): Promise<Response> {
    let delivery: unknown;
    try {
        delivery = await verify(await request.text(), request.headers, accepted);
    } catch (error) {
        if (error instanceof WebhookVerificationError) {
            return new Response(null, { status: 401 }); // unsigned, tampered, stale or unknown secret
        }
        throw error;
    }
    // Deduplicate on request.headers.get("webhook-id"): retries reuse it.
    // Store the delivery durably, return 2xx, then run the tool and submit its result.
    return new Response(null, { status: 200 });
}
```

Because the receiver accepts a list, rotating never drops a delivery: add the new secret to the list, send it in the next declaration (the old one stays valid for 24 hours), then remove the old one. `client.toolProviders.listToolProviders(project)` and `getToolProvider(project, "crm")` read providers with a `tool_providers:read` key. See [Rotate without downtime](https://docs.sikaru.ai/build/tool-providers#rotate-without-downtime).


## Streaming

Pass `stream: true` to get an async iterator of chunks. Streamed chat and completions responses end with a usage chunk.

```typescript
const stream = await client.chat.completions.create({
    model: "z-ai/glm-5.3-flash",
    messages: [{ role: "user", content: "Write a haiku about rain." }],
    stream: true,
});
for await (const chunk of stream) {
    process.stdout.write(chunk.choices?.[0]?.delta?.content ?? "");
}
```

Run events stream the same way with `client.runs.streamEvents(project, runId)`.


## Errors

Every non-success response throws `SikaruError`, or a typed subclass such as `PaymentRequiredError` or `TooManyRequestsError` from the package's `Sikaru` namespace export. A request that exceeds its timeout throws `SikaruTimeoutError`.

```typescript
import { SikaruError } from "@aadi-labs/sikaru-sdk";

try {
    await client.chat.completions.create({
        model: "z-ai/glm-5.3-flash",
        messages: [{ role: "user", content: "Hello" }],
    });
} catch (err) {
    if (err instanceof SikaruError) {
        console.log(err.statusCode, err.body);
    }
}
```

Inference errors use the OpenAI envelope, `{"error": {"message", "type", "param", "code"}}`, and `param` names the field at fault. Other endpoints return `{"detail", "code"}`. Branch on `code`. A `429` or `503` response carries `Retry-After` in seconds. A failed run is not an error response: read `error.code` on the run itself.


## Retries and timeouts

Only reads (GET and HEAD) retry automatically: up to 2 times by default on rate limits, timeouts and server errors, with exponential backoff that honors `Retry-After`. Requests that change state, including inference calls and `runs.start`, are never replayed for you. Retry them yourself with the same idempotency key where the operation takes one.

Requests time out after 60 seconds by default. Set `timeoutInSeconds` and `maxRetries` on the client, or override them per request:

```typescript
const client = new Sikaru({ timeoutInSeconds: 20, maxRetries: 0 });
const run = await client.runs.get(project, runId, { timeoutInSeconds: 10 });
```

A request timeout limits one HTTP call. It does not cancel a hosted run.


## For coding agents

- Read [docs.sikaru.ai/llms.txt](https://docs.sikaru.ai/llms.txt) for an index of every guide and API operation. Append `.md` to any docs page URL for plain Markdown, and add `?lang=typescript` to `llms.txt` to keep only TypeScript examples.
- Exact request and response fields are in the [OpenAPI schema](https://docs.sikaru.ai/openapi.json). Use only operations listed there.
- The `sikaru` CLI covers the same API from a shell. `sikaru inference chat z-ai/glm-5.3-flash "Hello" --output json` prints one structured result, and `--help` lists the commands at every level. Install it with `npm install -g @aadi-labs/sikaru-cli`.
- Address a tool provider by its capability prefix, never a stored ID, and run every delivery through `verify` from `@aadi-labs/sikaru-sdk/webhooks` before acting on it. Never log a delivery secret.
- Keep `SIKARU_API_KEY` out of code, commits and logs. An accepted run is not a finished run: read its status until it completes, fails or is cancelled.


## Learn more

- [Documentation](https://docs.sikaru.ai): guides for inference, managed agents, memory, graders and evaluations.
- [SDK configuration](https://docs.sikaru.ai/sdks-and-cli/sdk-configuration): timeouts, errors and pagination.
- [Method reference](https://github.com/aadi-labs/sikaru-clients/blob/main/sdk/typescript/reference.md): every client method with its parameters.
- [Release notes](https://docs.sikaru.ai/sdks-and-cli/release-notes): what changed in each version. The SDKs and the CLI ship together at one version.
- [Issues](https://github.com/aadi-labs/sikaru-clients/issues): report a bug with the client version, the operation name and a minimal reproduction. Redact keys and private data.


## Advanced

### Additional Headers

If you would like to send additional headers as part of the request, use the `headers` request option.

```typescript
import { SikaruApi } from "@aadi-labs/sikaru-sdk";

const client = new SikaruApi({
    ...
    headers: {
        'X-Custom-Header': 'custom value'
    }
});

const response = await client.chat.completions.create(..., {
    headers: {
        'X-Custom-Header': 'custom value'
    }
});
```

### Additional Query String Parameters

If you would like to send additional query string parameters as part of the request, use the `queryParams` request option.

```typescript
const response = await client.chat.completions.create(..., {
    queryParams: {
        'customQueryParamKey': 'custom query param value'
    }
});
```

### Aborting Requests

The SDK allows users to abort requests at any point by passing in an abort signal.

```typescript
const controller = new AbortController();
const response = await client.chat.completions.create(..., {
    abortSignal: controller.signal
});
controller.abort(); // aborts the request
```

### Access Raw Response Data

The SDK provides access to raw response data, including headers, through the `.withRawResponse()` method.
The `.withRawResponse()` method returns a promise that results to an object with a `data` and a `rawResponse` property.

```typescript
const { data, rawResponse } = await client.chat.completions.create(...).withRawResponse();

console.log(data);
console.log(rawResponse.headers['X-My-Header']);
```

### Logging

The SDK supports logging. You can configure the logger by passing in a `logging` object to the client options.

```typescript
import { SikaruApi, logging } from "@aadi-labs/sikaru-sdk";

const client = new SikaruApi({
    ...
    logging: {
        level: logging.LogLevel.Debug, // defaults to logging.LogLevel.Info
        logger: new logging.ConsoleLogger(), // defaults to ConsoleLogger
        silent: false, // defaults to true, set to false to enable logging
    }
});
```
The `logging` object can have the following properties:
- `level`: The log level to use. Defaults to `logging.LogLevel.Info`.
- `logger`: The logger to use. Defaults to a `logging.ConsoleLogger`.
- `silent`: Whether to silence the logger. Defaults to `true`.

The `level` property can be one of the following values:
- `logging.LogLevel.Debug`
- `logging.LogLevel.Info`
- `logging.LogLevel.Warn`
- `logging.LogLevel.Error`

To provide a custom logger, you can pass in an object that implements the `logging.ILogger` interface.

<details>
<summary>Custom logger examples</summary>

Here's an example using the popular `winston` logging library.
```ts
import winston from 'winston';

const winstonLogger = winston.createLogger({...});

const logger: logging.ILogger = {
    debug: (msg, ...args) => winstonLogger.debug(msg, ...args),
    info: (msg, ...args) => winstonLogger.info(msg, ...args),
    warn: (msg, ...args) => winstonLogger.warn(msg, ...args),
    error: (msg, ...args) => winstonLogger.error(msg, ...args),
};
```

Here's an example using the popular `pino` logging library.

```ts
import pino from 'pino';

const pinoLogger = pino({...});

const logger: logging.ILogger = {
  debug: (msg, ...args) => pinoLogger.debug(args, msg),
  info: (msg, ...args) => pinoLogger.info(args, msg),
  warn: (msg, ...args) => pinoLogger.warn(args, msg),
  error: (msg, ...args) => pinoLogger.error(args, msg),
};
```
</details>


### Custom Fetch

The SDK provides a low-level `fetch` method for making custom HTTP requests while still
benefiting from SDK-level configuration like authentication, retries, timeouts, and logging.
This is useful for calling API endpoints not yet supported in the SDK.

```typescript
const response = await client.fetch("/v1/custom/endpoint", {
    method: "GET",
}, {
    timeoutInSeconds: 30,
    maxRetries: 3,
    headers: {
        "X-Custom-Header": "custom-value",
    },
});

const data = await response.json();
```

