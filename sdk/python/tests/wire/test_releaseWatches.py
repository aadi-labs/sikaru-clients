from .conftest import get_client, verify_auth_headers, verify_request_count


def test_releaseWatches_create_release_watch() -> None:
    """Test create_release_watch endpoint with WireMock"""
    test_id = "release_watches.create_release_watch.0"
    client = get_client(test_id)
    client.release_watches.create_release_watch(
        project_id="project_id",
        trace_ids=["traceIds"],
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/release-watches", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/release-watches", {"Authorization": r"Bearer .+"}, [])
