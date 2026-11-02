from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_api import ToolSkillSourceRequest


def test_toolProviders_register_tool_provider() -> None:
    """Test register_tool_provider endpoint with WireMock"""
    test_id = "tool_providers.register_tool_provider.0"
    client = get_client(test_id)
    client.tool_providers.register_tool_provider(
        project_id="project_id",
        broker_endpoint_ref="broker_endpoint_ref",
        capability_prefix="capability_prefix",
        display_name="display_name",
        provider_type="provider_type",
        tool_catalog_ref="tool_catalog_ref",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/tool-providers", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/tool-providers", {"Authorization": r"Bearer .+"}, [])


def test_toolProviders_attach_source_tool_skill() -> None:
    """Test attach_source_tool_skill endpoint with WireMock"""
    test_id = "tool_providers.attach_source_tool_skill.0"
    client = get_client(test_id)
    client.tool_providers.attach_source_tool_skill(
        project_id="project_id",
        tool_provider_id="tool_provider_id",
        capability_refs=["capability_refs"],
        description="description",
        source=ToolSkillSourceRequest(
            kind="kind",
        ),
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/tool-providers/tool_provider_id/skills", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/tool-providers/tool_provider_id/skills",
        {"Authorization": r"Bearer .+"},
        [],
    )
