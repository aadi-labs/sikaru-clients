from .conftest import get_client, verify_auth_headers, verify_request_count

from sikaru_api import CaptureItem, NewDataset


def test_datasets_list_datasets() -> None:
    """Test list_datasets endpoint with WireMock"""
    test_id = "datasets.list_datasets.0"
    client = get_client(test_id)
    client.datasets.list_datasets(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/datasets", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/datasets", {"Authorization": r"Bearer .+"}, [])


def test_datasets_create_dataset() -> None:
    """Test create_dataset endpoint with WireMock"""
    test_id = "datasets.create_dataset.0"
    client = get_client(test_id)
    client.datasets.create_dataset(
        project_id="project_id",
        name="name",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/datasets", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/datasets", {"Authorization": r"Bearer .+"}, [])


def test_datasets_capture_into_new_dataset() -> None:
    """Test capture_into_new_dataset endpoint with WireMock"""
    test_id = "datasets.capture_into_new_dataset.0"
    client = get_client(test_id)
    client.datasets.capture_into_new_dataset(
        project_id="project_id",
        dataset=NewDataset(
            name="name",
        ),
        idempotency_key="idempotency_key",
        items=[
            CaptureItem(
                run_id="run_id",
            )
        ],
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/datasets/capture", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/datasets/capture", {"Authorization": r"Bearer .+"}, []
    )


def test_datasets_get_dataset() -> None:
    """Test get_dataset endpoint with WireMock"""
    test_id = "datasets.get_dataset.0"
    client = get_client(test_id)
    client.datasets.get_dataset(
        project_id="project_id",
        dataset_id="dataset_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/datasets/dataset_id", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/datasets/dataset_id", {"Authorization": r"Bearer .+"}, []
    )


def test_datasets_delete_dataset() -> None:
    """Test delete_dataset endpoint with WireMock"""
    test_id = "datasets.delete_dataset.0"
    client = get_client(test_id)
    client.datasets.delete_dataset(
        project_id="project_id",
        dataset_id="dataset_id",
    )
    verify_request_count(test_id, "DELETE", "/v1/projects/project_id/datasets/dataset_id", None, 1)
    verify_auth_headers(
        test_id, "DELETE", "/v1/projects/project_id/datasets/dataset_id", {"Authorization": r"Bearer .+"}, []
    )


def test_datasets_update_dataset() -> None:
    """Test update_dataset endpoint with WireMock"""
    test_id = "datasets.update_dataset.0"
    client = get_client(test_id)
    client.datasets.update_dataset(
        project_id="project_id",
        dataset_id="dataset_id",
    )
    verify_request_count(test_id, "PATCH", "/v1/projects/project_id/datasets/dataset_id", None, 1)
    verify_auth_headers(
        test_id, "PATCH", "/v1/projects/project_id/datasets/dataset_id", {"Authorization": r"Bearer .+"}, []
    )


def test_datasets_capture_into_dataset() -> None:
    """Test capture_into_dataset endpoint with WireMock"""
    test_id = "datasets.capture_into_dataset.0"
    client = get_client(test_id)
    client.datasets.capture_into_dataset(
        project_id="project_id",
        dataset_id="dataset_id",
        idempotency_key="idempotency_key",
        items=[
            CaptureItem(
                run_id="run_id",
            )
        ],
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/datasets/dataset_id/capture", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/datasets/dataset_id/capture", {"Authorization": r"Bearer .+"}, []
    )


def test_datasets_start_dataset_checks() -> None:
    """Test start_dataset_checks endpoint with WireMock"""
    test_id = "datasets.start_dataset_checks.0"
    client = get_client(test_id)
    client.datasets.start_dataset_checks(
        project_id="project_id",
        dataset_id="dataset_id",
        agent_slug="agent_slug",
        idempotency_key="idempotency_key",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/datasets/dataset_id/checks", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/datasets/dataset_id/checks", {"Authorization": r"Bearer .+"}, []
    )


