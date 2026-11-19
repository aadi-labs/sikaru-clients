from .conftest import get_client, verify_auth_headers, verify_request_count


def test_managedAgents_list_managed_agents() -> None:
    """Test list_managed_agents endpoint with WireMock"""
    test_id = "managed_agents.list_managed_agents.0"
    client = get_client(test_id)
    client.managed_agents.list_managed_agents(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/managed-agents", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/managed-agents", {"Authorization": r"Bearer .+"}, [])


def test_managedAgents_create_managed_agent() -> None:
    """Test create_managed_agent endpoint with WireMock"""
    test_id = "managed_agents.create_managed_agent.0"
    client = get_client(test_id)
    client.managed_agents.create_managed_agent(
        project_id="project_id",
        agent_slug="agentSlug",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/managed-agents", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/managed-agents", {"Authorization": r"Bearer .+"}, [])


def test_managedAgents_get() -> None:
    """Test get endpoint with WireMock"""
    test_id = "managed_agents.get.0"
    client = get_client(test_id)
    client.managed_agents.get(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug", {"Authorization": r"Bearer .+"}, []
    )


def test_managedAgents_delete_managed_agent() -> None:
    """Test delete_managed_agent endpoint with WireMock"""
    test_id = "managed_agents.delete_managed_agent.0"
    client = get_client(test_id)
    client.managed_agents.delete_managed_agent(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(test_id, "DELETE", "/v1/projects/project_id/managed-agents/agent_slug", None, 1)
    verify_auth_headers(
        test_id, "DELETE", "/v1/projects/project_id/managed-agents/agent_slug", {"Authorization": r"Bearer .+"}, []
    )


def test_managedAgents_update() -> None:
    """Test update endpoint with WireMock"""
    test_id = "managed_agents.update.0"
    client = get_client(test_id)
    client.managed_agents.update(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(test_id, "PATCH", "/v1/projects/project_id/managed-agents/agent_slug", None, 1)
    verify_auth_headers(
        test_id, "PATCH", "/v1/projects/project_id/managed-agents/agent_slug", {"Authorization": r"Bearer .+"}, []
    )


def test_managedAgents_list_agent_logins() -> None:
    """Test list_agent_logins endpoint with WireMock"""
    test_id = "managed_agents.list_agent_logins.0"
    client = get_client(test_id)
    client.managed_agents.list_agent_logins(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/browser-logins", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/browser-logins",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_managedAgents_add_agent_login() -> None:
    """Test add_agent_login endpoint with WireMock"""
    test_id = "managed_agents.add_agent_login.0"
    client = get_client(test_id)
    client.managed_agents.add_agent_login(
        project_id="project_id",
        agent_slug="agent_slug",
        url="url",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/managed-agents/agent_slug/browser-logins", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/browser-logins",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_managedAgents_delete_agent_login() -> None:
    """Test delete_agent_login endpoint with WireMock"""
    test_id = "managed_agents.delete_agent_login.0"
    client = get_client(test_id)
    client.managed_agents.delete_agent_login(
        project_id="project_id",
        agent_slug="agent_slug",
        login_id="login_id",
    )
    verify_request_count(
        test_id, "DELETE", "/v1/projects/project_id/managed-agents/agent_slug/browser-logins/login_id", None, 1
    )
    verify_auth_headers(
        test_id,
        "DELETE",
        "/v1/projects/project_id/managed-agents/agent_slug/browser-logins/login_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_managedAgents_list_playbooks() -> None:
    """Test list_playbooks endpoint with WireMock"""
    test_id = "managed_agents.list_playbooks.0"
    client = get_client(test_id)
    client.managed_agents.list_playbooks(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/browser-playbooks", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/browser-playbooks",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_managedAgents_delete_playbook() -> None:
    """Test delete_playbook endpoint with WireMock"""
    test_id = "managed_agents.delete_playbook.0"
    client = get_client(test_id)
    client.managed_agents.delete_playbook(
        project_id="project_id",
        agent_slug="agent_slug",
        playbook_id="playbook_id",
    )
    verify_request_count(
        test_id, "DELETE", "/v1/projects/project_id/managed-agents/agent_slug/browser-playbooks/playbook_id", None, 1
    )
    verify_auth_headers(
        test_id,
        "DELETE",
        "/v1/projects/project_id/managed-agents/agent_slug/browser-playbooks/playbook_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_managedAgents_list_rules() -> None:
    """Test list_rules endpoint with WireMock"""
    test_id = "managed_agents.list_rules.0"
    client = get_client(test_id)
    client.managed_agents.list_rules(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/browser-rules", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/browser-rules",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_managedAgents_delete_rule() -> None:
    """Test delete_rule endpoint with WireMock"""
    test_id = "managed_agents.delete_rule.0"
    client = get_client(test_id)
    client.managed_agents.delete_rule(
        project_id="project_id",
        agent_slug="agent_slug",
        rule_id="rule_id",
    )
    verify_request_count(
        test_id, "DELETE", "/v1/projects/project_id/managed-agents/agent_slug/browser-rules/rule_id", None, 1
    )
    verify_auth_headers(
        test_id,
        "DELETE",
        "/v1/projects/project_id/managed-agents/agent_slug/browser-rules/rule_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_managedAgents_get_definition_revision() -> None:
    """Test get_definition_revision endpoint with WireMock"""
    test_id = "managed_agents.get_definition_revision.0"
    client = get_client(test_id)
    client.managed_agents.get_definition_revision(
        project_id="project_id",
        agent_slug="agent_slug",
        changeset_id="changeset_id",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/definition-revisions/changeset_id", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/definition-revisions/changeset_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_managedAgents_update_end_user_skill_source() -> None:
    """Test update_end_user_skill_source endpoint with WireMock"""
    test_id = "managed_agents.update_end_user_skill_source.0"
    client = get_client(test_id)
    client.managed_agents.update_end_user_skill_source(
        project_id="project_id",
        agent_slug="agent_slug",
        tenant_id="tenant_id",
        user_id="user_id",
        skill_id="skill_id",
        enabled=True,
        operation_id="operationId",
        source_revision="sourceRevision",
    )
    verify_request_count(
        test_id,
        "PUT",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id/skill-sources/skill_id",
        None,
        1,
    )
    verify_auth_headers(
        test_id,
        "PUT",
        "/v1/projects/project_id/managed-agents/agent_slug/end-users/tenant_id/user_id/skill-sources/skill_id",
        {"Authorization": r"Bearer .+"},
        [],
    )
