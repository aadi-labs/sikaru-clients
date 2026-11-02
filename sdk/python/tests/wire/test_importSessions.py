from .conftest import get_client, verify_auth_headers, verify_request_count


def test_importSessions_list_import_sessions() -> None:
    """Test list_import_sessions endpoint with WireMock"""
    test_id = "import_sessions.list_import_sessions.0"
    client = get_client(test_id)
    client.import_sessions.list_import_sessions(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/import-sessions", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/import-sessions", {"Authorization": r"Bearer .+"}, [])


def test_importSessions_create_import_session() -> None:
    """Test create_import_session endpoint with WireMock"""
    test_id = "import_sessions.create_import_session.0"
    client = get_client(test_id)
    client.import_sessions.create_import_session(
        project_id="project_id",
        display_name="displayName",
        mode="workflow",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/import-sessions", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/import-sessions", {"Authorization": r"Bearer .+"}, [])


def test_importSessions_get_import_session() -> None:
    """Test get_import_session endpoint with WireMock"""
    test_id = "import_sessions.get_import_session.0"
    client = get_client(test_id)
    client.import_sessions.get_import_session(
        project_id="project_id",
        import_session_id="import_session_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/import-sessions/import_session_id", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/import-sessions/import_session_id", {"Authorization": r"Bearer .+"}, []
    )


def test_importSessions_create_compatibility_profile() -> None:
    """Test create_compatibility_profile endpoint with WireMock"""
    test_id = "import_sessions.create_compatibility_profile.0"
    client = get_client(test_id)
    client.import_sessions.create_compatibility_profile(
        project_id="project_id",
        import_session_id="import_session_id",
        display_name="displayName",
        harness_id="harnessId",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/import-sessions/import_session_id/compatibility-profile", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/import-sessions/import_session_id/compatibility-profile",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_importSessions_list_import_session_diffs() -> None:
    """Test list_import_session_diffs endpoint with WireMock"""
    test_id = "import_sessions.list_import_session_diffs.0"
    client = get_client(test_id)
    client.import_sessions.list_import_session_diffs(
        project_id="project_id",
        import_session_id="import_session_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/import-sessions/import_session_id/diffs", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/import-sessions/import_session_id/diffs",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_importSessions_get_parity_report() -> None:
    """Test get_parity_report endpoint with WireMock"""
    test_id = "import_sessions.get_parity_report.0"
    client = get_client(test_id)
    client.import_sessions.get_parity_report(
        project_id="project_id",
        import_session_id="import_session_id",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/import-sessions/import_session_id/parity-report", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/import-sessions/import_session_id/parity-report",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_importSessions_promote_import_session() -> None:
    """Test promote_import_session endpoint with WireMock"""
    test_id = "import_sessions.promote_import_session.0"
    client = get_client(test_id)
    client.import_sessions.promote_import_session(
        project_id="project_id",
        import_session_id="import_session_id",
        active_harness_version_id="activeHarnessVersionId",
        agent_slug="agentSlug",
        compatibility_profile_id="compatibilityProfileId",
        display_name="displayName",
        harness_id="harnessId",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/import-sessions/import_session_id/promote", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/import-sessions/import_session_id/promote",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_importSessions_create_replay_run() -> None:
    """Test create_replay_run endpoint with WireMock"""
    test_id = "import_sessions.create_replay_run.0"
    client = get_client(test_id)
    client.import_sessions.create_replay_run(
        project_id="project_id",
        import_session_id="import_session_id",
        compatibility_profile_id="compatibilityProfileId",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/import-sessions/import_session_id/replay-runs", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/import-sessions/import_session_id/replay-runs",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_importSessions_list_source_artifacts() -> None:
    """Test list_source_artifacts endpoint with WireMock"""
    test_id = "import_sessions.list_source_artifacts.0"
    client = get_client(test_id)
    client.import_sessions.list_source_artifacts(
        project_id="project_id",
        import_session_id="import_session_id",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/import-sessions/import_session_id/source-artifacts", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/import-sessions/import_session_id/source-artifacts",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_importSessions_create_source_artifact() -> None:
    """Test create_source_artifact endpoint with WireMock"""
    test_id = "import_sessions.create_source_artifact.0"
    client = get_client(test_id)
    client.import_sessions.create_source_artifact(
        project_id="project_id",
        import_session_id="import_session_id",
        content_digest="contentDigest",
        export_policy="customer_source_exportable",
        source_kind="workflow_json",
        storage_ref="storageRef",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/import-sessions/import_session_id/source-artifacts", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/import-sessions/import_session_id/source-artifacts",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_importSessions_create_staging_run() -> None:
    """Test create_staging_run endpoint with WireMock"""
    test_id = "import_sessions.create_staging_run.0"
    client = get_client(test_id)
    client.import_sessions.create_staging_run(
        project_id="project_id",
        import_session_id="import_session_id",
        compatibility_profile_id="compatibilityProfileId",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/import-sessions/import_session_id/staging-runs", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/import-sessions/import_session_id/staging-runs",
        {"Authorization": r"Bearer .+"},
        [],
    )
