from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_api import TraceImportScopeRequest


def test_traceImports_list_trace_imports() -> None:
    """Test list_trace_imports endpoint with WireMock"""
    test_id = "trace_imports.list_trace_imports.0"
    client = get_client(test_id)
    client.trace_imports.list_trace_imports(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/trace-imports", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/trace-imports", {"Authorization": r"Bearer .+"}, [])


def test_traceImports_create_trace_import() -> None:
    """Test create_trace_import endpoint with WireMock"""
    test_id = "trace_imports.create_trace_import.0"
    client = get_client(test_id)
    client.trace_imports.create_trace_import(
        project_id="project_id",
        connection_id="connectionId",
        converter_version="converterVersion",
        dataset="dataset",
        external_project_id="externalProjectId",
        mode="historical",
        provider="langsmith",
        scope=TraceImportScopeRequest(),
        source_instance="sourceInstance",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/trace-imports", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/trace-imports", {"Authorization": r"Bearer .+"}, [])


def test_traceImports_plan_trace_import() -> None:
    """Test plan_trace_import endpoint with WireMock"""
    test_id = "trace_imports.plan_trace_import.0"
    client = get_client(test_id)
    client.trace_imports.plan_trace_import(
        project_id="project_id",
        connection_id="connectionId",
        converter_version="converterVersion",
        dataset="dataset",
        external_project_id="externalProjectId",
        mode="historical",
        provider="langsmith",
        scope=TraceImportScopeRequest(),
        source_instance="sourceInstance",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/trace-imports/plan", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/trace-imports/plan", {"Authorization": r"Bearer .+"}, []
    )


def test_traceImports_get_trace_import() -> None:
    """Test get_trace_import endpoint with WireMock"""
    test_id = "trace_imports.get_trace_import.0"
    client = get_client(test_id)
    client.trace_imports.get_trace_import(
        project_id="project_id",
        trace_import_id="trace_import_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/trace-imports/trace_import_id", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/trace-imports/trace_import_id", {"Authorization": r"Bearer .+"}, []
    )


def test_traceImports_cancel_trace_import() -> None:
    """Test cancel_trace_import endpoint with WireMock"""
    test_id = "trace_imports.cancel_trace_import.0"
    client = get_client(test_id)
    client.trace_imports.cancel_trace_import(
        project_id="project_id",
        trace_import_id="trace_import_id",
        expected_version=1,
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/trace-imports/trace_import_id/cancel", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/trace-imports/trace_import_id/cancel",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_traceImports_get_trace_import_receipt() -> None:
    """Test get_trace_import_receipt endpoint with WireMock"""
    test_id = "trace_imports.get_trace_import_receipt.0"
    client = get_client(test_id)
    client.trace_imports.get_trace_import_receipt(
        project_id="project_id",
        trace_import_id="trace_import_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/trace-imports/trace_import_id/receipt", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/trace-imports/trace_import_id/receipt",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_traceImports_retry_trace_import() -> None:
    """Test retry_trace_import endpoint with WireMock"""
    test_id = "trace_imports.retry_trace_import.0"
    client = get_client(test_id)
    client.trace_imports.retry_trace_import(
        project_id="project_id",
        trace_import_id="trace_import_id",
        expected_version=1,
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/trace-imports/trace_import_id/retry", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/trace-imports/trace_import_id/retry",
        {"Authorization": r"Bearer .+"},
        [],
    )
