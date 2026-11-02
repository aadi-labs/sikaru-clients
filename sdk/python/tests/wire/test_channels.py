from .conftest import get_client, verify_auth_headers, verify_request_count


def test_channels_availability() -> None:
    """Test availability endpoint with WireMock"""
    test_id = "channels.availability.0"
    client = get_client(test_id)
    client.channels.availability(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/channels/availability", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/channels/availability", {"Authorization": r"Bearer .+"}, []
    )


def test_channels_bindings() -> None:
    """Test bindings endpoint with WireMock"""
    test_id = "channels.bindings.0"
    client = get_client(test_id)
    client.channels.bindings(
        project_id="project_id",
        agent_id="agent_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/channels/bindings", {"agent_id": "agent_id"}, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/channels/bindings", {"Authorization": r"Bearer .+"}, []
    )


def test_channels_create_binding() -> None:
    """Test create_binding endpoint with WireMock"""
    test_id = "channels.create_binding.0"
    client = get_client(test_id)
    client.channels.create_binding(
        project_id="project_id",
        agent_id="agent_id",
        transport="http",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/channels/bindings", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/channels/bindings", {"Authorization": r"Bearer .+"}, []
    )


def test_channels_delete_http_binding() -> None:
    """Test delete_http_binding endpoint with WireMock"""
    test_id = "channels.delete_http_binding.0"
    client = get_client(test_id)
    client.channels.delete_http_binding(
        project_id="project_id",
        binding_id="binding_id",
    )
    verify_request_count(test_id, "DELETE", "/v1/projects/project_id/channels/bindings/binding_id", None, 1)
    verify_auth_headers(
        test_id, "DELETE", "/v1/projects/project_id/channels/bindings/binding_id", {"Authorization": r"Bearer .+"}, []
    )


def test_channels_status() -> None:
    """Test status endpoint with WireMock"""
    test_id = "channels.status.0"
    client = get_client(test_id)
    client.channels.status(
        project_id="project_id",
        binding_id="binding_id",
        status="active",
    )
    verify_request_count(test_id, "PATCH", "/v1/projects/project_id/channels/bindings/binding_id", None, 1)
    verify_auth_headers(
        test_id, "PATCH", "/v1/projects/project_id/channels/bindings/binding_id", {"Authorization": r"Bearer .+"}, []
    )


