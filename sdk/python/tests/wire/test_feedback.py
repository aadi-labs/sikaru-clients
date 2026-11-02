from .conftest import get_client, verify_auth_headers, verify_request_count


def test_feedback_create_feedback() -> None:
    """Test create_feedback endpoint with WireMock"""
    test_id = "feedback.create_feedback.0"
    client = get_client(test_id)
    client.feedback.create_feedback(
        project_id="project_id",
        kind="thumbs_up",
        target="trace",
        target_id="targetId",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/feedback", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/feedback", {"Authorization": r"Bearer .+"}, [])
