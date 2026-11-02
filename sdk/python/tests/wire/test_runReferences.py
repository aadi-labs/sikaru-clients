from .conftest import get_client, verify_auth_headers, verify_request_count


def test_runReferences_resolve_run_reference() -> None:
    """Test resolve_run_reference endpoint with WireMock"""
    test_id = "run_references.resolve_run_reference.0"
    client = get_client(test_id)
    client.run_references.resolve_run_reference(
        project_id="project_id",
        reference="reference",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/run-references/reference", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/run-references/reference", {"Authorization": r"Bearer .+"}, []
    )
