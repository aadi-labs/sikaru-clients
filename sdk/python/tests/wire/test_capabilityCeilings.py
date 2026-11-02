from .conftest import get_client, verify_auth_headers, verify_request_count


def test_capabilityCeilings_get() -> None:
    """Test get endpoint with WireMock"""
    test_id = "capability_ceilings.get.0"
    client = get_client(test_id)
    client.capability_ceilings.get(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/capability-ceilings", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/capability-ceilings", {"Authorization": r"Bearer .+"}, []
    )


def test_capabilityCeilings_update() -> None:
    """Test update endpoint with WireMock"""
    test_id = "capability_ceilings.update.0"
    client = get_client(test_id)
    client.capability_ceilings.update(
        project_id="project_id",
    )
    verify_request_count(test_id, "PUT", "/v1/projects/project_id/capability-ceilings", None, 1)
    verify_auth_headers(
        test_id, "PUT", "/v1/projects/project_id/capability-ceilings", {"Authorization": r"Bearer .+"}, []
    )
