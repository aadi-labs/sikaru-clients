from .conftest import get_client, verify_auth_headers, verify_request_count


def test_completions_create_stream() -> None:
    """Test create_stream endpoint with WireMock"""
    test_id = "completions.create_stream.0"
    client = get_client(test_id)
    for _ in client.completions.create_stream(
        model="model",
        prompt="prompt",
    ):
        pass
    verify_request_count(test_id, "POST", "/v1/completions", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/completions", {"Authorization": r"Bearer .+"}, [])


def test_completions_create() -> None:
    """Test create endpoint with WireMock"""
    test_id = "completions.create.0"
    client = get_client(test_id)
    client.completions.create(
        model="model",
        prompt="prompt",
    )
    verify_request_count(test_id, "POST", "/v1/completions", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/completions", {"Authorization": r"Bearer .+"}, [])
