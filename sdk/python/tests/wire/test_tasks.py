from .conftest import get_client, verify_auth_headers, verify_request_count


def test_tasks_list_() -> None:
    """Test list endpoint with WireMock"""
    test_id = "tasks.list_.0"
    client = get_client(test_id)
    client.tasks.list(
        project_id="project_id",
        session_id="session_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/execution-sessions/session_id/tasks", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/execution-sessions/session_id/tasks",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_tasks_stop_all() -> None:
    """Test stop_all endpoint with WireMock"""
    test_id = "tasks.stop_all.0"
    client = get_client(test_id)
    client.tasks.stop_all(
        project_id="project_id",
        session_id="session_id",
        idempotency_key="idempotency_key",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/execution-sessions/session_id/tasks/stop-all", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/execution-sessions/session_id/tasks/stop-all",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_tasks_get() -> None:
    """Test get endpoint with WireMock"""
    test_id = "tasks.get.0"
    client = get_client(test_id)
    client.tasks.get(
        project_id="project_id",
        session_id="session_id",
        task_id="task_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/execution-sessions/session_id/tasks/task_id", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/execution-sessions/session_id/tasks/task_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_tasks_actions() -> None:
    """Test actions endpoint with WireMock"""
    test_id = "tasks.actions.0"
    client = get_client(test_id)
    client.tasks.actions(
        project_id="project_id",
        session_id="session_id",
        task_id="task_id",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/execution-sessions/session_id/tasks/task_id/actions", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/execution-sessions/session_id/tasks/task_id/actions",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_tasks_approve_action() -> None:
    """Test approve_action endpoint with WireMock"""
    test_id = "tasks.approve_action.0"
    client = get_client(test_id)
    client.tasks.approve_action(
        project_id="project_id",
        session_id="session_id",
        task_id="task_id",
        action_id="action_id",
        decision="approved",
        idempotency_key="idempotency_key",
    )
    verify_request_count(
        test_id,
        "POST",
        "/v1/projects/project_id/execution-sessions/session_id/tasks/task_id/actions/action_id/approval",
        None,
        1,
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/execution-sessions/session_id/tasks/task_id/actions/action_id/approval",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_tasks_cancel() -> None:
    """Test cancel endpoint with WireMock"""
    test_id = "tasks.cancel.0"
    client = get_client(test_id)
    client.tasks.cancel(
        project_id="project_id",
        session_id="session_id",
        task_id="task_id",
        idempotency_key="idempotency_key",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/execution-sessions/session_id/tasks/task_id/cancel", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/execution-sessions/session_id/tasks/task_id/cancel",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_tasks_changes() -> None:
    """Test changes endpoint with WireMock"""
    test_id = "tasks.changes.0"
    client = get_client(test_id)
    client.tasks.changes(
        project_id="project_id",
        session_id="session_id",
        task_id="task_id",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/execution-sessions/session_id/tasks/task_id/changes", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/execution-sessions/session_id/tasks/task_id/changes",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_tasks_integrations() -> None:
    """Test integrations endpoint with WireMock"""
    test_id = "tasks.integrations.0"
    client = get_client(test_id)
    client.tasks.integrations(
        project_id="project_id",
        session_id="session_id",
        task_id="task_id",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/execution-sessions/session_id/tasks/task_id/integrations", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/execution-sessions/session_id/tasks/task_id/integrations",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_tasks_integrate() -> None:
    """Test integrate endpoint with WireMock"""
    test_id = "tasks.integrate.0"
    client = get_client(test_id)
    client.tasks.integrate(
        project_id="project_id",
        session_id="session_id",
        task_id="task_id",
        idempotency_key="idempotency_key",
        revision=1,
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/execution-sessions/session_id/tasks/task_id/integrations", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/execution-sessions/session_id/tasks/task_id/integrations",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_tasks_message() -> None:
    """Test message endpoint with WireMock"""
    test_id = "tasks.message.0"
    client = get_client(test_id)
    client.tasks.message(
        project_id="project_id",
        session_id="session_id",
        task_id="task_id",
        idempotency_key="idempotency_key",
        input={"key": "value"},
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/execution-sessions/session_id/tasks/task_id/messages", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/execution-sessions/session_id/tasks/task_id/messages",
        {"Authorization": r"Bearer .+"},
        [],
    )
