"""Verify signed deliveries that Sikaru sends to your tool provider.

Hand-written next to the generated client, which owns every other file.
Deliveries follow Standard Webhooks v1: ``webhook-id``, ``webhook-timestamp`` and
``webhook-signature`` headers, where the signature header holds one or more
space-separated ``v1,<base64 HMAC-SHA256>`` values over ``{id}.{timestamp}.{body}``.
During a secret rotation a delivery is signed with both secrets, so pass every
secret you currently accept and either one verifies it.
"""
from __future__ import annotations

import base64
import binascii
import hashlib
import hmac
import json
import time
import typing

TOLERANCE_SECONDS = 300
SECRET_PREFIX = 'whsec_'
SIGNATURE_VERSION = 'v1,'


class WebhookVerificationError(Exception):
    """The delivery is unsigned, stale, or signed with none of the given secrets."""


def verify(payload: typing.Union[bytes, str], headers: typing.Mapping[str, str],
           secrets: typing.Union[str, typing.Sequence[str]]) -> typing.Any:
    """Return the parsed JSON body of a delivery, or raise ``WebhookVerificationError``.

    ``payload`` is the raw request body exactly as received. ``secrets`` is one
    ``whsec_`` secret or a list of them; the prefix is optional.
    """
    lowered = {name.lower(): value for name, value in headers.items()}
    message_id = _header(lowered, 'webhook-id')
    timestamp = _header(lowered, 'webhook-timestamp')
    _check_timestamp(timestamp)
    body = payload.encode() if isinstance(payload, str) else bytes(payload)
    content = f'{message_id}.{timestamp}.'.encode() + body
    offered = _offered_signatures(_header(lowered, 'webhook-signature'))
    expected = [_sign(secret, content) for secret in _secret_list(secrets)]
    if not any(hmac.compare_digest(want, got) for want in expected for got in offered):
        raise WebhookVerificationError('No signature matches the given secrets')
    return json.loads(body)


def _header(headers: typing.Mapping[str, str], name: str) -> str:
    value = headers.get(name)
    if not value:
        raise WebhookVerificationError(f'Missing {name} header')
    return value


def _check_timestamp(timestamp: str) -> None:
    # Plain ASCII digits only, as the TypeScript helper requires: int() would also
    # take signs, underscores, spaces and non-ASCII digits.
    if not (timestamp.isascii() and timestamp.isdigit()):
        raise WebhookVerificationError('Invalid webhook-timestamp header')
    # Integer arithmetic, so a large value is just outside the window instead of
    # overflowing a float; the length cap also keeps int() under Python's
    # digit limit. Twenty digits is far beyond any real Unix time.
    if len(timestamp) > 20 or abs(int(time.time()) - int(timestamp)) > TOLERANCE_SECONDS:
        raise WebhookVerificationError('Delivery timestamp is outside the tolerance window')


def _offered_signatures(header: str) -> list[bytes]:
    return [part[len(SIGNATURE_VERSION):].encode() for part in header.split() if part.startswith(SIGNATURE_VERSION)]


def _secret_list(secrets: typing.Union[str, typing.Sequence[str]]) -> list[str]:
    listed = [secrets] if isinstance(secrets, str) else list(secrets)
    if not listed:
        raise ValueError('At least one secret is required')
    return listed


def _sign(secret: str, content: bytes) -> bytes:
    encoded = secret[len(SECRET_PREFIX):] if secret.startswith(SECRET_PREFIX) else secret
    try:
        key = base64.b64decode(encoded, validate=True)
    except binascii.Error:
        raise ValueError('A secret is not valid base64') from None
    return base64.b64encode(hmac.new(key, content, hashlib.sha256).digest())