def test_datasets_preview_dataset_checks() -> None:
    """Test preview_dataset_checks endpoint with WireMock"""
    test_id = "datasets.preview_dataset_checks.0"
    client = get_client(test_id)
    client.datasets.preview_dataset_checks(
        project_id="project_id",
        dataset_id="dataset_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/datasets/dataset_id/checks/preview", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/datasets/dataset_id/checks/preview",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_datasets_list_dataset_check_results() -> None:
    """Test list_dataset_check_results endpoint with WireMock"""
    test_id = "datasets.list_dataset_check_results.0"
    client = get_client(test_id)
    client.datasets.list_dataset_check_results(
        project_id="project_id",
        dataset_id="dataset_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/datasets/dataset_id/checks/results", None, 1)
    verify_auth_headers(
        test_id,
        "GET",
        "/v1/projects/project_id/datasets/dataset_id/checks/results",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_datasets_list_examples() -> None:
    """Test list_examples endpoint with WireMock"""
    test_id = "datasets.list_examples.0"
    client = get_client(test_id)
    client.datasets.list_examples(
        project_id="project_id",
        dataset_id="dataset_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/datasets/dataset_id/examples", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/datasets/dataset_id/examples", {"Authorization": r"Bearer .+"}, []
    )


def test_datasets_delete_example() -> None:
    """Test delete_example endpoint with WireMock"""
    test_id = "datasets.delete_example.0"
    client = get_client(test_id)
    client.datasets.delete_example(
        project_id="project_id",
        dataset_id="dataset_id",
        example_id="example_id",
    )
    verify_request_count(test_id, "DELETE", "/v1/projects/project_id/datasets/dataset_id/examples/example_id", None, 1)
    verify_auth_headers(
        test_id,
        "DELETE",
        "/v1/projects/project_id/datasets/dataset_id/examples/example_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_datasets_update_example() -> None:
    """Test update_example endpoint with WireMock"""
    test_id = "datasets.update_example.0"
    client = get_client(test_id)
    client.datasets.update_example(
        project_id="project_id",
        dataset_id="dataset_id",
        example_id="example_id",
    )
    verify_request_count(test_id, "PATCH", "/v1/projects/project_id/datasets/dataset_id/examples/example_id", None, 1)
    verify_auth_headers(
        test_id,
        "PATCH",
        "/v1/projects/project_id/datasets/dataset_id/examples/example_id",
        {"Authorization": r"Bearer .+"},
        [],
    )


def test_datasets_export_dataset() -> None:
    """Test export_dataset endpoint with WireMock"""
    test_id = "datasets.export_dataset.0"
    client = get_client(test_id)
    for _ in client.datasets.export_dataset(
        project_id="project_id",
        dataset_id="dataset_id",
    ):
        pass
    verify_request_count(test_id, "GET", "/v1/projects/project_id/datasets/dataset_id/export", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/datasets/dataset_id/export", {"Authorization": r"Bearer .+"}, []
    )


def test_datasets_upload_examples() -> None:
    """Test upload_examples endpoint with WireMock"""
    test_id = "datasets.upload_examples.0"
    client = get_client(test_id)
    client.datasets.upload_examples(
        project_id="project_id",
        dataset_id="dataset_id",
        content="content",
        format="csv",
        idempotency_key="idempotency_key",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/datasets/dataset_id/uploads", None, 1)
    verify_auth_headers(
        test_id, "POST", "/v1/projects/project_id/datasets/dataset_id/uploads", {"Authorization": r"Bearer .+"}, []
    )


def test_datasets_list_versions() -> None:
    """Test list_versions endpoint with WireMock"""
    test_id = "datasets.list_versions.0"
    client = get_client(test_id)
    client.datasets.list_versions(
        project_id="project_id",
        dataset_id="dataset_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/datasets/dataset_id/versions", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/datasets/dataset_id/versions", {"Authorization": r"Bearer .+"}, []
    )
