from .conftest import get_client, verify_auth_headers, verify_request_count


def test_graders_list_() -> None:
    """Test list endpoint with WireMock"""
    test_id = "graders.list_.0"
    client = get_client(test_id)
    client.graders.list(
        project_id="project_id",
        agent_id="agent_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/agents/agent_id/graders", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/agents/agent_id/graders", {"Authorization": r"Bearer .+"}, []
    )


def test_graders_test() -> None:
    """Test test endpoint with WireMock"""
    test_id = "graders.test.0"
    client = get_client(test_id)
    client.graders.test(
        project_id="project_id",
        agent_id="agent_id",
        file="file",
        name="name",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/agents/agent_id/graders/test", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/agents/agent_id/graders/test", {"Authorization": r"Bearer .+"}, []
    )


def test_graders_get_test() -> None:
    """Test get_test endpoint with WireMock"""
    test_id = "graders.get_test.0"
    client = get_client(test_id)
    client.graders.get_test(
        project_id="project_id",
        agent_id="agent_id",
        test_id="test_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/agents/agent_id/graders/tests/test_id", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/agents/agent_id/graders/tests/test_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_graders_get() -> None:
    """Test get endpoint with WireMock"""
    test_id = "graders.get.0"
    client = get_client(test_id)
    client.graders.get(
        project_id="project_id",
        agent_id="agent_id",
        grader_id="grader_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/agents/agent_id/graders/grader_id", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/agents/agent_id/graders/grader_id", {"Authorization": r"Bearer .+"}, []
    )


def test_graders_update() -> None:
    """Test update endpoint with WireMock"""
    test_id = "graders.update.0"
    client = get_client(test_id)
    client.graders.update(
        project_id="project_id",
        agent_id="agent_id",
        grader_id="grader_id",
    )
    verify_request_count(test_id, "PATCH", "/v1/projects/project_id/agents/agent_id/graders/grader_id", None, 1)
    verify_auth_headers(
        test_id,
        "PATCH",
        "/v1/projects/project_id/agents/agent_id/graders/grader_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_graders_estimate_cost() -> None:
    """Test estimate_cost endpoint with WireMock"""
    test_id = "graders.estimate_cost.0"
    client = get_client(test_id)
    client.graders.estimate_cost(
        project_id="project_id",
        agent_id="agent_id",
        grader_id="grader_id",
        judge="llm",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/agents/agent_id/graders/grader_id/cost-estimate", {"judge": "llm"}, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/agents/agent_id/graders/grader_id/cost-estimate",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_graders_label_queue() -> None:
    """Test label_queue endpoint with WireMock"""
    test_id = "graders.label_queue.0"
    client = get_client(test_id)
    client.graders.label_queue(
        project_id="project_id",
        agent_id="agent_id",
        grader_id="grader_id",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/agents/agent_id/graders/grader_id/label-queue", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/agents/agent_id/graders/grader_id/label-queue",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_graders_label() -> None:
    """Test label endpoint with WireMock"""
    test_id = "graders.label.0"
    client = get_client(test_id)
    client.graders.label(
        project_id="project_id",
        agent_id="agent_id",
        grader_id="grader_id",
        conversation_key="conversationKey",
        verdict="flagged",
    )
    verify_request_count(test_id, "PUT", "/v1/projects/project_id/agents/agent_id/graders/grader_id/labels", None, 1)
    verify_auth_headers(
        test_id,
        "PUT",
        "/v1/projects/project_id/agents/agent_id/graders/grader_id/labels",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_graders_run() -> None:
    """Test run endpoint with WireMock"""
    test_id = "graders.run.0"
    client = get_client(test_id)
    client.graders.run(
        project_id="project_id",
        agent_id="agent_id",
        grader_id="grader_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/agents/agent_id/graders/grader_id/run", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/agents/agent_id/graders/grader_id/run",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_graders_get_run() -> None:
    """Test get_run endpoint with WireMock"""
    test_id = "graders.get_run.0"
    client = get_client(test_id)
    client.graders.get_run(
        project_id="project_id",
        agent_id="agent_id",
        grader_id="grader_id",
        run_id="run_id",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/agents/agent_id/graders/grader_id/runs/run_id", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/agents/agent_id/graders/grader_id/runs/run_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_graders_samples() -> None:
    """Test samples endpoint with WireMock"""
    test_id = "graders.samples.0"
    client = get_client(test_id)
    client.graders.samples(
        project_id="project_id",
        agent_id="agent_id",
        grader_id="grader_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/agents/agent_id/graders/grader_id/samples", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/agents/agent_id/graders/grader_id/samples",
        {"Authorization": r"Bearer .+"},
        [],
    )
