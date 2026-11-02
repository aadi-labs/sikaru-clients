from .conftest import get_client, verify_auth_headers, verify_request_count


def test_agents_create_managed_session() -> None:
    """Test create_managed_session endpoint with WireMock"""
    test_id = "agents.create_managed_session.0"
    client = get_client(test_id)
    client.agents.create_managed_session(
        project_id="project_id",
        agent_id="agent_id",
        environment_id="environmentId",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/agents/agent_id/sessions", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/agents/agent_id/sessions", {"Authorization": r"Bearer .+"}, []
    )
