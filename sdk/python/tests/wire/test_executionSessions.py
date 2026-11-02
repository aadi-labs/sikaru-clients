from .conftest import get_client, verify_auth_headers, verify_request_count


def test_executionSessions_list_() -> None:
    """Test list endpoint with WireMock"""
    test_id = "execution_sessions.list_.0"
    client = get_client(test_id)
    client.execution_sessions.list(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/execution-sessions", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/execution-sessions", {"Authorization": r"Bearer .+"}, []
    )


def test_executionSessions_get() -> None:
    """Test get endpoint with WireMock"""
    test_id = "execution_sessions.get.0"
    client = get_client(test_id)
    client.execution_sessions.get(
        project_id="project_id",
        session_id="session_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/execution-sessions/session_id", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/execution-sessions/session_id", {"Authorization": r"Bearer .+"}, []
    )


def test_executionSessions_branch() -> None:
    """Test branch endpoint with WireMock"""
    test_id = "execution_sessions.branch.0"
    client = get_client(test_id)
    client.execution_sessions.branch(
        project_id="project_id",
        session_id="session_id",
        idempotency_key="idempotency_key",
        source_run_id="source_run_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/execution-sessions/session_id/branches", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/execution-sessions/session_id/branches",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_executionSessions_list_files() -> None:
    """Test list_files endpoint with WireMock"""
    test_id = "execution_sessions.list_files.0"
    client = get_client(test_id)
    client.execution_sessions.list_files(
        project_id="project_id",
        session_id="session_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/execution-sessions/session_id/files", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/execution-sessions/session_id/files",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_executionSessions_delete_file() -> None:
    """Test delete_file endpoint with WireMock"""
    test_id = "execution_sessions.delete_file.0"
    client = get_client(test_id)
    client.execution_sessions.delete_file(
        project_id="project_id",
        session_id="session_id",
        file_id="file_id",
    )
    verify_request_count(
        test_id, "DELETE", "/v1/projects/project_id/execution-sessions/session_id/files/file_id", None, 1
    )
    verify_auth_headers(
        test_id,
        "DELETE",
        "/v1/projects/project_id/execution-sessions/session_id/files/file_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_executionSessions_download_file() -> None:
    """Test download_file endpoint with WireMock"""
    test_id = "execution_sessions.download_file.0"
    client = get_client(test_id)
    for _ in client.execution_sessions.download_file(
        project_id="project_id",
        session_id="session_id",
        file_id="file_id",
    ):
        pass
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/execution-sessions/session_id/files/file_id/content", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/execution-sessions/session_id/files/file_id/content",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_executionSessions_list_session_inputs() -> None:
    """Test list_session_inputs endpoint with WireMock"""
    test_id = "execution_sessions.list_session_inputs.0"
    client = get_client(test_id)
    client.execution_sessions.list_session_inputs(
        project_id="project_id",
        session_id="session_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/execution-sessions/session_id/inputs", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/execution-sessions/session_id/inputs",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_executionSessions_spend() -> None:
    """Test spend endpoint with WireMock"""
    test_id = "execution_sessions.spend.0"
    client = get_client(test_id)
    client.execution_sessions.spend(
        project_id="project_id",
        session_id="session_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/execution-sessions/session_id/spend", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/execution-sessions/session_id/spend",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_executionSessions_append_turn() -> None:
    """Test append_turn endpoint with WireMock"""
    test_id = "execution_sessions.append_turn.0"
    client = get_client(test_id)
    client.execution_sessions.append_turn(
        project_id="project_id",
        session_id="session_id",
        idempotency_key="idempotency_key",
        input={"key": "value"},
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/execution-sessions/session_id/turns", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/execution-sessions/session_id/turns",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_executionSessions_create() -> None:
    """Test create endpoint with WireMock"""
    test_id = "execution_sessions.create.0"
    client = get_client(test_id)
    client.execution_sessions.create(
        project_id="project_id",
        harness_id="harness_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/harnesses/harness_id/execution-sessions", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/harnesses/harness_id/execution-sessions",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_executionSessions_list_eligible_releases() -> None:
    """Test list_eligible_releases endpoint with WireMock"""
    test_id = "execution_sessions.list_eligible_releases.0"
    client = get_client(test_id)
    client.execution_sessions.list_eligible_releases(
        project_id="project_id",
        harness_id="harness_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/harnesses/harness_id/releases", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/harnesses/harness_id/releases", {"Authorization": r"Bearer .+"}, []
    )
