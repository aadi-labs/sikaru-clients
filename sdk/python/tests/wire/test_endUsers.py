from .conftest import get_client, verify_auth_headers, verify_request_count


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


def test_endUsers_update_fact() -> None:
    """Test update_fact endpoint with WireMock"""
    test_id = "end_users.update_fact.0"
    client = get_client(test_id)
    client.end_users.update_fact(
        project_id="project_id",
        agent_slug="agent_slug",
        tenant_id="tenant_id",
        user_id="user_id",
        content="content",
        path="path",
    )
    verify_request_count(
        test_id, "PUT", "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id/facts", None, 1
    )
    verify_auth_headers(
        test_id,
        "PUT",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id/facts",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_endUsers_delete_fact() -> None:
    """Test delete_fact endpoint with WireMock"""
    test_id = "end_users.delete_fact.0"
    client = get_client(test_id)
    client.end_users.delete_fact(
        project_id="project_id",
        agent_slug="agent_slug",
        tenant_id="tenant_id",
        user_id="user_id",
        path="path",
    )
    verify_request_count(
        test_id,
        "DELETE",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id/facts",
        {"path": "path"},
        1,
    )
    verify_auth_headers(
        test_id,
        "DELETE",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id/facts",
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
