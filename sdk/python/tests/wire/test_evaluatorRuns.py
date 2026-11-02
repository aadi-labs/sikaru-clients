from .conftest import get_client, verify_auth_headers, verify_request_count


def test_evaluatorRuns_create_evaluator_run() -> None:
    """Test create_evaluator_run endpoint with WireMock"""
    test_id = "evaluator_runs.create_evaluator_run.0"
    client = get_client(test_id)
    client.evaluator_runs.create_evaluator_run(
        project_id="project_id",
        trace_ids=["traceIds"],
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/evaluator-runs", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/evaluator-runs", {"Authorization": r"Bearer .+"}, [])
