from .conftest import get_client, verify_auth_headers, verify_request_count


def test_deployments_list_console_deployments() -> None:
    """Test list_console_deployments endpoint with WireMock"""
    test_id = "deployments.list_console_deployments.0"
    client = get_client(test_id)
    client.deployments.list_console_deployments(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/deployments", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/deployments", {"Authorization": r"Bearer .+"}, [])
