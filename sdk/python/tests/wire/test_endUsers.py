from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_sdk import AuthoredLearningWorkflow, ImportFile, ProductLearningEvidence


def test_endUsers_get() -> None:
    """Test get endpoint with WireMock"""
    test_id = "end_users.get.0"
    client = get_client(test_id)
    client.end_users.get(
        project_id="project_id",
        agent_slug="agent_slug",
        tenant_id="tenant_id",
        user_id="user_id",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_endUsers_upsert() -> None:
    """Test upsert endpoint with WireMock"""
    test_id = "end_users.upsert.0"
    client = get_client(test_id)
    client.end_users.upsert(
        project_id="project_id",
        agent_slug="agent_slug",
        tenant_id="tenant_id",
        user_id="user_id",
    )
    verify_request_count(
        test_id, "PUT", "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id", None, 1
    )
    verify_auth_headers(
        test_id,
        "PUT",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_endUsers_delete() -> None:
    """Test delete endpoint with WireMock"""
    test_id = "end_users.delete.0"
    client = get_client(test_id)
    client.end_users.delete(
        project_id="project_id",
        agent_slug="agent_slug",
        tenant_id="tenant_id",
        user_id="user_id",
    )
    verify_request_count(
        test_id, "DELETE", "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id", None, 1
    )
    verify_auth_headers(
        test_id,
        "DELETE",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_endUsers_import_state() -> None:
    """Test import_state endpoint with WireMock"""
    test_id = "end_users.import_state.0"
    client = get_client(test_id)
    client.end_users.import_state(
        project_id="project_id",
        agent_slug="agent_slug",
        tenant_id="tenant_id",
        user_id="user_id",
        digest="digest",
        files=[
            ImportFile(
                content="content",
                expected_etag=None,
                path="path",
                provenance="provenance",
            )
        ],
        import_id="importId",
        source_revision="sourceRevision",
    )
    verify_request_count(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id/imports",
        None,
        1,
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id/imports",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_endUsers_start_learning_run() -> None:
    """Test start_learning_run endpoint with WireMock"""
    test_id = "end_users.start_learning_run.0"
    client = get_client(test_id)
    client.end_users.start_learning_run(
        project_id="project_id",
        agent_slug="agent_slug",
        tenant_id="tenant_id",
        user_id="user_id",
        evidence=[
            ProductLearningEvidence(
                content="content",
                digest="digest",
                learning_eligible=True,
                provenance="user_message",
                source_id="sourceId",
            )
        ],
        invocation_id="invocationId",
        source_revision="sourceRevision",
        workflow=AuthoredLearningWorkflow(
            name="name",
        ),
    )
    verify_request_count(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id/learning-runs",
        None,
        1,
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id/learning-runs",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_endUsers_get_learning_run() -> None:
    """Test get_learning_run endpoint with WireMock"""
    test_id = "end_users.get_learning_run.0"
    client = get_client(test_id)
    client.end_users.get_learning_run(
        project_id="project_id",
        agent_slug="agent_slug",
        tenant_id="tenant_id",
        user_id="user_id",
        invocation_id="invocation_id",
    )
    verify_request_count(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id/learning-runs/invocation_id",
        None,
        1,
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id/learning-runs/invocation_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_endUsers_revoke_learning_source() -> None:
    """Test revoke_learning_source endpoint with WireMock"""
    test_id = "end_users.revoke_learning_source.0"
    client = get_client(test_id)
    client.end_users.revoke_learning_source(
        project_id="project_id",
        agent_slug="agent_slug",
        tenant_id="tenant_id",
        user_id="user_id",
        source_id="source_id",
    )
    verify_request_count(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id/learning-sources/source_id/revoke",
        None,
        1,
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id/learning-sources/source_id/revoke",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_endUsers_correct_persona_summary() -> None:
    """Test correct_persona_summary endpoint with WireMock"""
    test_id = "end_users.correct_persona_summary.0"
    client = get_client(test_id)
    client.end_users.correct_persona_summary(
        project_id="project_id",
        agent_slug="agent_slug",
        tenant_id="tenant_id",
        user_id="user_id",
        text="text",
    )
    verify_request_count(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id/persona-summary/corrections",
        None,
        1,
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id/persona-summary/corrections",
        {"Authorization": r"Bearer .+"},
        [],
    )
