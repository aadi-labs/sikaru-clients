from .conftest import get_client, verify_auth_headers, verify_request_count


def test_auth_get_device_configuration() -> None:
    """Test get_device_configuration endpoint with WireMock"""
    test_id = "auth.get_device_configuration.0"
    client = get_client(test_id)
    client.auth.get_device_configuration()
    verify_request_count(test_id, "GET", "/v1/auth/device/config", None, 1)
    verify_auth_headers(test_id, "GET", "/v1/auth/device/config", {}, ["Authorization"])
