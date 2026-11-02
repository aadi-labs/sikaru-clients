from .conftest import get_client, verify_auth_headers, verify_request_count


def test_gitCredentials_list_() -> None:
    """Test list endpoint with WireMock"""
    test_id = "git_credentials.list_.0"
    client = get_client(test_id)
    client.git_credentials.list(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/git-credentials", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/git-credentials", {"Authorization": r"Bearer .+"}, [])


def test_gitCredentials_create() -> None:
    """Test create endpoint with WireMock"""
    test_id = "git_credentials.create.0"
    client = get_client(test_id)
    client.git_credentials.create(
        project_id="project_id",
        host="host",
        token="token",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/git-credentials", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/git-credentials", {"Authorization": r"Bearer .+"}, [])


def test_gitCredentials_grant() -> None:
    """Test grant endpoint with WireMock"""
    test_id = "git_credentials.grant.0"
    client = get_client(test_id)
    client.git_credentials.grant(
        project_id="project_id",
        credential_id="credential_id",
        agent_id="agentId",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/git-credentials/credential_id/grants", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/git-credentials/credential_id/grants",
        {"Authorization": r"Bearer .+"},
        [],
    )
