from .conftest import get_client, verify_auth_headers, verify_request_count


def test_computeOperations_submit_receipt() -> None:
    """Test submit_receipt endpoint with WireMock"""
    test_id = "compute_operations.submit_receipt.0"
    client = get_client(test_id)
    client.compute_operations.submit_receipt(
        project_id="project_id",
        attachment_id="attachment_id",
        idempotency_key="idempotency_key",
        payload={"key": "value"},
        request_digest="request_digest",
        run_id="run_id",
        status="completed",
        tool_call_id="tool_call_id",
        tool_provider_id="tool_provider_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/compute-attachments/attachment_id/receipts", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/compute-attachments/attachment_id/receipts",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_computeOperations_poll() -> None:
    """Test poll endpoint with WireMock"""
    test_id = "compute_operations.poll.0"
    client = get_client(test_id)
    client.compute_operations.poll(
        project_id="project_id",
        attachment_id="attachment_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/compute-attachments/attachment_id/work", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/compute-attachments/attachment_id/work",
        {"Authorization": r"Bearer .+"},
        [],
    )
