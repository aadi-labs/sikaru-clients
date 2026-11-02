from .conftest import get_client, verify_auth_headers, verify_request_count


def test_httpChannels_invoke() -> None:
    """Test invoke endpoint with WireMock"""
    test_id = "http_channels.invoke.0"
    client = get_client(test_id)
    client.http_channels.invoke(
        binding_id="binding_id",
        content="content",
        conversation_id="conversation_id",
        message_id="message_id",
    )
    verify_request_count(test_id, "POST", "/v1/channel-bindings/binding_id/messages", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/channel-bindings/binding_id/messages", {"Authorization": r"Bearer .+"}, []
    )


def test_httpChannels_poll() -> None:
    """Test poll endpoint with WireMock"""
    test_id = "http_channels.poll.0"
    client = get_client(test_id)
    client.http_channels.poll(
        binding_id="binding_id",
        receipt_id="receipt_id",
    )
    verify_request_count(test_id, "GET", "/v1/channel-bindings/binding_id/messages/receipt_id", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/channel-bindings/binding_id/messages/receipt_id", {"Authorization": r"Bearer .+"}, []
    )


def test_httpChannels_start_slack_link() -> None:
    """Test start_slack_link endpoint with WireMock"""
    test_id = "http_channels.start_slack_link.0"
    client = get_client(test_id)
    client.http_channels.start_slack_link(
        binding_id="binding_id",
        installation_id="installation_id",
    )
    verify_request_count(test_id, "POST", "/v1/channel-bindings/binding_id/slack-links", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/channel-bindings/binding_id/slack-links", {"Authorization": r"Bearer .+"}, []
    )


def test_httpChannels_unlink_slack_identity() -> None:
    """Test unlink_slack_identity endpoint with WireMock"""
    test_id = "http_channels.unlink_slack_identity.0"
    client = get_client(test_id)
    client.http_channels.unlink_slack_identity(
        binding_id="binding_id",
        installation_id="installation_id",
    )
    verify_request_count(test_id, "DELETE", "/v1/channel-bindings/binding_id/slack-links/installation_id", None, 1)
    verify_auth_headers(
        test_id,
        "DELETE",
        "/v1/channel-bindings/binding_id/slack-links/installation_id",
        {"Authorization": r"Bearer .+"},
        [],
    )
