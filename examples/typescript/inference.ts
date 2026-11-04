// Call a Sikaru catalog model with the TypeScript SDK.
// Set SIKARU_API_KEY, and SIKARU_MODEL to a model ID from `client.models.list()`,
// for example `moonshotai/kimi-k3`.
import { Sikaru } from "@aadi-labs/sikaru-sdk/inference";

const apiKey = process.env.SIKARU_API_KEY;
const model = process.env.SIKARU_MODEL;
if (!apiKey || !model) throw new Error("Set SIKARU_API_KEY and SIKARU_MODEL");

const client = new Sikaru({ apiKey });
const messages = [{ role: "user", content: "Explain a rainbow in one sentence." }];

const models = await client.models.list();
console.log("Models:", models.data.slice(0, 5).map((m) => m.id).join(", "));

const reply = await client.chat.completions.create({ model, messages });
console.log(reply.choices?.[0]?.message?.content);

const stream = await client.chat.completions.create({ model, messages, stream: true });
for await (const chunk of stream) {
  process.stdout.write(chunk.choices?.[0]?.delta?.content ?? "");
}
console.log();
