from .conftest import get_client, verify_auth_headers, verify_request_count


def test_evaluationCriteria_list_criteria() -> None:
    """Test list_criteria endpoint with WireMock"""
    test_id = "evaluation_criteria.list_criteria.0"
    client = get_client(test_id)
    client.evaluation_criteria.list_criteria(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/evaluation-criteria", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/evaluation-criteria", {"Authorization": r"Bearer .+"}, []
    )
