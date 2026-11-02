from .conftest import get_client, verify_auth_headers, verify_request_count


def test_onlineEvaluations_list_policies() -> None:
    """Test list_policies endpoint with WireMock"""
    test_id = "online_evaluations.list_policies.0"
    client = get_client(test_id)
    client.online_evaluations.list_policies(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/online-evaluations", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/online-evaluations", {"Authorization": r"Bearer .+"}, []
    )


def test_onlineEvaluations_create_policy() -> None:
    """Test create_policy endpoint with WireMock"""
    test_id = "online_evaluations.create_policy.0"
    client = get_client(test_id)
    client.online_evaluations.create_policy(
        project_id="project_id",
        evaluator="evaluator",
        id="id",
        revision="revision",
        rubric="rubric",
        sample_percent=1,
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/online-evaluations", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/online-evaluations", {"Authorization": r"Bearer .+"}, []
    )


def test_onlineEvaluations_preview_policy_eligibility() -> None:
    """Test preview_policy_eligibility endpoint with WireMock"""
    test_id = "online_evaluations.preview_policy_eligibility.0"
    client = get_client(test_id)
    client.online_evaluations.preview_policy_eligibility(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/online-evaluations/preview", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/online-evaluations/preview", {"Authorization": r"Bearer .+"}, []
    )


def test_onlineEvaluations_update_policy() -> None:
    """Test update_policy endpoint with WireMock"""
    test_id = "online_evaluations.update_policy.0"
    client = get_client(test_id)
    client.online_evaluations.update_policy(
        project_id="project_id",
        policy_id="policy_id",
        enabled=True,
    )
    verify_request_count(test_id, "PATCH", "/v1/projects/project_id/online-evaluations/policy_id", None, 1)
    verify_auth_headers(
        test_id, "PATCH", "/v1/projects/project_id/online-evaluations/policy_id", {"Authorization": r"Bearer .+"}, []
    )
