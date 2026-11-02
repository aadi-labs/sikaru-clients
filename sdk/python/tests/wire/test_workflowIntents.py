from .conftest import get_client, verify_auth_headers, verify_request_count


def test_workflowIntents_create_project_workflow_intent() -> None:
    """Test create_project_workflow_intent endpoint with WireMock"""
    test_id = "workflow_intents.create_project_workflow_intent.0"
    client = get_client(test_id)
    client.workflow_intents.create_project_workflow_intent(
        project_id="project_id",
        description="description",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/workflow-intents", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/workflow-intents", {"Authorization": r"Bearer .+"}, []
    )


def test_workflowIntents_compile_project_workflow_intent() -> None:
    """Test compile_project_workflow_intent endpoint with WireMock"""
    test_id = "workflow_intents.compile_project_workflow_intent.0"
    client = get_client(test_id)
    client.workflow_intents.compile_project_workflow_intent(
        project_id="project_id",
        intent_id="intent_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/workflow-intents/intent_id/compile", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/workflow-intents/intent_id/compile",
        {"Authorization": r"Bearer .+"},
        [],
    )
