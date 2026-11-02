from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_api import ConnectionConfig, ConnectionCredentials


def test_connections_list_connections() -> None:
    """Test list_connections endpoint with WireMock"""
    test_id = "connections.list_connections.0"
    client = get_client(test_id)
    client.connections.list_connections(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/connections", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/connections", {"Authorization": r"Bearer .+"}, [])


def test_connections_create_connection() -> None:
    """Test create_connection endpoint with WireMock"""
    test_id = "connections.create_connection.0"
    client = get_client(test_id)
    client.connections.create_connection(
        project_id="project_id",
        config=ConnectionConfig(),
        display_name="display_name",
        kind="mcp",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/connections", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/connections", {"Authorization": r"Bearer .+"}, [])


def test_connections_list_apps() -> None:
    """Test list_apps endpoint with WireMock"""
    test_id = "connections.list_apps.0"
    client = get_client(test_id)
    client.connections.list_apps(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/connections/catalog/apps", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/connections/catalog/apps", {"Authorization": r"Bearer .+"}, []
    )


def test_connections_get_connection() -> None:
    """Test get_connection endpoint with WireMock"""
    test_id = "connections.get_connection.0"
    client = get_client(test_id)
    client.connections.get_connection(
        project_id="project_id",
        connection_id="connection_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/connections/connection_id", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/connections/connection_id", {"Authorization": r"Bearer .+"}, []
    )


def test_connections_update_connection() -> None:
    """Test update_connection endpoint with WireMock"""
    test_id = "connections.update_connection.0"
    client = get_client(test_id)
    client.connections.update_connection(
        project_id="project_id",
        connection_id="connection_id",
        expected_version=1,
    )
    verify_request_count(test_id, "PATCH", "/v1/projects/project_id/connections/connection_id", None, 1)
    verify_auth_headers(
        test_id, "PATCH", "/v1/projects/project_id/connections/connection_id", {"Authorization": r"Bearer .+"}, []
    )


def test_connections_authorize() -> None:
    """Test authorize endpoint with WireMock"""
    test_id = "connections.authorize.0"
    client = get_client(test_id)
    client.connections.authorize(
        project_id="project_id",
        connection_id="connection_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/connections/connection_id/authorize", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/connections/connection_id/authorize",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_connections_complete() -> None:
    """Test complete endpoint with WireMock"""
    test_id = "connections.complete.0"
    client = get_client(test_id)
    client.connections.complete(
        project_id="project_id",
        connection_id="connection_id",
        state="state",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/connections/connection_id/complete", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/connections/connection_id/complete",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_connections_credentials() -> None:
    """Test credentials endpoint with WireMock"""
    test_id = "connections.credentials.0"
    client = get_client(test_id)
    client.connections.credentials(
        project_id="project_id",
        connection_id="connection_id",
        credentials=ConnectionCredentials(),
    )
    verify_request_count(test_id, "PUT", "/v1/projects/project_id/connections/connection_id/credentials", None, 1)
    verify_auth_headers(
        test_id,
        "PUT",
        "/v1/projects/project_id/connections/connection_id/credentials",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_connections_disable() -> None:
    """Test disable endpoint with WireMock"""
    test_id = "connections.disable.0"
    client = get_client(test_id)
    client.connections.disable(
        project_id="project_id",
        connection_id="connection_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/connections/connection_id/disable", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/connections/connection_id/disable",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_connections_disconnect() -> None:
    """Test disconnect endpoint with WireMock"""
    test_id = "connections.disconnect.0"
    client = get_client(test_id)
    client.connections.disconnect(
        project_id="project_id",
        connection_id="connection_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/connections/connection_id/disconnect", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/connections/connection_id/disconnect",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_connections_discover() -> None:
    """Test discover endpoint with WireMock"""
    test_id = "connections.discover.0"
    client = get_client(test_id)
    client.connections.discover(
        project_id="project_id",
        connection_id="connection_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/connections/connection_id/discover", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/connections/connection_id/discover",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_connections_enable() -> None:
    """Test enable endpoint with WireMock"""
    test_id = "connections.enable.0"
    client = get_client(test_id)
    client.connections.enable(
        project_id="project_id",
        connection_id="connection_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/connections/connection_id/enable", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/connections/connection_id/enable", {"Authorization": r"Bearer .+"}, []
    )


def test_connections_events() -> None:
    """Test events endpoint with WireMock"""
    test_id = "connections.events.0"
    client = get_client(test_id)
    client.connections.events(
        project_id="project_id",
        connection_id="connection_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/connections/connection_id/events", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/connections/connection_id/events", {"Authorization": r"Bearer .+"}, []
    )


def test_connections_grant() -> None:
    """Test grant endpoint with WireMock"""
    test_id = "connections.grant.0"
    client = get_client(test_id)
    client.connections.grant(
        project_id="project_id",
        connection_id="connection_id",
        agent_id="agent_id",
        tools=["tools"],
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/connections/connection_id/grants", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/connections/connection_id/grants", {"Authorization": r"Bearer .+"}, []
    )


def test_connections_revoke_grant() -> None:
    """Test revoke_grant endpoint with WireMock"""
    test_id = "connections.revoke_grant.0"
    client = get_client(test_id)
    client.connections.revoke_grant(
        project_id="project_id",
        connection_id="connection_id",
        grant_id="grant_id",
    )
    verify_request_count(
        test_id, "DELETE", "/v1/projects/project_id/connections/connection_id/grants/grant_id", None, 1
    )
    verify_auth_headers(
        test_id,
        "DELETE",
        "/v1/projects/project_id/connections/connection_id/grants/grant_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_connections_revoke() -> None:
    """Test revoke endpoint with WireMock"""
    test_id = "connections.revoke.0"
    client = get_client(test_id)
    client.connections.revoke(
        project_id="project_id",
        connection_id="connection_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/connections/connection_id/revoke", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/connections/connection_id/revoke", {"Authorization": r"Bearer .+"}, []
    )


def test_connections_usage() -> None:
    """Test usage endpoint with WireMock"""
    test_id = "connections.usage.0"
    client = get_client(test_id)
    client.connections.usage(
        project_id="project_id",
        connection_id="connection_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/connections/connection_id/usage", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/connections/connection_id/usage", {"Authorization": r"Bearer .+"}, []
    )
