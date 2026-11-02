from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_api import WorkspaceProvenance


def test_computeAttachments_get() -> None:
    """Test get endpoint with WireMock"""
    test_id = "compute_attachments.get.0"
    client = get_client(test_id)
    client.compute_attachments.get(
        project_id="project_id",
        attachment_id="attachment_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/compute-attachments/attachment_id", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/compute-attachments/attachment_id", {"Authorization": r"Bearer .+"}, []
    )


def test_computeAttachments_abandon() -> None:
    """Test abandon endpoint with WireMock"""
    test_id = "compute_attachments.abandon.0"
    client = get_client(test_id)
    client.compute_attachments.abandon(
        project_id="project_id",
        attachment_id="attachment_id",
        evidence="evidence",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/compute-attachments/attachment_id/abandon", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/compute-attachments/attachment_id/abandon",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_computeAttachments_cancel() -> None:
    """Test cancel endpoint with WireMock"""
    test_id = "compute_attachments.cancel.0"
    client = get_client(test_id)
    client.compute_attachments.cancel(
        project_id="project_id",
        attachment_id="attachment_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/compute-attachments/attachment_id/cancel", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/compute-attachments/attachment_id/cancel",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_computeAttachments_claim() -> None:
    """Test claim endpoint with WireMock"""
    test_id = "compute_attachments.claim.0"
    client = get_client(test_id)
    client.compute_attachments.claim(
        project_id="project_id",
        attachment_id="attachment_id",
        idempotency_key="idempotency_key",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/compute-attachments/attachment_id/claim", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/compute-attachments/attachment_id/claim",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_computeAttachments_cleanup() -> None:
    """Test cleanup endpoint with WireMock"""
    test_id = "compute_attachments.cleanup.0"
    client = get_client(test_id)
    client.compute_attachments.cleanup(
        project_id="project_id",
        attachment_id="attachment_id",
        children_terminated=True,
        evidence="evidence",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/compute-attachments/attachment_id/cleanup", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/compute-attachments/attachment_id/cleanup",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_computeAttachments_connect() -> None:
    """Test connect endpoint with WireMock"""
    test_id = "compute_attachments.connect.0"
    client = get_client(test_id)
    client.compute_attachments.connect(
        project_id="project_id",
        attachment_id="attachment_id",
        capabilities=["compute.execute"],
        executor_instance_id="executor_instance_id",
        journal_id="journal_id",
        protocol_version="sikaru-compute-v1",
        workspace_provenance=WorkspaceProvenance(
            identity="identity",
            kind="existing_directory",
        ),
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/compute-attachments/attachment_id/connect", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/compute-attachments/attachment_id/connect",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_computeAttachments_issue_credential() -> None:
    """Test issue_credential endpoint with WireMock"""
    test_id = "compute_attachments.issue_credential.0"
    client = get_client(test_id)
    client.compute_attachments.issue_credential(
        project_id="project_id",
        attachment_id="attachment_id",
        owner_epoch=1,
        owner_id="owner_id",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/compute-attachments/attachment_id/credentials", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/compute-attachments/attachment_id/credentials",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_computeAttachments_heartbeat() -> None:
    """Test heartbeat endpoint with WireMock"""
    test_id = "compute_attachments.heartbeat.0"
    client = get_client(test_id)
    client.compute_attachments.heartbeat(
        project_id="project_id",
        attachment_id="attachment_id",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/compute-attachments/attachment_id/heartbeat", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/compute-attachments/attachment_id/heartbeat",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_computeAttachments_ready() -> None:
    """Test ready endpoint with WireMock"""
    test_id = "compute_attachments.ready.0"
    client = get_client(test_id)
    client.compute_attachments.ready(
        project_id="project_id",
        attachment_id="attachment_id",
        capabilities=["compute.execute"],
        executor_instance_id="executor_instance_id",
        journal_id="journal_id",
        protocol_version="sikaru-compute-v1",
        workspace_provenance=WorkspaceProvenance(
            identity="identity",
            kind="existing_directory",
        ),
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/compute-attachments/attachment_id/ready", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/compute-attachments/attachment_id/ready",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_computeAttachments_reconcile() -> None:
    """Test reconcile endpoint with WireMock"""
    test_id = "compute_attachments.reconcile.0"
    client = get_client(test_id)
    client.compute_attachments.reconcile(
        project_id="project_id",
        attachment_id="attachment_id",
        executor_instance_id="executor_instance_id",
        journal_id="journal_id",
        workspace_provenance=WorkspaceProvenance(
            identity="identity",
            kind="existing_directory",
        ),
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/compute-attachments/attachment_id/reconcile", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/compute-attachments/attachment_id/reconcile",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_computeAttachments_status() -> None:
    """Test status endpoint with WireMock"""
    test_id = "compute_attachments.status.0"
    client = get_client(test_id)
    client.compute_attachments.status(
        project_id="project_id",
        attachment_id="attachment_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/compute-attachments/attachment_id/status", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/compute-attachments/attachment_id/status",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_computeAttachments_stop() -> None:
    """Test stop endpoint with WireMock"""
    test_id = "compute_attachments.stop.0"
    client = get_client(test_id)
    client.compute_attachments.stop(
        project_id="project_id",
        attachment_id="attachment_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/compute-attachments/attachment_id/stop", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/compute-attachments/attachment_id/stop",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_computeAttachments_teardown() -> None:
    """Test teardown endpoint with WireMock"""
    test_id = "compute_attachments.teardown.0"
    client = get_client(test_id)
    client.compute_attachments.teardown(
        project_id="project_id",
        attachment_id="attachment_id",
        children_terminated=True,
        evidence="evidence",
        owner_epoch=1,
        owner_id="owner_id",
        workspace_generation="workspace_generation",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/compute-attachments/attachment_id/teardown", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/compute-attachments/attachment_id/teardown",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_computeAttachments_create() -> None:
    """Test create endpoint with WireMock"""
    test_id = "compute_attachments.create.0"
    client = get_client(test_id)
    client.compute_attachments.create(
        project_id="project_id",
        session_id="session_id",
        environment_id="environment_id",
        idempotency_key="idempotency_key",
        workspace_provenance=WorkspaceProvenance(
            identity="identity",
            kind="existing_directory",
        ),
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/execution-sessions/session_id/compute-attachments", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/execution-sessions/session_id/compute-attachments",
        {"Authorization": r"Bearer .+"},
        [],
    )
