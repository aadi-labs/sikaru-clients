from .conftest import get_client, verify_auth_headers, verify_request_count


def test_runSchedules_list_schedules() -> None:
    """Test list_schedules endpoint with WireMock"""
    test_id = "run_schedules.list_schedules.0"
    client = get_client(test_id)
    client.run_schedules.list_schedules(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/run-schedules", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/run-schedules", {"Authorization": r"Bearer .+"}, [])


def test_runSchedules_create_schedule() -> None:
    """Test create_schedule endpoint with WireMock"""
    test_id = "run_schedules.create_schedule.0"
    client = get_client(test_id)
    client.run_schedules.create_schedule(
        project_id="project_id",
        input={"key": "value"},
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/run-schedules", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/run-schedules", {"Authorization": r"Bearer .+"}, [])


def test_runSchedules_delete_schedule() -> None:
    """Test delete_schedule endpoint with WireMock"""
    test_id = "run_schedules.delete_schedule.0"
    client = get_client(test_id)
    client.run_schedules.delete_schedule(
        project_id="project_id",
        schedule_id="schedule_id",
    )
    verify_request_count(test_id, "DELETE", "/v1/projects/project_id/run-schedules/schedule_id", None, 1)
    verify_auth_headers(
        test_id, "DELETE", "/v1/projects/project_id/run-schedules/schedule_id", {"Authorization": r"Bearer .+"}, []
    )


def test_runSchedules_pause_schedule() -> None:
    """Test pause_schedule endpoint with WireMock"""
    test_id = "run_schedules.pause_schedule.0"
    client = get_client(test_id)
    client.run_schedules.pause_schedule(
        project_id="project_id",
        schedule_id="schedule_id",
        paused=True,
    )
    verify_request_count(test_id, "PATCH", "/v1/projects/project_id/run-schedules/schedule_id", None, 1)
    verify_auth_headers(
        test_id, "PATCH", "/v1/projects/project_id/run-schedules/schedule_id", {"Authorization": r"Bearer .+"}, []
    )


def test_runSchedules_schedule_notices() -> None:
    """Test schedule_notices endpoint with WireMock"""
    test_id = "run_schedules.schedule_notices.0"
    client = get_client(test_id)
    client.run_schedules.schedule_notices(
        project_id="project_id",
        schedule_id="schedule_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/run-schedules/schedule_id/notices", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/run-schedules/schedule_id/notices", {"Authorization": r"Bearer .+"}, []
    )


def test_runSchedules_schedule_history() -> None:
    """Test schedule_history endpoint with WireMock"""
    test_id = "run_schedules.schedule_history.0"
    client = get_client(test_id)
    client.run_schedules.schedule_history(
        project_id="project_id",
        schedule_id="schedule_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/run-schedules/schedule_id/occurrences", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/run-schedules/schedule_id/occurrences",
        {"Authorization": r"Bearer .+"},
        [],
    )
