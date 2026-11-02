from .conftest import get_client, verify_auth_headers, verify_request_count


def test_retentionPolicies_create_retention_policy_update() -> None:
    """Test create_retention_policy_update endpoint with WireMock"""
    test_id = "retention_policies.create_retention_policy_update.0"
    client = get_client(test_id)
    client.retention_policies.create_retention_policy_update(
        project_id="project_id",
        trace_ids=["traceIds"],
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/retention-policies", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/retention-policies", {"Authorization": r"Bearer .+"}, []
    )
