from .conftest import get_client, verify_auth_headers, verify_request_count


def test_executions_execution_runtime_lineage() -> None:
    """Test execution_runtime_lineage endpoint with WireMock"""
    test_id = "executions.execution_runtime_lineage.0"
    client = get_client(test_id)
    client.executions.execution_runtime_lineage(
        project_id="project_id",
        trace_id="trace_id",
        account_id="account_id",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/executions/trace_id/runtime", {"account_id": "account_id"}, 1
    )
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/executions/trace_id/runtime", {"Authorization": r"Bearer .+"}, []
    )
