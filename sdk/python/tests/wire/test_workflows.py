from .conftest import get_client, verify_auth_headers, verify_request_count


def test_workflows_import_workflow() -> None:
    """Test import_workflow endpoint with WireMock"""
    test_id = "workflows.import_workflow.0"
    client = get_client(test_id)
    client.workflows.import_workflow(
        project_id="project_id",
        payload={"key": "value"},
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/workflows/import", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/workflows/import", {"Authorization": r"Bearer .+"}, []
    )


def test_workflows_export_product_workflow() -> None:
    """Test export_product_workflow endpoint with WireMock"""
    test_id = "workflows.export_product_workflow.0"
    client = get_client(test_id)
    client.workflows.export_product_workflow(
        project_id="project_id",
        workflow_id="workflow_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/workflows/workflow_id/export", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/workflows/workflow_id/export", {"Authorization": r"Bearer .+"}, []
    )


def test_workflows_start_project_workflow_run() -> None:
    """Test start_project_workflow_run endpoint with WireMock"""
    test_id = "workflows.start_project_workflow_run.0"
    client = get_client(test_id)
    client.workflows.start_project_workflow_run(
        project_id="project_id",
        workflow_id="workflow_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/workflows/workflow_id/runs", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/workflows/workflow_id/runs", {"Authorization": r"Bearer .+"}, []
    )


def test_workflows_create_project_workflow_version() -> None:
    """Test create_project_workflow_version endpoint with WireMock"""
    test_id = "workflows.create_project_workflow_version.0"
    client = get_client(test_id)
    client.workflows.create_project_workflow_version(
        project_id="project_id",
        workflow_id="workflow_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/workflows/workflow_id/versions", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/workflows/workflow_id/versions", {"Authorization": r"Bearer .+"}, []
    )
