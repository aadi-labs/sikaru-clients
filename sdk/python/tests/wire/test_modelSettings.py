from .conftest import get_client, verify_auth_headers, verify_request_count


def test_modelSettings_get_model_settings() -> None:
    """Test get_model_settings endpoint with WireMock"""
    test_id = "model_settings.get_model_settings.0"
    client = get_client(test_id)
    client.model_settings.get_model_settings(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/model-settings", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/model-settings", {"Authorization": r"Bearer .+"}, [])


def test_modelSettings_update_model_settings() -> None:
    """Test update_model_settings endpoint with WireMock"""
    test_id = "model_settings.update_model_settings.0"
    client = get_client(test_id)
    client.model_settings.update_model_settings(
        project_id="project_id",
        selected_model_id="selectedModelId",
    )
    verify_request_count(test_id, "PUT", "/v1/projects/project_id/model-settings", None, 1)
    verify_auth_headers(test_id, "PUT", "/v1/projects/project_id/model-settings", {"Authorization": r"Bearer .+"}, [])
