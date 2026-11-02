from .conftest import get_client, verify_auth_headers, verify_request_count


def test_environments_list_managed_environments() -> None:
    """Test list_managed_environments endpoint with WireMock"""
    test_id = "environments.list_managed_environments.0"
    client = get_client(test_id)
    client.environments.list_managed_environments(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/environments", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/environments", {"Authorization": r"Bearer .+"}, [])


def test_environments_create_managed_environment() -> None:
    """Test create_managed_environment endpoint with WireMock"""
    test_id = "environments.create_managed_environment.0"
    client = get_client(test_id)
    client.environments.create_managed_environment(
        project_id="project_id",
        environment_slug="environmentSlug",
        provider_type="providerType",
        runtime_provider="runtimeProvider",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/environments", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/environments", {"Authorization": r"Bearer .+"}, [])
