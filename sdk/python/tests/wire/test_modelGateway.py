from .conftest import get_client, verify_auth_headers, verify_request_count


def test_modelGateway_capture_model_gateway_chat_completion() -> None:
    """Test capture_model_gateway_chat_completion endpoint with WireMock"""
    test_id = "model_gateway.capture_model_gateway_chat_completion.0"
    client = get_client(test_id)
    client.model_gateway.capture_model_gateway_chat_completion(
        project_id="project_id",
        provider="provider",
        request={"key": "value"},
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/model-gateway/provider/chat/completions/capture", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/model-gateway/provider/chat/completions/capture",
        {"Authorization": r"Bearer .+"},
        [],
    )
