// Hand-written next to the generated client, which owns every other file.
//
// `Sikaru` is the generated `SikaruApi` client, except that `chat.completions`,
// `completions` and `responses` accept `stream: true` on `create`, as the OpenAI SDK
// does. Each call dispatches to the generated `create` or `createStream`, so request
// fields, request options and transport stay the generated client's own.

import type * as Api from "./api/index.js";
import type { ChatClient } from "./api/resources/chat/client/Client.js";
import type { CompletionsClient as ChatCompletionsClient } from "./api/resources/chat/resources/completions/client/Client.js";
import type { CompletionsClient } from "./api/resources/completions/client/Client.js";
import type { ResponsesClient } from "./api/resources/responses/client/Client.js";
import { SikaruApi } from "./Client.js";
import type * as core from "./core/index.js";

/** A request that streams: the call resolves to a stream of events. */
export type Streamed<Request> = Request & { stream: true };
/** A request that does not stream: the call resolves to the complete result. */
export type Unstreamed<Request> = Request & { stream?: false | null };

export interface StreamingCreate<Request, Options, Result, Event> {
    create(request: Streamed<Request>, requestOptions?: Options): core.HttpResponsePromise<core.Stream<Event>>;
    create(request: Unstreamed<Request>, requestOptions?: Options): core.HttpResponsePromise<Result>;
}

export type ChatCompletions = StreamingCreate<
    Api.chat.ChatCompletionCreateParams,
    ChatCompletionsClient.RequestOptions,
    Api.ChatCompletion,
    Api.ChatCompletionChunk
> &
    ChatCompletionsClient;

export type Completions = StreamingCreate<
    Api.CompletionCreateParams,
    CompletionsClient.RequestOptions,
    Api.Completion,
    Api.Completion
> &
    CompletionsClient;

export type Responses = StreamingCreate<
    Api.ResponseCreateParams,
    ResponsesClient.RequestOptions,
    Api.ModelResponse,
    Api.ResponseStreamEvent
> &
    ResponsesClient;

export type Chat = { readonly completions: ChatCompletions } & ChatClient;

interface GeneratedCreate {
    create(request: object, requestOptions?: object): unknown;
    createStream(request: object, requestOptions?: object): unknown;
}

/** The generated resource with `create(stream)`; every other member is inherited unchanged. */
function streamingCreate<Wrapped>(generated: GeneratedCreate): Wrapped {
    const create = (request: { stream?: boolean | null }, requestOptions?: object): unknown => {
        const { stream, ...rest } = request;
        return stream ? generated.createStream(rest, requestOptions) : generated.create(rest, requestOptions);
    };
    return Object.create(generated, { create: { value: create } });
}

/** `SikaruApi` with OpenAI-shaped `create({ stream })`; construct it exactly like `SikaruApi`. */
export class Sikaru extends SikaruApi {
    public override get chat(): Chat {
        const chat = super.chat;
        return Object.create(chat, { completions: { value: streamingCreate<ChatCompletions>(chat.completions) } });
    }

    public override get completions(): Completions {
        return streamingCreate<Completions>(super.completions);
    }

    public override get responses(): Responses {
        return streamingCreate<Responses>(super.responses);
    }
}
