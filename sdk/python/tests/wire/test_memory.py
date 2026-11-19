from .conftest import get_client, verify_auth_headers, verify_request_count


def test_memory_get_dream() -> None:
    """Test get_dream endpoint with WireMock"""
    test_id = "memory.get_dream.0"
    client = get_client(test_id)
    client.memory.get_dream(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/memory/dreams", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/memory/dreams",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_memory_dream() -> None:
    """Test dream endpoint with WireMock"""
    test_id = "memory.dream.0"
    client = get_client(test_id)
    client.memory.dream(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/managed-agents/agent_slug/memory/dreams", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/memory/dreams",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_memory_list_files() -> None:
    """Test list_files endpoint with WireMock"""
    test_id = "memory.list_files.0"
    client = get_client(test_id)
    client.memory.list_files(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/memory/files", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/memory/files",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_memory_read_file() -> None:
    """Test read_file endpoint with WireMock"""
    test_id = "memory.read_file.0"
    client = get_client(test_id)
    client.memory.read_file(
        project_id="project_id",
        agent_slug="agent_slug",
        path="path",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/memory/files/content", {"path": "path"}, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/memory/files/content",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_memory_write_file() -> None:
    """Test write_file endpoint with WireMock"""
    test_id = "memory.write_file.0"
    client = get_client(test_id)
    client.memory.write_file(
        project_id="project_id",
        agent_slug="agent_slug",
        content="content",
        path="path",
    )
    verify_request_count(
        test_id, "PUT", "/v1/projects/project_id/managed-agents/agent_slug/memory/files/content", None, 1
    )
    verify_auth_headers(
        test_id,
        "PUT",
        "/v1/projects/project_id/managed-agents/agent_slug/memory/files/content",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_memory_delete_file() -> None:
    """Test delete_file endpoint with WireMock"""
    test_id = "memory.delete_file.0"
    client = get_client(test_id)
    client.memory.delete_file(
        project_id="project_id",
        agent_slug="agent_slug",
        path="path",
        base="base",
    )
    verify_request_count(
        test_id,
        "DELETE",
        "/v1/projects/project_id/managed-agents/agent_slug/memory/files/content",
        {"path": "path", "base": "base"},
        1,
    )
    verify_auth_headers(
        test_id,
        "DELETE",
        "/v1/projects/project_id/managed-agents/agent_slug/memory/files/content",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_memory_move_file() -> None:
    """Test move_file endpoint with WireMock"""
    test_id = "memory.move_file.0"
    client = get_client(test_id)
    client.memory.move_file(
        project_id="project_id",
        agent_slug="agent_slug",
        path="path",
        to="to",
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/managed-agents/agent_slug/memory/files/move", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/memory/files/move",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_memory_list_forgets() -> None:
    """Test list_forgets endpoint with WireMock"""
    test_id = "memory.list_forgets.0"
    client = get_client(test_id)
    client.memory.list_forgets(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/memory/forgets", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/memory/forgets",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_memory_forget() -> None:
    """Test forget endpoint with WireMock"""
    test_id = "memory.forget.0"
    client = get_client(test_id)
    client.memory.forget(
        project_id="project_id",
        agent_slug="agent_slug",
        target="target",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/managed-agents/agent_slug/memory/forgets", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/memory/forgets",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_memory_get_forget() -> None:
    """Test get_forget endpoint with WireMock"""
    test_id = "memory.get_forget.0"
    client = get_client(test_id)
    client.memory.get_forget(
        project_id="project_id",
        agent_slug="agent_slug",
        job_id="job_id",
    )
    verify_request_count(
        test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/memory/forgets/job_id", None, 1
    )
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/memory/forgets/job_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_memory_remember() -> None:
    """Test remember endpoint with WireMock"""
    test_id = "memory.remember.0"
    client = get_client(test_id)
    client.memory.remember(
        project_id="project_id",
        agent_slug="agent_slug",
        target="target",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/managed-agents/agent_slug/memory/remember", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/memory/remember",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_memory_list_versions() -> None:
    """Test list_versions endpoint with WireMock"""
    test_id = "memory.list_versions.0"
    client = get_client(test_id)
    client.memory.list_versions(
        project_id="project_id",
        agent_slug="agent_slug",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/managed-agents/agent_slug/memory/versions", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/managed-agents/agent_slug/memory/versions",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_memory_rollback() -> None:
    """Test rollback endpoint with WireMock"""
    test_id = "memory.rollback.0"
    client = get_client(test_id)
    client.memory.rollback(
        project_id="project_id",
        agent_slug="agent_slug",
        version=1,
    )
    verify_request_count(
        test_id, "POST", "/v1/projects/project_id/managed-agents/agent_slug/memory/versions/1/rollback", None, 1
    )
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/managed-agents/agent_slug/memory/versions/1/rollback",
        {"Authorization": r"Bearer .+"},
        [],
    )
