from .conftest import get_client, verify_auth_headers, verify_request_count


def test_computeCredentials_renew() -> None:
    """Test renew endpoint with WireMock"""
    test_id = "compute_credentials.renew.0"
    client = get_client(test_id)
    client.compute_credentials.renew(
        project_id="project_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/compute-credentials/renew", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/compute-credentials/renew", {"Authorization": r"Bearer .+"}, []
    )


def test_computeCredentials_revoke() -> None:
    """Test revoke endpoint with WireMock"""
    test_id = "compute_credentials.revoke.0"
    client = get_client(test_id)
    client.compute_credentials.revoke(
        project_id="project_id",
        credential_id="credential_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/compute-credentials/credential_id/revoke", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/compute-credentials/credential_id/revoke",
        {"Authorization": r"Bearer .+"},
        [],
    )
