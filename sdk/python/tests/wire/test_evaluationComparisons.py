from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_api import AnswerRef, CaseInput


def test_evaluationComparisons_list_comparisons() -> None:
    """Test list_comparisons endpoint with WireMock"""
    test_id = "evaluation_comparisons.list_comparisons.0"
    client = get_client(test_id)
    client.evaluation_comparisons.list_comparisons(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/evaluation-comparisons", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/evaluation-comparisons", {"Authorization": r"Bearer .+"}, []
    )


def test_evaluationComparisons_create_comparison() -> None:
    """Test create_comparison endpoint with WireMock"""
    test_id = "evaluation_comparisons.create_comparison.0"
    client = get_client(test_id)
    client.evaluation_comparisons.create_comparison(
        project_id="project_id",
        cases=[
            CaseInput(
                baseline=AnswerRef(
                    account_id="accountId",
                    conversation_id="conversationId",
                    message_id="messageId",
                ),
                candidate=AnswerRef(
                    account_id="accountId",
                    conversation_id="conversationId",
                    message_id="messageId",
                ),
                case_id="caseId",
            )
        ],
        evaluator="evaluator",
        id="id",
        revision="revision",
        rubric="rubric",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/evaluation-comparisons", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/evaluation-comparisons", {"Authorization": r"Bearer .+"}, []
    )


def test_evaluationComparisons_get_comparison() -> None:
    """Test get_comparison endpoint with WireMock"""
    test_id = "evaluation_comparisons.get_comparison.0"
    client = get_client(test_id)
    client.evaluation_comparisons.get_comparison(
        project_id="project_id",
        comparison_id="comparison_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/evaluation-comparisons/comparison_id", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/evaluation-comparisons/comparison_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_evaluationComparisons_cancel_comparison() -> None:
    """Test cancel_comparison endpoint with WireMock"""
    test_id = "evaluation_comparisons.cancel_comparison.0"
    client = get_client(test_id)
    client.evaluation_comparisons.cancel_comparison(
        project_id="project_id",
        comparison_id="comparison_id",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/evaluation-comparisons/comparison_id/cancel", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/evaluation-comparisons/comparison_id/cancel",
        {"Authorization": r"Bearer .+"},
        [],
    )
