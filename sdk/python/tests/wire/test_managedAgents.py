from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_sdk import AgentDefinition


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


def test_managedAgents_create_definition_revision() -> None:
    """Test create_definition_revision endpoint with WireMock"""
    test_id = "managed_agents.create_definition_revision.0"
    client = get_client(test_id)
    client.managed_agents.create_definition_revision(
        project_id="project_id",
        agent_slug="agent_slug",
        content_digest="contentDigest",
        definition=AgentDefinition(
            schema="sikaru.agent.contract.v1",
        ),
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/managed-agents/agent_slug/definition-revisions", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/definition-revisions",
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
