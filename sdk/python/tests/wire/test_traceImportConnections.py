from .conftest import get_client, verify_auth_headers, verify_request_count


def test_traceImportConnections_list_trace_import_connections() -> None:
    """Test list_trace_import_connections endpoint with WireMock"""
    test_id = "trace_import_connections.list_trace_import_connections.0"
    client = get_client(test_id)
    client.trace_import_connections.list_trace_import_connections(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/trace-import-connections", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/trace-import-connections", {"Authorization": r"Bearer .+"}, []
    )
