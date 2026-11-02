from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_api import JudgmentTarget, TargetInput


def test_evaluationJobs_list_jobs() -> None:
    """Test list_jobs endpoint with WireMock"""
    test_id = "evaluation_jobs.list_jobs.0"
    client = get_client(test_id)
    client.evaluation_jobs.list_jobs(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/evaluation-jobs", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/evaluation-jobs", {"Authorization": r"Bearer .+"}, [])


def test_evaluationJobs_create_job() -> None:
    """Test create_job endpoint with WireMock"""
    test_id = "evaluation_jobs.create_job.0"
    client = get_client(test_id)
    client.evaluation_jobs.create_job(
        project_id="project_id",
        evaluator="evaluator",
        request_id="requestId",
        revision="revision",
        rubric="rubric",
        targets=[
            TargetInput(
                target=JudgmentTarget(
                    account_id="accountId",
                    kind="message",
                ),
                trace_id="traceId",
            )
        ],
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/evaluation-jobs", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/evaluation-jobs", {"Authorization": r"Bearer .+"}, [])


def test_evaluationJobs_get_job() -> None:
    """Test get_job endpoint with WireMock"""
    test_id = "evaluation_jobs.get_job.0"
    client = get_client(test_id)
    client.evaluation_jobs.get_job(
        project_id="project_id",
        job_id="job_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/evaluation-jobs/job_id", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/evaluation-jobs/job_id", {"Authorization": r"Bearer .+"}, []
    )


def test_evaluationJobs_cancel_job() -> None:
    """Test cancel_job endpoint with WireMock"""
    test_id = "evaluation_jobs.cancel_job.0"
    client = get_client(test_id)
    client.evaluation_jobs.cancel_job(
        project_id="project_id",
        job_id="job_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/evaluation-jobs/job_id/cancel", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/evaluation-jobs/job_id/cancel", {"Authorization": r"Bearer .+"}, []
    )