def test_channels_configure_http_binding() -> None:
    """Test configure_http_binding endpoint with WireMock"""
    test_id = "channels.configure_http_binding.0"
    client = get_client(test_id)
    client.channels.configure_http_binding(
        project_id="project_id",
        binding_id="binding_id",
    )
    verify_request_count(
        test_id, "PATCH", "/v1/projects/project_id/channels/bindings/binding_id/configuration", None, 1
    )
    verify_auth_headers(
        test_id,
        "PATCH",
        "/v1/projects/project_id/channels/bindings/binding_id/configuration",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_channels_issue_http_credential() -> None:
    """Test issue_http_credential endpoint with WireMock"""
    test_id = "channels.issue_http_credential.0"
    client = get_client(test_id)
    client.channels.issue_http_credential(
        project_id="project_id",
        binding_id="binding_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/channels/bindings/binding_id/credential", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/channels/bindings/binding_id/credential",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_channels_configure_personal_access() -> None:
    """Test configure_personal_access endpoint with WireMock"""
    test_id = "channels.configure_personal_access.0"
    client = get_client(test_id)
    client.channels.configure_personal_access(
        project_id="project_id",
        binding_id="binding_id",
        enabled=True,
    )
    verify_request_count(
        test_id, "PUT", "/v1/projects/project_id/channels/bindings/binding_id/personal-access", None, 1
    )
    verify_auth_headers(
        test_id,
        "PUT",
        "/v1/projects/project_id/channels/bindings/binding_id/personal-access",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_channels_recent_receipts() -> None:
    """Test recent_receipts endpoint with WireMock"""
    test_id = "channels.recent_receipts.0"
    client = get_client(test_id)
    client.channels.recent_receipts(
        project_id="project_id",
        binding_id="binding_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/channels/bindings/binding_id/receipts", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/channels/bindings/binding_id/receipts",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_channels_get_member_slack_link() -> None:
    """Test get_member_slack_link endpoint with WireMock"""
    test_id = "channels.get_member_slack_link.0"
    client = get_client(test_id)
    client.channels.get_member_slack_link(
        project_id="project_id",
        binding_id="binding_id",
        verification_id="verification_id",
    )
    verify_request_count(
        test_id,
        "GET",
        "/v1/projects/project_id/channels/bindings/binding_id/slack-link-verifications/verification_id",
        None,
        1,
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/channels/bindings/binding_id/slack-link-verifications/verification_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_channels_start_member_slack_link() -> None:
    """Test start_member_slack_link endpoint with WireMock"""
    test_id = "channels.start_member_slack_link.0"
    client = get_client(test_id)
    client.channels.start_member_slack_link(
        project_id="project_id",
        binding_id="binding_id",
        installation_id="installation_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/channels/bindings/binding_id/slack-links", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/channels/bindings/binding_id/slack-links",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_channels_unlink_member_slack_identity() -> None:
    """Test unlink_member_slack_identity endpoint with WireMock"""
    test_id = "channels.unlink_member_slack_identity.0"
    client = get_client(test_id)
    client.channels.unlink_member_slack_identity(
        project_id="project_id",
        binding_id="binding_id",
        installation_id="installation_id",
    )
    verify_request_count(
        test_id, "DELETE", "/v1/projects/project_id/channels/bindings/binding_id/slack-links/installation_id", None, 1
    )
    verify_auth_headers(
        test_id,
        "DELETE",
        "/v1/projects/project_id/channels/bindings/binding_id/slack-links/installation_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_channels_save_creation_resume() -> None:
    """Test save_creation_resume endpoint with WireMock"""
    test_id = "channels.save_creation_resume.0"
    client = get_client(test_id)
    client.channels.save_creation_resume(
        project_id="project_id",
        payload={"key": "value"},
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/channels/creation-resumes", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/channels/creation-resumes", {"Authorization": r"Bearer .+"}, []
    )


def test_channels_get_creation_resume() -> None:
    """Test get_creation_resume endpoint with WireMock"""
    test_id = "channels.get_creation_resume.0"
    client = get_client(test_id)
    client.channels.get_creation_resume(
        project_id="project_id",
        resume_id="resume_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/channels/creation-resumes/resume_id", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/channels/creation-resumes/resume_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_channels_list_identity_apps() -> None:
    """Test list_identity_apps endpoint with WireMock"""
    test_id = "channels.list_identity_apps.0"
    client = get_client(test_id)
    client.channels.list_identity_apps(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/channels/identity-apps", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/channels/identity-apps", {"Authorization": r"Bearer .+"}, []
    )


def test_channels_create_identity_app() -> None:
    """Test create_identity_app endpoint with WireMock"""
    test_id = "channels.create_identity_app.0"
    client = get_client(test_id)
    client.channels.create_identity_app(
        project_id="project_id",
        audience="audience",
        issuer="issuer",
        public_jwk={"key": "value"},
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/channels/identity-apps", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/channels/identity-apps", {"Authorization": r"Bearer .+"}, []
    )


def test_channels_revoke_identity_app() -> None:
    """Test revoke_identity_app endpoint with WireMock"""
    test_id = "channels.revoke_identity_app.0"
    client = get_client(test_id)
    client.channels.revoke_identity_app(
        project_id="project_id",
        app_id="app_id",
    )
    verify_request_count(test_id, "DELETE", "/v1/projects/project_id/channels/identity-apps/app_id", None, 1)
    verify_auth_headers(
        test_id, "DELETE", "/v1/projects/project_id/channels/identity-apps/app_id", {"Authorization": r"Bearer .+"}, []
    )


def test_channels_update_identity_app_urls() -> None:
    """Test update_identity_app_urls endpoint with WireMock"""
    test_id = "channels.update_identity_app_urls.0"
    client = get_client(test_id)
    client.channels.update_identity_app_urls(
        project_id="project_id",
        app_id="app_id",
    )
    verify_request_count(test_id, "PATCH", "/v1/projects/project_id/channels/identity-apps/app_id", None, 1)
    verify_auth_headers(
        test_id, "PATCH", "/v1/projects/project_id/channels/identity-apps/app_id", {"Authorization": r"Bearer .+"}, []
    )


def test_channels_revoke_subject() -> None:
    """Test revoke_subject endpoint with WireMock"""
    test_id = "channels.revoke_subject.0"
    client = get_client(test_id)
    client.channels.revoke_subject(
        project_id="project_id",
        app_id="app_id",
        subject="subject",
        tenant_id="tenant_id",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/channels/identity-apps/app_id/subjects/revoke", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/channels/identity-apps/app_id/subjects/revoke",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_channels_create_personal_slack_binding() -> None:
    """Test create_personal_slack_binding endpoint with WireMock"""
    test_id = "channels.create_personal_slack_binding.0"
    client = get_client(test_id)
    client.channels.create_personal_slack_binding(
        project_id="project_id",
        agent_id="agent_id",
        identity_app_id="identity_app_id",
        installation_id="installation_id",
        verification_id="verification_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/channels/personal-slack-bindings", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/channels/personal-slack-bindings", {"Authorization": r"Bearer .+"}, []
    )


def test_channels_delivery() -> None:
    """Test delivery endpoint with WireMock"""
    test_id = "channels.delivery.0"
    client = get_client(test_id)
    client.channels.delivery(
        project_id="project_id",
        run_id="run_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/channels/runs/run_id/delivery", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/channels/runs/run_id/delivery", {"Authorization": r"Bearer .+"}, []
    )


def test_channels_resend_delivery() -> None:
    """Test resend_delivery endpoint with WireMock"""
    test_id = "channels.resend_delivery.0"
    client = get_client(test_id)
    client.channels.resend_delivery(
        project_id="project_id",
        run_id="run_id",
        acknowledge_possible_duplicate=True,
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/channels/runs/run_id/delivery/resend", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/channels/runs/run_id/delivery/resend",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_channels_dm_status() -> None:
    """Test dm_status endpoint with WireMock"""
    test_id = "channels.dm_status.0"
    client = get_client(test_id)
    client.channels.dm_status(
        project_id="project_id",
        verification_id="verification_id",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/channels/slack/dm-verifications/verification_id", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/channels/slack/dm-verifications/verification_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_channels_installations() -> None:
    """Test installations endpoint with WireMock"""
    test_id = "channels.installations.0"
    client = get_client(test_id)
    client.channels.installations(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/channels/slack/installations", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/channels/slack/installations", {"Authorization": r"Bearer .+"}, []
    )


def test_channels_disconnect() -> None:
    """Test disconnect endpoint with WireMock"""
    test_id = "channels.disconnect.0"
    client = get_client(test_id)
    client.channels.disconnect(
        project_id="project_id",
        installation_id="installation_id",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/channels/slack/installations/installation_id/disconnect", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/channels/slack/installations/installation_id/disconnect",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_channels_start_dm() -> None:
    """Test start_dm endpoint with WireMock"""
    test_id = "channels.start_dm.0"
    client = get_client(test_id)
    client.channels.start_dm(
        project_id="project_id",
        installation_id="installation_id",
    )
    verify_request_count(
        test_id,
        "POST",
        "/v1/projects/project_id/channels/slack/installations/installation_id/dm-verifications",
        None,
        1,
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/channels/slack/installations/installation_id/dm-verifications",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_channels_rooms() -> None:
    """Test rooms endpoint with WireMock"""
    test_id = "channels.rooms.0"
    client = get_client(test_id)
    client.channels.rooms(
        project_id="project_id",
        installation_id="installation_id",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/channels/slack/installations/installation_id/rooms", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/channels/slack/installations/installation_id/rooms",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_channels_complete() -> None:
    """Test complete endpoint with WireMock"""
    test_id = "channels.complete.0"
    client = get_client(test_id)
    client.channels.complete(
        project_id="project_id",
        state="state",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/channels/slack/oauth/complete", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/channels/slack/oauth/complete", {"Authorization": r"Bearer .+"}, []
    )


def test_channels_start() -> None:
    """Test start endpoint with WireMock"""
    test_id = "channels.start.0"
    client = get_client(test_id)
    client.channels.start(
        project_id="project_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/channels/slack/oauth/start", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/channels/slack/oauth/start", {"Authorization": r"Bearer .+"}, []
    )
