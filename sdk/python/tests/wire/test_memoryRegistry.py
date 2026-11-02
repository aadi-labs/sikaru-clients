from .conftest import get_client, verify_auth_headers, verify_request_count


def test_memoryRegistry_create_memory_registry_change() -> None:
    """Test create_memory_registry_change endpoint with WireMock"""
    test_id = "memory_registry.create_memory_registry_change.0"
    client = get_client(test_id)
    client.memory_registry.create_memory_registry_change(
        project_id="project_id",
        memory_id="memoryId",
        memory_type="memoryType",
        name="name",
        scope="scope",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/memory-registry", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/memory-registry", {"Authorization": r"Bearer .+"}, [])
