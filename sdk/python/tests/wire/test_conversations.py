import datetime

from .conftest import get_client, verify_auth_headers, verify_request_count


def test_conversations_list_messages() -> None:
    """Test list_messages endpoint with WireMock"""
    test_id = "conversations.list_messages.0"
    client = get_client(test_id)
    client.conversations.list_messages(
        project_id="project_id",
        conversation_id="conversation_id",
        account_id="account_id",
    )
    verify_request_count(
        test_id,
        "GET",
        "/v1/projects/project_id/conversations/conversation_id/messages",
        {"account_id": "account_id"},
        1,
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/conversations/conversation_id/messages",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_conversations_record_message() -> None:
    """Test record_message endpoint with WireMock"""
    test_id = "conversations.record_message.0"
    client = get_client(test_id)
    client.conversations.record_message(
        project_id="project_id",
        conversation_id="conversation_id",
        account_id="account_id",
        content="content",
        delivered_at=datetime.datetime.fromisoformat("2024-01-15T09:30:00+00:00"),
        message_id="messageId",
        position=1,
        role="user",
    )
    verify_request_count(
        test_id,
        "POST",
        "/v1/projects/project_id/conversations/conversation_id/messages",
        {"account_id": "account_id"},
        1,
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/conversations/conversation_id/messages",
        {"Authorization": r"Bearer .+"},
        [],
    )
