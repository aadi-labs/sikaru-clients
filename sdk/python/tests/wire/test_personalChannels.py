from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_api import ConnectionCredentials


def test_personalChannels_get_message() -> None:
    """Test get_message endpoint with WireMock"""
    test_id = "personal_channels.get_message.0"
    client = get_client(test_id)
    client.personal_channels.get_message(
        binding_id="binding_id",
        receipt_id="receipt_id",
    )
    verify_request_count(test_id, "GET", "/v1/personal-channel-bindings/binding_id/messages/receipt_id", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/personal-channel-bindings/binding_id/messages/receipt_id", {}, ["Authorization"]
    )


def test_personalChannels_authorize_connection() -> None:
    """Test authorize_connection endpoint with WireMock"""
    test_id = "personal_channels.authorize_connection.0"
    client = get_client(test_id)
    client.personal_channels.authorize_connection(
        binding_id="binding_id",
        receipt_id="receipt_id",
        connection_id="connection_id",
    )
    verify_request_count(
        test_id,
        "POST",
        "/v1/personal-channel-bindings/binding_id/messages/receipt_id/connections/connection_id/authorize",
        None,
        1,
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/personal-channel-bindings/binding_id/messages/receipt_id/connections/connection_id/authorize",
        {},
        ["Authorization"],
    )


def test_personalChannels_complete_connection() -> None:
    """Test complete_connection endpoint with WireMock"""
    test_id = "personal_channels.complete_connection.0"
    client = get_client(test_id)
    client.personal_channels.complete_connection(
        binding_id="binding_id",
        receipt_id="receipt_id",
        connection_id="connection_id",
        state="state",
    )
    verify_request_count(
        test_id,
        "POST",
        "/v1/personal-channel-bindings/binding_id/messages/receipt_id/connections/connection_id/complete",
        None,
        1,
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/personal-channel-bindings/binding_id/messages/receipt_id/connections/connection_id/complete",
        {},
        ["Authorization"],
    )


def test_personalChannels_replace_connection_credentials() -> None:
    """Test replace_connection_credentials endpoint with WireMock"""
    test_id = "personal_channels.replace_connection_credentials.0"
    client = get_client(test_id)
    client.personal_channels.replace_connection_credentials(
        binding_id="binding_id",
        receipt_id="receipt_id",
        connection_id="connection_id",
        credentials=ConnectionCredentials(),
    )
    verify_request_count(
        test_id,
        "PUT",
        "/v1/personal-channel-bindings/binding_id/messages/receipt_id/connections/connection_id/credentials",
        None,
        1,
    )
    verify_auth_headers(
        test_id,
        "PUT",
        "/v1/personal-channel-bindings/binding_id/messages/receipt_id/connections/connection_id/credentials",
        {},
        ["Authorization"],
    )


def test_personalChannels_list_files() -> None:
    """Test list_files endpoint with WireMock"""
    test_id = "personal_channels.list_files.0"
    client = get_client(test_id)
    client.personal_channels.list_files(
        binding_id="binding_id",
        receipt_id="receipt_id",
    )
    verify_request_count(test_id, "GET", "/v1/personal-channel-bindings/binding_id/messages/receipt_id/files", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/personal-channel-bindings/binding_id/messages/receipt_id/files", {}, ["Authorization"]
    )


def test_personalChannels_download_file() -> None:
    """Test download_file endpoint with WireMock"""
    test_id = "personal_channels.download_file.0"
    client = get_client(test_id)
    for _ in client.personal_channels.download_file(
        binding_id="binding_id",
        receipt_id="receipt_id",
        file_id="file_id",
    ):
        pass
    verify_request_count(
        test_id, "GET", "/v1/personal-channel-bindings/binding_id/messages/receipt_id/files/file_id/content", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/personal-channel-bindings/binding_id/messages/receipt_id/files/file_id/content",
        {},
        ["Authorization"],
    )


def test_personalChannels_decide_approval() -> None:
    """Test decide_approval endpoint with WireMock"""
    test_id = "personal_channels.decide_approval.0"
    client = get_client(test_id)
    client.personal_channels.decide_approval(
        binding_id="binding_id",
        receipt_id="receipt_id",
        tool_call_id="tool_call_id",
        decision="approved",
        idempotency_key="idempotency_key",
    )
    verify_request_count(
        test_id,
        "POST",
        "/v1/personal-channel-bindings/binding_id/messages/receipt_id/tool-calls/tool_call_id/approval",
        None,
        1,
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/personal-channel-bindings/binding_id/messages/receipt_id/tool-calls/tool_call_id/approval",
        {},
        ["Authorization"],
    )


def test_personalChannels_get_slack_link() -> None:
    """Test get_slack_link endpoint with WireMock"""
    test_id = "personal_channels.get_slack_link.0"
    client = get_client(test_id)
    client.personal_channels.get_slack_link(
        binding_id="binding_id",
        verification_id="verification_id",
    )
    verify_request_count(
        test_id, "GET", "/v1/personal-channel-bindings/binding_id/slack-link-verifications/verification_id", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/personal-channel-bindings/binding_id/slack-link-verifications/verification_id",
        {},
        ["Authorization"],
    )


def test_personalChannels_start_slack_link() -> None:
    """Test start_slack_link endpoint with WireMock"""
    test_id = "personal_channels.start_slack_link.0"
    client = get_client(test_id)
    client.personal_channels.start_slack_link(
        binding_id="binding_id",
        installation_id="installation_id",
    )
    verify_request_count(test_id, "POST", "/v1/personal-channel-bindings/binding_id/slack-links", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/personal-channel-bindings/binding_id/slack-links", {}, ["Authorization"])


def test_personalChannels_unlink_slack_identity() -> None:
    """Test unlink_slack_identity endpoint with WireMock"""
    test_id = "personal_channels.unlink_slack_identity.0"
    client = get_client(test_id)
    client.personal_channels.unlink_slack_identity(
        binding_id="binding_id",
        installation_id="installation_id",
    )
    verify_request_count(
        test_id, "DELETE", "/v1/personal-channel-bindings/binding_id/slack-links/installation_id", None, 1
    )
    verify_auth_headers(
        test_id, "DELETE", "/v1/personal-channel-bindings/binding_id/slack-links/installation_id", {}, ["Authorization"]
    )
