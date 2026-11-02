from .conftest import get_client, verify_auth_headers, verify_request_count


def test_agentDocuments_draft() -> None:
    """Test draft endpoint with WireMock"""
    test_id = "agent_documents.draft.0"
    client = get_client(test_id)
    client.agent_documents.draft(
        project_id="project_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/agent-documents/draft", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/agent-documents/draft", {"Authorization": r"Bearer .+"}, []
    )


def test_agentDocuments_import_files() -> None:
    """Test import_files endpoint with WireMock"""
    test_id = "agent_documents.import_files.0"
    client = get_client(test_id)
    client.agent_documents.import_files(
        project_id="project_id",
        files={"key": "value"},
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/agent-documents/import", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/agent-documents/import", {"Authorization": r"Bearer .+"}, []
    )


def test_agentDocuments_list_resources() -> None:
    """Test list_resources endpoint with WireMock"""
    test_id = "agent_documents.list_resources.0"
    client = get_client(test_id)
    client.agent_documents.list_resources(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/agent-documents/resources", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/agent-documents/resources", {"Authorization": r"Bearer .+"}, []
    )


def test_agentDocuments_edit_setting() -> None:
    """Test edit_setting endpoint with WireMock"""
    test_id = "agent_documents.edit_setting.0"
    client = get_client(test_id)
    client.agent_documents.edit_setting(
        project_id="project_id",
        document="document",
        path=["path"],
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/agent-documents/settings", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/agent-documents/settings", {"Authorization": r"Bearer .+"}, []
    )


def test_agentDocuments_list_templates() -> None:
    """Test list_templates endpoint with WireMock"""
    test_id = "agent_documents.list_templates.0"
    client = get_client(test_id)
    client.agent_documents.list_templates(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/agent-documents/templates", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/agent-documents/templates", {"Authorization": r"Bearer .+"}, []
    )


def test_agentDocuments_validate_text() -> None:
    """Test validate_text endpoint with WireMock"""
    test_id = "agent_documents.validate_text.0"
    client = get_client(test_id)
    client.agent_documents.validate_text(
        project_id="project_id",
        document="document",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/agent-documents/validate", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/agent-documents/validate", {"Authorization": r"Bearer .+"}, []
    )


def test_agentDocuments_get() -> None:
    """Test get endpoint with WireMock"""
    test_id = "agent_documents.get.0"
    client = get_client(test_id)
    client.agent_documents.get(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/document", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/document",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_agentDocuments_save() -> None:
    """Test save endpoint with WireMock"""
    test_id = "agent_documents.save.0"
    client = get_client(test_id)
    client.agent_documents.save(
        project_id="project_id",
        agent_slug="agent_slug",
        document="document",
        expected_revision=1,
    )
    verify_request_count(test_id, "PUT", "/v1/projects/project_id/managed-agents/agent_slug/document", None, 1)
    verify_auth_headers(
        test_id,
        "PUT",
        "/v1/projects/project_id/managed-agents/agent_slug/document",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_agentDocuments_compare() -> None:
    """Test compare endpoint with WireMock"""
    test_id = "agent_documents.compare.0"
    client = get_client(test_id)
    client.agent_documents.compare(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/document/compare", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/document/compare",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_agentDocuments_discard() -> None:
    """Test discard endpoint with WireMock"""
    test_id = "agent_documents.discard.0"
    client = get_client(test_id)
    client.agent_documents.discard(
        project_id="project_id",
        agent_slug="agent_slug",
        expected_revision=1,
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/managed-agents/agent_slug/document/discard", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/document/discard",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_agentDocuments_publish() -> None:
    """Test publish endpoint with WireMock"""
    test_id = "agent_documents.publish.0"
    client = get_client(test_id)
    client.agent_documents.publish(
        project_id="project_id",
        agent_slug="agent_slug",
        revision=1,
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/managed-agents/agent_slug/document/publish", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/document/publish",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_agentDocuments_revert() -> None:
    """Test revert endpoint with WireMock"""
    test_id = "agent_documents.revert.0"
    client = get_client(test_id)
    client.agent_documents.revert(
        project_id="project_id",
        agent_slug="agent_slug",
        harness_version_id="harnessVersionId",
        revision=1,
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/managed-agents/agent_slug/document/revert", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/document/revert",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_agentDocuments_review() -> None:
    """Test review endpoint with WireMock"""
    test_id = "agent_documents.review.0"
    client = get_client(test_id)
    client.agent_documents.review(
        project_id="project_id",
        agent_slug="agent_slug",
        revision=1,
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/managed-agents/agent_slug/document/review", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/document/review",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_agentDocuments_snippets() -> None:
    """Test snippets endpoint with WireMock"""
    test_id = "agent_documents.snippets.0"
    client = get_client(test_id)
    client.agent_documents.snippets(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/document/snippets", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/document/snippets",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_agentDocuments_list_suggestions() -> None:
    """Test list_suggestions endpoint with WireMock"""
    test_id = "agent_documents.list_suggestions.0"
    client = get_client(test_id)
    client.agent_documents.list_suggestions(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/document/suggestions", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/document/suggestions",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_agentDocuments_adopt_suggestion() -> None:
    """Test adopt_suggestion endpoint with WireMock"""
    test_id = "agent_documents.adopt_suggestion.0"
    client = get_client(test_id)
    client.agent_documents.adopt_suggestion(
        project_id="project_id",
        agent_slug="agent_slug",
        suggestion_id="suggestion_id",
        expected_revision=1,
    )
    verify_request_count(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/document/suggestions/suggestion_id/adopt",
        None,
        1,
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/document/suggestions/suggestion_id/adopt",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_agentDocuments_dismiss_suggestion() -> None:
    """Test dismiss_suggestion endpoint with WireMock"""
    test_id = "agent_documents.dismiss_suggestion.0"
    client = get_client(test_id)
    client.agent_documents.dismiss_suggestion(
        project_id="project_id",
        agent_slug="agent_slug",
        suggestion_id="suggestion_id",
    )
    verify_request_count(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/document/suggestions/suggestion_id/dismiss",
        None,
        1,
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/document/suggestions/suggestion_id/dismiss",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_agentDocuments_validate() -> None:
    """Test validate endpoint with WireMock"""
    test_id = "agent_documents.validate.0"
    client = get_client(test_id)
    client.agent_documents.validate(
        project_id="project_id",
        agent_slug="agent_slug",
        document="document",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/managed-agents/agent_slug/document/validate", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/document/validate",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_agentDocuments_list_versions() -> None:
    """Test list_versions endpoint with WireMock"""
    test_id = "agent_documents.list_versions.0"
    client = get_client(test_id)
    client.agent_documents.list_versions(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/document/versions", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/document/versions",
        {"Authorization": r"Bearer .+"},
        [],
    )
