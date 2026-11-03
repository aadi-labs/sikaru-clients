from .conftest import get_client, verify_auth_headers, verify_request_count


def test_projects_list_projects() -> None:
    """Test list_projects endpoint with WireMock"""
    test_id = "projects.list_projects.0"
    client = get_client(test_id)
    client.projects.list_projects()
    verify_request_count(test_id, "GET", "/v1/projects", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects", {"Authorization": r"Bearer .+"}, [])
