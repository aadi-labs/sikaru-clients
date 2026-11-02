from .conftest import get_client, verify_auth_headers, verify_request_count


def test_judgeAlignment_get_judge_alignment() -> None:
    """Test get_judge_alignment endpoint with WireMock"""
    test_id = "judge_alignment.get_judge_alignment.0"
    client = get_client(test_id)
    client.judge_alignment.get_judge_alignment(
        project_id="project_id",
        evaluator="evaluator",
        revision="revision",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/judge-alignment", {"evaluator": "evaluator", "revision": "revision"}, 1
    )
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/judge-alignment", {"Authorization": r"Bearer .+"}, [])
