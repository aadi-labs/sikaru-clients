from .conftest import get_client, verify_auth_headers, verify_request_count


def test_creditTopUps_create() -> None:
    """Test create endpoint with WireMock"""
    test_id = "credit_top_ups.create.0"
    client = get_client(test_id)
    client.credit_top_ups.create(
        project_id="project_id",
        amount_usd="amountUsd",
    )
    verify_request_count(test_id, "POST", "/v1/projects/project_id/credit-top-ups", None, 1)
    verify_auth_headers(test_id, "POST", "/v1/projects/project_id/credit-top-ups", {"Authorization": r"Bearer .+"}, [])


def test_creditTopUps_get() -> None:
    """Test get endpoint with WireMock"""
    test_id = "credit_top_ups.get.0"
    client = get_client(test_id)
    client.credit_top_ups.get(
        project_id="project_id",
        top_up_id="top_up_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/credit-top-ups/top_up_id", None, 1)
    verify_auth_headers(
        test_id, "GET", "/v1/projects/project_id/credit-top-ups/top_up_id", {"Authorization": r"Bearer .+"}, []
    )


def test_creditTopUps_get_balance() -> None:
    """Test get_balance endpoint with WireMock"""
    test_id = "credit_top_ups.get_balance.0"
    client = get_client(test_id)
    client.credit_top_ups.get_balance(
        project_id="project_id",
    )
    verify_request_count(test_id, "GET", "/v1/projects/project_id/credits", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/projects/project_id/credits", {"Authorization": r"Bearer .+"}, [])
