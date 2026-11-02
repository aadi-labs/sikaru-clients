from .conftest import get_client, verify_auth_headers, verify_request_count


def test_contextRegistry_create_context_registry_change() -> None:
    """Test create_context_registry_change endpoint with WireMock"""
    test_id = "context_registry.create_context_registry_change.0"
    client = get_client(test_id)
    client.context_registry.create_context_registry_change(
        project_id="project_id",
        name="name",
        repo_id="repoId",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/context-registry", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/context-registry", {"Authorization": r"Bearer .+"}, []
    )
