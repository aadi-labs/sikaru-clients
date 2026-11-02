from .conftest import get_client, verify_auth_headers, verify_request_count


def test_issueClusters_list_issue_clusters() -> None:
    """Test list_issue_clusters endpoint with WireMock"""
    test_id = "issue_clusters.list_issue_clusters.0"
    client = get_client(test_id)
    client.issue_clusters.list_issue_clusters(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/issue-clusters", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/issue-clusters", {"Authorization": r"Bearer .+"}, [])


def test_issueClusters_upsert_issue_cluster() -> None:
    """Test upsert_issue_cluster endpoint with WireMock"""
    test_id = "issue_clusters.upsert_issue_cluster.0"
    client = get_client(test_id)
    client.issue_clusters.upsert_issue_cluster(
        project_id="project_id",
        label="label",
        trace_ids=["traceIds"],
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/issue-clusters", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/issue-clusters", {"Authorization": r"Bearer .+"}, [])


def test_issueClusters_get_issue_cluster() -> None:
    """Test get_issue_cluster endpoint with WireMock"""
    test_id = "issue_clusters.get_issue_cluster.0"
    client = get_client(test_id)
    client.issue_clusters.get_issue_cluster(
        project_id="project_id",
        cluster_id="cluster_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/issue-clusters/cluster_id", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/issue-clusters/cluster_id", {"Authorization": r"Bearer .+"}, []
    )


def test_issueClusters_update_issue_cluster_status() -> None:
    """Test update_issue_cluster_status endpoint with WireMock"""
    test_id = "issue_clusters.update_issue_cluster_status.0"
    client = get_client(test_id)
    client.issue_clusters.update_issue_cluster_status(
        project_id="project_id",
        cluster_id="cluster_id",
        status="status",
    )
    verify_request_count(test_id, "PATCH", "/v1/projects/project_id/issue-clusters/cluster_id", None, 1)
    verify_auth_headers(
        test_id, "PATCH", "/v1/projects/project_id/issue-clusters/cluster_id", {"Authorization": r"Bearer .+"}, []
    )


def test_issueClusters_propose_issue_cluster_fix() -> None:
    """Test propose_issue_cluster_fix endpoint with WireMock"""
    test_id = "issue_clusters.propose_issue_cluster_fix.0"
    client = get_client(test_id)
    client.issue_clusters.propose_issue_cluster_fix(
        project_id="project_id",
        cluster_id="cluster_id",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/issue-clusters/cluster_id/propose-fix", None, 1)
    verify_auth_headers(
        test_id,
        "POST",
        "/v1/projects/project_id/issue-clusters/cluster_id/propose-fix",
        {"Authorization": r"Bearer .+"},
        [],
    )
