"""OpenAI-shaped inference calls over the generated Sikaru client.

Hand-written next to the generated client, which owns every other file.
``Sikaru`` and ``AsyncSikaru`` are the generated ``SikaruApi`` clients, except that
``chat.completions``, ``completions`` and ``responses`` accept ``stream=True`` on
``create``, as the OpenAI SDK does. Each call dispatches to the generated ``create`` or ``create_stream``, so parameters,
request options and transport stay the generated client's own.
"""
from __future__ import annotations

import typing

from .client import AsyncSikaruApi, SikaruApi
from .types import ChatCompletion, ChatCompletionChunk, Completion, ModelResponse, ResponseStreamEvent


class _Resource:
    """A generated resource whose ``create`` also takes ``stream``; everything else passes through."""

    def __init__(self, generated: typing.Any):
        self._generated = generated

    def __getattr__(self, name: str) -> typing.Any:
        return getattr(self._generated, name)

    def _create(self, stream: typing.Optional[bool], params: typing.Dict[str, typing.Any]) -> typing.Any:
        if stream:
            return self._generated.create_stream(**params)
        return self._generated.create(**params)


class _AsyncResource(_Resource):
    async def _create(self, stream: typing.Optional[bool], params: typing.Dict[str, typing.Any]) -> typing.Any:
        if stream:
            return self._generated.create_stream(**params)
        return await self._generated.create(**params)


class ChatCompletions(_Resource):
    @typing.overload
    def create(self, *, stream: typing.Literal[True], **params: typing.Any) -> typing.Iterator[ChatCompletionChunk]: ...

    @typing.overload
    def create(self, *, stream: typing.Optional[typing.Literal[False]] = None, **params: typing.Any) -> ChatCompletion: ...

    def create(self, *, stream: typing.Optional[bool] = None, **params: typing.Any) -> typing.Any:
        return self._create(stream, params)


class Completions(_Resource):
    @typing.overload
    def create(self, *, stream: typing.Literal[True], **params: typing.Any) -> typing.Iterator[Completion]: ...

    @typing.overload
    def create(self, *, stream: typing.Optional[typing.Literal[False]] = None, **params: typing.Any) -> Completion: ...

    def create(self, *, stream: typing.Optional[bool] = None, **params: typing.Any) -> typing.Any:
        return self._create(stream, params)


class Responses(_Resource):
    @typing.overload
    def create(self, *, stream: typing.Literal[True], **params: typing.Any) -> typing.Iterator[ResponseStreamEvent]: ...

    @typing.overload
    def create(self, *, stream: typing.Optional[typing.Literal[False]] = None, **params: typing.Any) -> ModelResponse: ...

    def create(self, *, stream: typing.Optional[bool] = None, **params: typing.Any) -> typing.Any:
        return self._create(stream, params)


class AsyncChatCompletions(_AsyncResource):
    @typing.overload
    async def create(
        self, *, stream: typing.Literal[True], **params: typing.Any
    ) -> typing.AsyncIterator[ChatCompletionChunk]: ...

    @typing.overload
    async def create(
        self, *, stream: typing.Optional[typing.Literal[False]] = None, **params: typing.Any
    ) -> ChatCompletion: ...

    async def create(self, *, stream: typing.Optional[bool] = None, **params: typing.Any) -> typing.Any:
        return await self._create(stream, params)


class AsyncCompletions(_AsyncResource):
    @typing.overload
    async def create(self, *, stream: typing.Literal[True], **params: typing.Any) -> typing.AsyncIterator[Completion]: ...

    @typing.overload
    async def create(
        self, *, stream: typing.Optional[typing.Literal[False]] = None, **params: typing.Any
    ) -> Completion: ...

    async def create(self, *, stream: typing.Optional[bool] = None, **params: typing.Any) -> typing.Any:
        return await self._create(stream, params)


class AsyncResponses(_AsyncResource):
    @typing.overload
    async def create(
        self, *, stream: typing.Literal[True], **params: typing.Any
    ) -> typing.AsyncIterator[ResponseStreamEvent]: ...

    @typing.overload
    async def create(
        self, *, stream: typing.Optional[typing.Literal[False]] = None, **params: typing.Any
    ) -> ModelResponse: ...

    async def create(self, *, stream: typing.Optional[bool] = None, **params: typing.Any) -> typing.Any:
        return await self._create(stream, params)


class _Chat:
    def __init__(self, generated: typing.Any, completions: _Resource):
        self._generated = generated
        self.completions = completions

    def __getattr__(self, name: str) -> typing.Any:
        return getattr(self._generated, name)


class Sikaru(SikaruApi):
    """``SikaruApi`` with OpenAI-shaped ``create(stream=...)``; construct it exactly like ``SikaruApi``."""

    @property
    def chat(self) -> typing.Any:
        generated = super().chat
        return _Chat(generated, ChatCompletions(generated.completions))

    @property
    def completions(self) -> Completions:  # type: ignore[override]
        return Completions(super().completions)

    @property
    def responses(self) -> Responses:  # type: ignore[override]
        return Responses(super().responses)


class AsyncSikaru(AsyncSikaruApi):
    """``AsyncSikaruApi`` with OpenAI-shaped ``create(stream=...)``; ``await`` returns a stream or a result."""

    @property
    def chat(self) -> typing.Any:
        generated = super().chat
        return _Chat(generated, AsyncChatCompletions(generated.completions))

    @property
    def completions(self) -> AsyncCompletions:  # type: ignore[override]
        return AsyncCompletions(super().completions)

    @property
    def responses(self) -> AsyncResponses:  # type: ignore[override]
        return AsyncResponses(super().responses)
