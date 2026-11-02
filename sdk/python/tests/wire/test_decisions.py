from .conftest import get_client, verify_auth_headers, verify_request_count


def test_decisions_create() -> None:
    """Test create endpoint with WireMock"""
    test_id = "decisions.create.0"
    client = get_client(test_id)
    client.decisions.create(
        questions={"key": {"key": "value"}},
        state="state",
    )
    verify_request_count(test_id, "POST", "/v1/systemone", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/systemone", {"Authorization": r"Bearer .+"}, [])
