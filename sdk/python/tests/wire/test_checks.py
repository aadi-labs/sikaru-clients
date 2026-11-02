from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_api import CheckEnvironment, HarborTaskFiles


def test_checks_list_() -> None:
    """Test list endpoint with WireMock"""
    test_id = "checks.list_.0"
    client = get_client(test_id)
    client.checks.list(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/checks", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/checks", {"Authorization": r"Bearer .+"}, []
    )


def test_checks_create() -> None:
    """Test create endpoint with WireMock"""
    test_id = "checks.create.0"
    client = get_client(test_id)
    client.checks.create(
        project_id="project_id",
        agent_slug="agent_slug",
        environment=CheckEnvironment(
            kind="managed",
        ),
        idempotency_key="idempotency_key",
        name="name",
        task=HarborTaskFiles(
            files={"key": "value"},
        ),
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/managed-agents/agent_slug/checks", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/managed-agents/agent_slug/checks", {"Authorization": r"Bearer .+"}, []
    )


def test_checks_list_results() -> None:
    """Test list_results endpoint with WireMock"""
    test_id = "checks.list_results.0"
    client = get_client(test_id)
    client.checks.list_results(
        project_id="project_id",
        agent_slug="agent_slug",
        check_id="check_id",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/checks/check_id/results", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/checks/check_id/results",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_checks_run() -> None:
    """Test run endpoint with WireMock"""
    test_id = "checks.run.0"
    client = get_client(test_id)
    client.checks.run(
        project_id="project_id",
        agent_slug="agent_slug",
        check_id="check_id",
        idempotency_key="idempotency_key",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/managed-agents/agent_slug/checks/check_id/runs", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/checks/check_id/runs",
        {"Authorization": r"Bearer .+"},
        [],
    )
