from .conftest import get_client, verify_auth_headers, verify_request_count


def test_specialists_list_() -> None:
    """Test list endpoint with WireMock"""
    test_id = "specialists.list_.0"
    client = get_client(test_id)
    client.specialists.list(
        project_id="project_id",
        session_id="session_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/execution-sessions/session_id/specialists", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/execution-sessions/session_id/specialists",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_specialists_get() -> None:
    """Test get endpoint with WireMock"""
    test_id = "specialists.get.0"
    client = get_client(test_id)
    client.specialists.get(
        project_id="project_id",
        session_id="session_id",
        thread_id="thread_id",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/execution-sessions/session_id/specialists/thread_id", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/execution-sessions/session_id/specialists/thread_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_specialists_cancel() -> None:
    """Test cancel endpoint with WireMock"""
    test_id = "specialists.cancel.0"
    client = get_client(test_id)
    client.specialists.cancel(
        project_id="project_id",
        session_id="session_id",
        thread_id="thread_id",
        idempotency_key="idempotency_key",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/execution-sessions/session_id/specialists/thread_id/cancel", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/execution-sessions/session_id/specialists/thread_id/cancel",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_specialists_message() -> None:
    """Test message endpoint with WireMock"""
    test_id = "specialists.message.0"
    client = get_client(test_id)
    client.specialists.message(
        project_id="project_id",
        session_id="session_id",
        thread_id="thread_id",
        idempotency_key="idempotency_key",
        input={"key": "value"},
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/execution-sessions/session_id/specialists/thread_id/messages", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/execution-sessions/session_id/specialists/thread_id/messages",
        {"Authorization": r"Bearer .+"},
        [],
    )
