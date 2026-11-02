from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_api import ReleaseActionRequest


def test_changesets_list_changesets() -> None:
    """Test list_changesets endpoint with WireMock"""
    test_id = "changesets.list_changesets.0"
    client = get_client(test_id)
    client.changesets.list_changesets(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/changesets", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/changesets", {"Authorization": r"Bearer .+"}, [])


def test_changesets_create_changeset() -> None:
    """Test create_changeset endpoint with WireMock"""
    test_id = "changesets.create_changeset.0"
    client = get_client(test_id)
    client.changesets.create_changeset(
        project_id="project_id",
        public_summary="publicSummary",
        public_title="publicTitle",
        scope_ref="scopeRef",
        scope_type="global",
        source_type="import",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/changesets", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/changesets", {"Authorization": r"Bearer .+"}, [])


def test_changesets_get_changeset() -> None:
    """Test get_changeset endpoint with WireMock"""
    test_id = "changesets.get_changeset.0"
    client = get_client(test_id)
    client.changesets.get_changeset(
        project_id="project_id",
        changeset_id="changeset_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/changesets/changeset_id", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/changesets/changeset_id", {"Authorization": r"Bearer .+"}, []
    )


def test_changesets_approve_changeset() -> None:
    """Test approve_changeset endpoint with WireMock"""
    test_id = "changesets.approve_changeset.0"
    client = get_client(test_id)
    client.changesets.approve_changeset(
        project_id="project_id",
        changeset_id="changeset_id",
        request=ReleaseActionRequest(),
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/changesets/changeset_id/approve", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/changesets/changeset_id/approve", {"Authorization": r"Bearer .+"}, []
    )


def test_changesets_list_changeset_diffs() -> None:
    """Test list_changeset_diffs endpoint with WireMock"""
    test_id = "changesets.list_changeset_diffs.0"
    client = get_client(test_id)
    client.changesets.list_changeset_diffs(
        project_id="project_id",
        changeset_id="changeset_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/changesets/changeset_id/diffs", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/changesets/changeset_id/diffs", {"Authorization": r"Bearer .+"}, []
    )


def test_changesets_list_changeset_evidence() -> None:
    """Test list_changeset_evidence endpoint with WireMock"""
    test_id = "changesets.list_changeset_evidence.0"
    client = get_client(test_id)
    client.changesets.list_changeset_evidence(
        project_id="project_id",
        changeset_id="changeset_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/changesets/changeset_id/evidence", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/changesets/changeset_id/evidence", {"Authorization": r"Bearer .+"}, []
    )


def test_changesets_promote_changeset() -> None:
    """Test promote_changeset endpoint with WireMock"""
    test_id = "changesets.promote_changeset.0"
    client = get_client(test_id)
    client.changesets.promote_changeset(
        project_id="project_id",
        changeset_id="changeset_id",
        request=ReleaseActionRequest(),
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/changesets/changeset_id/promote", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/changesets/changeset_id/promote", {"Authorization": r"Bearer .+"}, []
    )


def test_changesets_reject_changeset() -> None:
    """Test reject_changeset endpoint with WireMock"""
    test_id = "changesets.reject_changeset.0"
    client = get_client(test_id)
    client.changesets.reject_changeset(
        project_id="project_id",
        changeset_id="changeset_id",
        request=ReleaseActionRequest(),
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/changesets/changeset_id/reject", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/changesets/changeset_id/reject", {"Authorization": r"Bearer .+"}, []
    )


def test_changesets_rollback_changeset() -> None:
    """Test rollback_changeset endpoint with WireMock"""
    test_id = "changesets.rollback_changeset.0"
    client = get_client(test_id)
    client.changesets.rollback_changeset(
        project_id="project_id",
        changeset_id="changeset_id",
        request=ReleaseActionRequest(),
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/changesets/changeset_id/rollback", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/changesets/changeset_id/rollback", {"Authorization": r"Bearer .+"}, []
    )


def test_changesets_stage_changeset() -> None:
    """Test stage_changeset endpoint with WireMock"""
    test_id = "changesets.stage_changeset.0"
    client = get_client(test_id)
    client.changesets.stage_changeset(
        project_id="project_id",
        changeset_id="changeset_id",
        request=ReleaseActionRequest(),
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/changesets/changeset_id/stage", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/changesets/changeset_id/stage", {"Authorization": r"Bearer .+"}, []
    )
