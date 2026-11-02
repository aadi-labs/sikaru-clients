from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_api import TraceMetadata


def test_traceStreams_stream_openinference_spans() -> None:
    """Test stream_openinference_spans endpoint with WireMock"""
    test_id = "trace_streams.stream_openinference_spans.0"
    client = get_client(test_id)
    client.trace_streams.stream_openinference_spans(
        format="openinference.v1",
        metadata=TraceMetadata(
            account_id="account_id",
            project_id="project_id",
            source="source",
        ),
    )
    verify_request_count(test_id, "POST", "/v1/trace-streams", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/trace-streams", {"Authorization": r"Bearer .+"}, [])
