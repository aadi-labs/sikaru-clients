from .conftest import get_client, verify_auth_headers, verify_request_count


def test_chat_completions_create_stream() -> None:
    """Test create_stream endpoint with WireMock"""
    test_id = "chat.completions.create_stream.0"
    client = get_client(test_id)
    for _ in client.chat.completions.create_stream(
        messages=[{"key": "value"}],
        model="model",
    ):
        pass
    verify_request_count(test_id, "POST", "/v1/chat/completions", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/chat/completions", {"Authorization": r"Bearer .+"}, [])


def test_chat_completions_create() -> None:
    """Test create endpoint with WireMock"""
    test_id = "chat.completions.create.0"
    client = get_client(test_id)
    client.chat.completions.create(
        messages=[{"key": "value"}],
        model="model",
    )
    verify_request_count(test_id, "POST", "/v1/chat/completions", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/chat/completions", {"Authorization": r"Bearer .+"}, [])
