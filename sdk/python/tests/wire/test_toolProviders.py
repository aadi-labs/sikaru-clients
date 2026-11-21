from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_sdk import ToolSkillSourceRequest


def test_toolProviders_list_tool_providers() -> None:
    """Test list_tool_providers endpoint with WireMock"""
    test_id = "tool_providers.list_tool_providers.0"
    client = get_client(test_id)
    client.tool_providers.list_tool_providers(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/tool-providers", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/tool-providers", {"Authorization": r"Bearer .+"}, [])


def test_toolProviders_get_tool_provider() -> None:
    """Test get_tool_provider endpoint with WireMock"""
    test_id = "tool_providers.get_tool_provider.0"
    client = get_client(test_id)
    client.tool_providers.get_tool_provider(
        project_id="project_id",
        capability_prefix="capability_prefix",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/tool-providers/capability_prefix", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/tool-providers/capability_prefix", {"Authorization": r"Bearer .+"}, []
    )


def test_toolProviders_apply_tool_provider() -> None:
    """Test apply_tool_provider endpoint with WireMock"""
    test_id = "tool_providers.apply_tool_provider.0"
    client = get_client(test_id)
    client.tool_providers.apply_tool_provider(
        project_id="project_id",
        capability_prefix="capability_prefix",
        broker_endpoint_ref="broker_endpoint_ref",
        display_name="display_name",
        provider_type="provider_type",
        tool_catalog_ref="tool_catalog_ref",
    )
    verify_request_count(test_id, "PUT", "/v1/projects/project_id/tool-providers/capability_prefix", None, 1)
    verify_auth_headers(
        test_id, "PUT", "/v1/projects/project_id/tool-providers/capability_prefix", {"Authorization": r"Bearer .+"}, []
    )


def test_toolProviders_expire_previous_tool_provider_secret() -> None:
    """Test expire_previous_tool_provider_secret endpoint with WireMock"""
    test_id = "tool_providers.expire_previous_tool_provider_secret.0"
    client = get_client(test_id)
    client.tool_providers.expire_previous_tool_provider_secret(
        project_id="project_id",
        capability_prefix="capability_prefix",
    )
    verify_request_count(
        test_id, "DELETE", "/v1/projects/project_id/tool-providers/capability_prefix/secret/previous", None, 1
    )
    verify_auth_headers(
        test_id,
        "DELETE",
        "/v1/projects/project_id/tool-providers/capability_prefix/secret/previous",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_toolProviders_rotate_tool_provider_secret() -> None:
    """Test rotate_tool_provider_secret endpoint with WireMock"""
    test_id = "tool_providers.rotate_tool_provider_secret.0"
    client = get_client(test_id)
    client.tool_providers.rotate_tool_provider_secret(
        project_id="project_id",
        capability_prefix="capability_prefix",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/tool-providers/capability_prefix/secret/rotate", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/tool-providers/capability_prefix/secret/rotate",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_toolProviders_attach_source_tool_skill() -> None:
    """Test attach_source_tool_skill endpoint with WireMock"""
    test_id = "tool_providers.attach_source_tool_skill.0"
    client = get_client(test_id)
    client.tool_providers.attach_source_tool_skill(
        project_id="project_id",
        capability_prefix="capability_prefix",
        capability_refs=["capability_refs"],
        description="description",
        source=ToolSkillSourceRequest(
            kind="kind",
        ),
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/tool-providers/capability_prefix/skills", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/tool-providers/capability_prefix/skills",
        {"Authorization": r"Bearer .+"},
        [],
    )
