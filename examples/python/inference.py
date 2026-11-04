"""Call a Sikaru catalog model with the Python SDK.

Set SIKARU_API_KEY, and SIKARU_MODEL to a model ID from `client.models.list()`,
for example `moonshotai/kimi-k3`.
"""
import asyncio
import os

from sikaru_sdk import AsyncSikaru, Sikaru

model = os.environ["SIKARU_MODEL"]
messages = [{"role": "user", "content": "Explain a rainbow in one sentence."}]

# The client reads SIKARU_API_KEY from the environment.
client = Sikaru()

print("Models:", ", ".join(m.id for m in client.models.list().data[:5]))

reply = client.chat.completions.create(model=model, messages=messages)
print(reply.choices[0].message.content)

stream = client.chat.completions.create(model=model, messages=messages, stream=True)
for chunk in stream:
    if chunk.choices and chunk.choices[0].delta:
        print(chunk.choices[0].delta.content or "", end="", flush=True)
print()


async def main() -> None:
    async_client = AsyncSikaru()
    result = await async_client.chat.completions.create(model=model, messages=messages)
    print(result.choices[0].message.content)


asyncio.run(main())
