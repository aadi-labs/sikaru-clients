from .conftest import get_client, verify_auth_headers, verify_request_count


def test_sessions_get_managed_session() -> None:
    """Test get_managed_session endpoint with WireMock"""
    test_id = "sessions.get_managed_session.0"
    client = get_client(test_id)
    client.sessions.get_managed_session(
        project_id="project_id",
        session_id="session_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/sessions/session_id", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/sessions/session_id", {"Authorization": r"Bearer .+"}, []
    )


def test_sessions_list_managed_session_events() -> None:
    """Test list_managed_session_events endpoint with WireMock"""
    test_id = "sessions.list_managed_session_events.0"
    client = get_client(test_id)
    client.sessions.list_managed_session_events(
        project_id="project_id",
        session_id="session_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/sessions/session_id/events", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/sessions/session_id/events", {"Authorization": r"Bearer .+"}, []
    )


def test_sessions_list_managed_session_files() -> None:
    """Test list_managed_session_files endpoint with WireMock"""
    test_id = "sessions.list_managed_session_files.0"
    client = get_client(test_id)
    client.sessions.list_managed_session_files(
        project_id="project_id",
        session_id="session_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/sessions/session_id/files", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/sessions/session_id/files", {"Authorization": r"Bearer .+"}, []
    )


def test_sessions_list_managed_session_plan() -> None:
    """Test list_managed_session_plan endpoint with WireMock"""
    test_id = "sessions.list_managed_session_plan.0"
    client = get_client(test_id)
    client.sessions.list_managed_session_plan(
        project_id="project_id",
        session_id="session_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/sessions/session_id/plan", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/sessions/session_id/plan", {"Authorization": r"Bearer .+"}, []
    )
