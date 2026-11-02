from .conftest import get_client, verify_auth_headers, verify_request_count


def test_reviewQueue_create_review_queue_item() -> None:
    """Test create_review_queue_item endpoint with WireMock"""
    test_id = "review_queue.create_review_queue_item.0"
    client = get_client(test_id)
    client.review_queue.create_review_queue_item(
        project_id="project_id",
        trace_ids=["traceIds"],
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/review-queue", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/review-queue", {"Authorization": r"Bearer .+"}, [])
