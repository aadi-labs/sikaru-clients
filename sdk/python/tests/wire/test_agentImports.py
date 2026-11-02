from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_api import AgentImportImproveRequest


def test_agentImports_list_agent_imports() -> None:
    """Test list_agent_imports endpoint with WireMock"""
    test_id = "agent_imports.list_agent_imports.0"
    client = get_client(test_id)
    client.agent_imports.list_agent_imports(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/agent-imports", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/agent-imports", {"Authorization": r"Bearer .+"}, [])


def test_agentImports_create_agent_import() -> None:
    """Test create_agent_import endpoint with WireMock"""
    test_id = "agent_imports.create_agent_import.0"
    client = get_client(test_id)
    client.agent_imports.create_agent_import(
        project_id="project_id",
        improve=AgentImportImproveRequest(
            mode="mode",
            objective="objective",
            promotion_gate="promotionGate",
        ),
        name="name",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/agent-imports", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/agent-imports", {"Authorization": r"Bearer .+"}, [])
