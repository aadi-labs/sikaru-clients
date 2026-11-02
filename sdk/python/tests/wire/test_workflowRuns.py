from .conftest import get_client, verify_auth_headers, verify_request_count


def test_workflowRuns_project_workflow_run_events() -> None:
    """Test project_workflow_run_events endpoint with WireMock"""
    test_id = "workflow_runs.project_workflow_run_events.0"
    client = get_client(test_id)
    client.workflow_runs.project_workflow_run_events(
        project_id="project_id",
        run_id="run_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/workflow-runs/run_id/events", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/workflow-runs/run_id/events", {"Authorization": r"Bearer .+"}, []
    )


def test_workflowRuns_recover_project_workflow_run() -> None:
    """Test recover_project_workflow_run endpoint with WireMock"""
    test_id = "workflow_runs.recover_project_workflow_run.0"
    client = get_client(test_id)
    client.workflow_runs.recover_project_workflow_run(
        project_id="project_id",
        run_id="run_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/workflow-runs/run_id/recover", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/workflow-runs/run_id/recover", {"Authorization": r"Bearer .+"}, []
    )
