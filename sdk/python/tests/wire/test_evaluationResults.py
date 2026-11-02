from .conftest import get_client, verify_auth_headers, verify_request_count


def test_evaluationResults_list_results() -> None:
    """Test list_results endpoint with WireMock"""
    test_id = "evaluation_results.list_results.0"
    client = get_client(test_id)
    client.evaluation_results.list_results(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/evaluation-results", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/evaluation-results", {"Authorization": r"Bearer .+"}, []
    )


def test_evaluationResults_record_result() -> None:
    """Test record_result endpoint with WireMock"""
    test_id = "evaluation_results.record_result.0"
    client = get_client(test_id)
    client.evaluation_results.record_result(
        project_id="project_id",
        environment="production",
        evaluator="evaluator",
        id="id",
        public_reason="publicReason",
        revision="revision",
        source="human",
        trace_id="traceId",
        verdict="pass",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/evaluation-results", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/evaluation-results", {"Authorization": r"Bearer .+"}, []
    )
