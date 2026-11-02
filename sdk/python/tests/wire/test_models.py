from .conftest import get_client, verify_auth_headers, verify_request_count


def test_models_list_() -> None:
    """Test list endpoint with WireMock"""
    test_id = "models.list_.0"
    client = get_client(test_id)
    client.models.list()
    verify_request_count(test_id, "GET", "/v1/models", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/models", {"Authorization": r"Bearer .+"}, [])


def test_models_retrieve() -> None:
    """Test retrieve endpoint with WireMock"""
    test_id = "models.retrieve.0"
    client = get_client(test_id)
    client.models.retrieve(
        model="model",
    )
    verify_request_count(test_id, "GET", "/v1/models/model", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/models/model", {"Authorization": r"Bearer .+"}, [])
