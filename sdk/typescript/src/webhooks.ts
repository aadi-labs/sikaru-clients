// Hand-written next to the generated client, which owns every other file.
//
// Verify signed deliveries that Sikaru sends to your tool provider. Deliveries follow
// Standard Webhooks v1: `webhook-id`, `webhook-timestamp` and `webhook-signature`
// headers, where the signature header holds one or more space-separated
// `v1,<base64 HMAC-SHA256>` values over `{id}.{timestamp}.{body}`. During a secret
// rotation a delivery is signed with both secrets, so pass every secret you currently
// accept and either one verifies it. Uses Web Crypto, so it runs in Node.js 20+,
// browsers and edge runtimes.

export const TOLERANCE_SECONDS = 300;
const SECRET_PREFIX = "whsec_";
const SIGNATURE_VERSION = "v1,";

/** Bytes backed by a plain ArrayBuffer, as Web Crypto requires. */
type Bytes = Uint8Array<ArrayBuffer>;

/** The delivery is unsigned, stale, or signed with none of the given secrets. */
export class WebhookVerificationError extends Error {
    constructor(message: string) {
        super(message);
        this.name = "WebhookVerificationError";
        Object.setPrototypeOf(this, WebhookVerificationError.prototype);
    }
}

export type WebhookHeaders = Headers | Record<string, string | string[] | undefined>;

/**
 * Resolve to the parsed JSON body of a delivery, or reject with `WebhookVerificationError`.
 *
 * `payload` is the raw request body exactly as received. `secrets` is one `whsec_`
 * secret or a list of them; the prefix is optional.
 */
export async function verify(
    payload: string | Uint8Array,
    headers: WebhookHeaders,
    secrets: string | readonly string[],
): Promise<unknown> {
    const messageId = requiredHeader(headers, "webhook-id");
    const timestamp = requiredHeader(headers, "webhook-timestamp");
    checkTimestamp(timestamp);
    const body = typeof payload === "string" ? new TextEncoder().encode(payload) : payload;
    const content = concat(new TextEncoder().encode(`${messageId}.${timestamp}.`), body);
    const offered = offeredSignatures(requiredHeader(headers, "webhook-signature"));
    for (const secret of secretList(secrets)) {
        if (await matchesAny(secret, content, offered)) {
            return JSON.parse(new TextDecoder().decode(body));
        }
    }
    throw new WebhookVerificationError("No signature matches the given secrets");
}

function requiredHeader(headers: WebhookHeaders, name: string): string {
    const value = headerValue(headers, name);
    if (!value) {
        throw new WebhookVerificationError(`Missing ${name} header`);
    }
    return value;
}

function headerValue(headers: WebhookHeaders, name: string): string | undefined {
    if (typeof Headers !== "undefined" && headers instanceof Headers) {
        return headers.get(name) ?? undefined;
    }
    for (const [key, value] of Object.entries(headers)) {
        if (key.toLowerCase() === name) {
            return Array.isArray(value) ? value.join(" ") : value;
        }
    }
    return undefined;
}

function checkTimestamp(timestamp: string): void {
    if (!/^\d+$/.test(timestamp)) {
        throw new WebhookVerificationError("Invalid webhook-timestamp header");
    }
    if (Math.abs(Date.now() / 1000 - Number(timestamp)) > TOLERANCE_SECONDS) {
        throw new WebhookVerificationError("Delivery timestamp is outside the tolerance window");
    }
}

function offeredSignatures(header: string): Bytes[] {
    return header
        .split(" ")
        .filter((part) => part.startsWith(SIGNATURE_VERSION))
        .map((part) => decodeBase64(part.slice(SIGNATURE_VERSION.length)))
        .filter((signature): signature is Bytes => signature !== undefined);
}

function secretList(secrets: string | readonly string[]): readonly string[] {
    const listed = typeof secrets === "string" ? [secrets] : secrets;
    if (listed.length === 0) {
        throw new TypeError("At least one secret is required");
    }
    return listed;
}

/** `crypto.subtle.verify` compares HMACs in constant time. */
async function matchesAny(secret: string, content: Bytes, offered: readonly Bytes[]): Promise<boolean> {
    const key = await crypto.subtle.importKey("raw", signingKey(secret), { name: "HMAC", hash: "SHA-256" }, false, [
        "verify",
    ]);
    for (const signature of offered) {
        if (await crypto.subtle.verify("HMAC", key, signature, content)) {
            return true;
        }
    }
    return false;
}

function signingKey(secret: string): Bytes {
    const encoded = secret.startsWith(SECRET_PREFIX) ? secret.slice(SECRET_PREFIX.length) : secret;
    const key = decodeBase64(encoded);
    if (key === undefined) {
        throw new TypeError("A secret is not valid base64");
    }
    return key;
}

function decodeBase64(value: string): Bytes | undefined {
    try {
        return Uint8Array.from(atob(value), (character) => character.charCodeAt(0));
    } catch {
        return undefined;
    }
}

function concat(first: Uint8Array, second: Uint8Array): Bytes {
    const joined = new Uint8Array(first.length + second.length);
    joined.set(first);
    joined.set(second, first.length);
    return joined;
}
