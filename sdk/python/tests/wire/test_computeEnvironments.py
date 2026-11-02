from .conftest import get_client, verify_auth_headers, verify_request_count


def test_computeEnvironments_create() -> None:
    """Test create endpoint with WireMock"""
    test_id = "compute_environments.create.0"
    client = get_client(test_id)
    client.compute_environments.create(
        project_id="project_id",
        environment_slug="environment_slug",
        idempotency_key="idempotency_key",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/compute-environments", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/compute-environments", {"Authorization": r"Bearer .+"}, []
    )


def test_computeEnvironments_get() -> None:
    """Test get endpoint with WireMock"""
    test_id = "compute_environments.get.0"
    client = get_client(test_id)
    client.compute_environments.get(
        project_id="project_id",
        environment_id="environment_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/compute-environments/environment_id", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/compute-environments/environment_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_computeEnvironments_disable() -> None:
    """Test disable endpoint with WireMock"""
    test_id = "compute_environments.disable.0"
    client = get_client(test_id)
    client.compute_environments.disable(
        project_id="project_id",
        environment_id="environment_id",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/compute-environments/environment_id/disable", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/compute-environments/environment_id/disable",
        {"Authorization": r"Bearer .+"},
        [],
    )
