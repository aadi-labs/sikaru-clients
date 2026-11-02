from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_api import ResumeInput


def test_executionObjectives_list_objectives() -> None:
    """Test list_objectives endpoint with WireMock"""
    test_id = "execution_objectives.list_objectives.0"
    client = get_client(test_id)
    client.execution_objectives.list_objectives(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/execution-objectives", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/execution-objectives", {"Authorization": r"Bearer .+"}, []
    )


def test_executionObjectives_create() -> None:
    """Test create endpoint with WireMock"""
    test_id = "execution_objectives.create.0"
    client = get_client(test_id)
    client.execution_objectives.create(
        project_id="project_id",
        criterion="criterion",
        idempotency_key="idempotency_key",
        objective="objective",
        run_id="run_id",
        session_id="session_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/execution-objectives", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/execution-objectives", {"Authorization": r"Bearer .+"}, []
    )


def test_executionObjectives_get() -> None:
    """Test get endpoint with WireMock"""
    test_id = "execution_objectives.get.0"
    client = get_client(test_id)
    client.execution_objectives.get(
        project_id="project_id",
        objective_id="objective_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/execution-objectives/objective_id", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/execution-objectives/objective_id", {"Authorization": r"Bearer .+"}, []
    )


def test_executionObjectives_cancel() -> None:
    """Test cancel endpoint with WireMock"""
    test_id = "execution_objectives.cancel.0"
    client = get_client(test_id)
    client.execution_objectives.cancel(
        project_id="project_id",
        objective_id="objective_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/execution-objectives/objective_id/cancel", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/execution-objectives/objective_id/cancel",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_executionObjectives_pause() -> None:
    """Test pause endpoint with WireMock"""
    test_id = "execution_objectives.pause.0"
    client = get_client(test_id)
    client.execution_objectives.pause(
        project_id="project_id",
        objective_id="objective_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/execution-objectives/objective_id/pause", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/execution-objectives/objective_id/pause",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_executionObjectives_resume() -> None:
    """Test resume endpoint with WireMock"""
    test_id = "execution_objectives.resume.0"
    client = get_client(test_id)
    client.execution_objectives.resume(
        project_id="project_id",
        objective_id="objective_id",
        request=ResumeInput(),
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/execution-objectives/objective_id/resume", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/execution-objectives/objective_id/resume",
        {"Authorization": r"Bearer .+"},
        [],
    )
