from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_sdk import TakeOverRequest


def test_runs_start() -> None:
    """Test start endpoint with WireMock"""
    test_id = "runs.start.0"
    client = get_client(test_id)
    client.runs.start(
        project_id="project_id",
        harness_id="harness_id",
        input={"key": "value"},
        policy={"key": "value"},
        product_context={"key": "value"},
        tenant_id="tenant_id",
        user_id="user_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/harnesses/harness_id/runs", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/harnesses/harness_id/runs", {"Authorization": r"Bearer .+"}, []
    )


def test_runs_record() -> None:
    """Test record endpoint with WireMock"""
    test_id = "runs.record.0"
    client = get_client(test_id)
    client.runs.record(
        project_id="project_id",
        input="input",
        output="output",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/runs/record", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/runs/record", {"Authorization": r"Bearer .+"}, [])


def test_runs_resume_all() -> None:
    """Test resume_all endpoint with WireMock"""
    test_id = "runs.resume_all.0"
    client = get_client(test_id)
    client.runs.resume_all(
        project_id="project_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/runs/resume", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/runs/resume", {"Authorization": r"Bearer .+"}, [])


def test_runs_resume_count() -> None:
    """Test resume_count endpoint with WireMock"""
    test_id = "runs.resume_count.0"
    client = get_client(test_id)
    client.runs.resume_count(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/runs/resume/count", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/runs/resume/count", {"Authorization": r"Bearer .+"}, []
    )


def test_runs_get() -> None:
    """Test get endpoint with WireMock"""
    test_id = "runs.get.0"
    client = get_client(test_id)
    client.runs.get(
        project_id="project_id",
        run_id="run_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/runs/run_id", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/runs/run_id", {"Authorization": r"Bearer .+"}, [])


def test_runs_pending_actions() -> None:
    """Test pending_actions endpoint with WireMock"""
    test_id = "runs.pending_actions.0"
    client = get_client(test_id)
    client.runs.pending_actions(
        project_id="project_id",
        run_id="run_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/runs/run_id/actions", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/runs/run_id/actions", {"Authorization": r"Bearer .+"}, []
    )


def test_runs_create_handoff() -> None:
    """Test create_handoff endpoint with WireMock"""
    test_id = "runs.create_handoff.0"
    client = get_client(test_id)
    client.runs.create_handoff(
        project_id="project_id",
        run_id="run_id",
        tool_call_id="tool_call_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/runs/run_id/browser/handoffs/tool_call_id", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/runs/run_id/browser/handoffs/tool_call_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_runs_get_preview() -> None:
    """Test get_preview endpoint with WireMock"""
    test_id = "runs.get_preview.0"
    client = get_client(test_id)
    client.runs.get_preview(
        project_id="project_id",
        run_id="run_id",
        digest="digest",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/runs/run_id/browser/previews/digest", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/runs/run_id/browser/previews/digest",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_runs_preview_screenshot() -> None:
    """Test preview_screenshot endpoint with WireMock"""
    test_id = "runs.preview_screenshot.0"
    client = get_client(test_id)
    for _ in client.runs.preview_screenshot(
        project_id="project_id",
        run_id="run_id",
        digest="digest",
    ):
        pass
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/runs/run_id/browser/previews/digest/screenshot", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/runs/run_id/browser/previews/digest/screenshot",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_runs_get_receipt() -> None:
    """Test get_receipt endpoint with WireMock"""
    test_id = "runs.get_receipt.0"
    client = get_client(test_id)
    client.runs.get_receipt(
        project_id="project_id",
        run_id="run_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/runs/run_id/browser/receipt", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/runs/run_id/browser/receipt", {"Authorization": r"Bearer .+"}, []
    )


def test_runs_get_receipt_screenshot() -> None:
    """Test get_receipt_screenshot endpoint with WireMock"""
    test_id = "runs.get_receipt_screenshot.0"
    client = get_client(test_id)
    for _ in client.runs.get_receipt_screenshot(
        project_id="project_id",
        run_id="run_id",
        digest="digest",
    ):
        pass
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/runs/run_id/browser/receipt/screenshots/digest", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/runs/run_id/browser/receipt/screenshots/digest",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_runs_take_over() -> None:
    """Test take_over endpoint with WireMock"""
    test_id = "runs.take_over.0"
    client = get_client(test_id)
    client.runs.take_over(
        project_id="project_id",
        run_id="run_id",
        request=TakeOverRequest(),
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/runs/run_id/browser/takeover", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/runs/run_id/browser/takeover", {"Authorization": r"Bearer .+"}, []
    )


def test_runs_end_take_over() -> None:
    """Test end_take_over endpoint with WireMock"""
    test_id = "runs.end_take_over.0"
    client = get_client(test_id)
    client.runs.end_take_over(
        project_id="project_id",
        run_id="run_id",
    )
    verify_request_count(test_id, "DELETE", "/v1/projects/project_id/runs/run_id/browser/takeover", None, 1)
    verify_auth_headers(
        test_id, "DELETE", "/v1/projects/project_id/runs/run_id/browser/takeover", {"Authorization": r"Bearer .+"}, []
    )


def test_runs_watch_run() -> None:
    """Test watch_run endpoint with WireMock"""
    test_id = "runs.watch_run.0"
    client = get_client(test_id)
    client.runs.watch_run(
        project_id="project_id",
        run_id="run_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/runs/run_id/browser/watch", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/runs/run_id/browser/watch", {"Authorization": r"Bearer .+"}, []
    )


def test_runs_cancel() -> None:
    """Test cancel endpoint with WireMock"""
    test_id = "runs.cancel.0"
    client = get_client(test_id)
    client.runs.cancel(
        project_id="project_id",
        run_id="run_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/runs/run_id/cancel", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/runs/run_id/cancel", {"Authorization": r"Bearer .+"}, []
    )


def test_runs_events() -> None:
    """Test events endpoint with WireMock"""
    test_id = "runs.events.0"
    client = get_client(test_id)
    client.runs.events(
        project_id="project_id",
        run_id="run_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/runs/run_id/events", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/runs/run_id/events", {"Authorization": r"Bearer .+"}, []
    )


def test_runs_stream_events() -> None:
    """Test stream_events endpoint with WireMock"""
    test_id = "runs.stream_events.0"
    client = get_client(test_id)
    for _ in client.runs.stream_events(
        project_id="project_id",
        run_id="run_id",
    ):
        pass
    verify_request_count(test_id, "GET", "/v1/projects/project_id/runs/run_id/events/stream", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/runs/run_id/events/stream", {"Authorization": r"Bearer .+"}, []
    )


def test_runs_recover() -> None:
    """Test recover endpoint with WireMock"""
    test_id = "runs.recover.0"
    client = get_client(test_id)
    client.runs.recover(
        project_id="project_id",
        run_id="run_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/runs/run_id/recover", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/runs/run_id/recover", {"Authorization": r"Bearer .+"}, []
    )


def test_runs_resume() -> None:
    """Test resume endpoint with WireMock"""
    test_id = "runs.resume.0"
    client = get_client(test_id)
    client.runs.resume(
        project_id="project_id",
        run_id="run_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/runs/run_id/resume", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/runs/run_id/resume", {"Authorization": r"Bearer .+"}, []
    )


def test_runs_retry() -> None:
    """Test retry endpoint with WireMock"""
    test_id = "runs.retry.0"
    client = get_client(test_id)
    client.runs.retry(
        project_id="project_id",
        run_id="run_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/runs/run_id/retry", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/runs/run_id/retry", {"Authorization": r"Bearer .+"}, []
    )


def test_runs_decide_approval() -> None:
    """Test decide_approval endpoint with WireMock"""
    test_id = "runs.decide_approval.0"
    client = get_client(test_id)
    client.runs.decide_approval(
        project_id="project_id",
        run_id="run_id",
        tool_call_id="tool_call_id",
        decision="approved",
        idempotency_key="idempotency_key",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/runs/run_id/tool-calls/tool_call_id/approval", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/runs/run_id/tool-calls/tool_call_id/approval",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_runs_submit_tool_result() -> None:
    """Test submit_tool_result endpoint with WireMock"""
    test_id = "runs.submit_tool_result.0"
    client = get_client(test_id)
    client.runs.submit_tool_result(
        project_id="project_id",
        run_id="run_id",
        capability_name="capability_name",
        idempotency_key="idempotency_key",
        payload={"key": "value"},
        status="completed",
        tool_call_id="tool_call_id",
        tool_provider_id="tool_provider_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/runs/run_id/tool-results", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/runs/run_id/tool-results", {"Authorization": r"Bearer .+"}, []
    )


def test_runs_get_trajectory() -> None:
    """Test get_trajectory endpoint with WireMock"""
    test_id = "runs.get_trajectory.0"
    client = get_client(test_id)
    client.runs.get_trajectory(
        project_id="project_id",
        run_id="run_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/runs/run_id/trajectory", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/runs/run_id/trajectory", {"Authorization": r"Bearer .+"}, []
    )


def test_runs_get_transcript() -> None:
    """Test get_transcript endpoint with WireMock"""
    test_id = "runs.get_transcript.0"
    client = get_client(test_id)
    client.runs.get_transcript(
        project_id="project_id",
        run_id="run_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/runs/run_id/transcript", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/runs/run_id/transcript", {"Authorization": r"Bearer .+"}, []
    )


def test_runs_stream_transcript_events() -> None:
    """Test stream_transcript_events endpoint with WireMock"""
    test_id = "runs.stream_transcript_events.0"
    client = get_client(test_id)
    for _ in client.runs.stream_transcript_events(
        project_id="project_id",
        run_id="run_id",
    ):
        pass
    verify_request_count(test_id, "GET", "/v1/projects/project_id/runs/run_id/transcript/events/stream", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/runs/run_id/transcript/events/stream",
        {"Authorization": r"Bearer .+"},
        [],
    )
