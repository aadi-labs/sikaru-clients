from .conftest import get_client, verify_auth_headers, verify_request_count


def test_runWebhooks_list_webhooks() -> None:
    """Test list_webhooks endpoint with WireMock"""
    test_id = "run_webhooks.list_webhooks.0"
    client = get_client(test_id)
    client.run_webhooks.list_webhooks(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/run-webhooks", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/run-webhooks", {"Authorization": r"Bearer .+"}, [])


def test_runWebhooks_create_webhook() -> None:
    """Test create_webhook endpoint with WireMock"""
    test_id = "run_webhooks.create_webhook.0"
    client = get_client(test_id)
    client.run_webhooks.create_webhook(
        project_id="project_id",
        run_id="run_id",
        url="url",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/run-webhooks", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/run-webhooks", {"Authorization": r"Bearer .+"}, [])


def test_runWebhooks_delete_webhook() -> None:
    """Test delete_webhook endpoint with WireMock"""
    test_id = "run_webhooks.delete_webhook.0"
    client = get_client(test_id)
    client.run_webhooks.delete_webhook(
        project_id="project_id",
        webhook_id="webhook_id",
    )
    verify_request_count(test_id, "DELETE", "/v1/projects/project_id/run-webhooks/webhook_id", None, 1)
    verify_auth_headers(
        test_id, "DELETE", "/v1/projects/project_id/run-webhooks/webhook_id", {"Authorization": r"Bearer .+"}, []
    )
