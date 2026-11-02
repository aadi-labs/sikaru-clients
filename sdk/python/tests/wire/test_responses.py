from .conftest import get_client, verify_auth_headers, verify_request_count


def test_responses_create_stream() -> None:
    """Test create_stream endpoint with WireMock"""
    test_id = "responses.create_stream.0"
    client = get_client(test_id)
    for _ in client.responses.create_stream(
        model="model",
    ):
        pass
    verify_request_count(test_id, "POST", "/v1/responses", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/responses", {"Authorization": r"Bearer .+"}, [])


def test_responses_create() -> None:
    """Test create endpoint with WireMock"""
    test_id = "responses.create.0"
    client = get_client(test_id)
    client.responses.create(
        model="model",
    )
    verify_request_count(test_id, "POST", "/v1/responses", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/responses", {"Authorization": r"Bearer .+"}, [])
