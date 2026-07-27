from __future__ import annotations

import json
import threading
from http.server import BaseHTTPRequestHandler, HTTPServer
from urllib.parse import parse_qs, urlparse

import pytest

from tinyhumans.http import HttpClient, TinyHumansError


class _Handler(BaseHTTPRequestHandler):
    def log_message(self, *args: object) -> None:  # silence server logging
        pass

    def _echo(self) -> None:
        parsed = urlparse(self.path)
        path = parsed.path

        if path == "/status/204":
            self.send_response(204)
            self.end_headers()
            return

        if path == "/status/error":
            payload = json.dumps({"success": False, "error": "boom", "errorCode": "BAD"})
            body = payload.encode("utf-8")
            self.send_response(422)
            self.send_header("content-type", "application/json")
            self.send_header("content-length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
            return

        if path == "/text":
            body = b"just plain text"
            self.send_response(200)
            self.send_header("content-type", "text/plain")
            self.send_header("content-length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
            return

        length = int(self.headers.get("content-length", 0) or 0)
        raw_body = self.rfile.read(length).decode("utf-8") if length else None

        echo = {
            "method": self.command,
            "path": path,
            "query": {k: v for k, v in parse_qs(parsed.query, keep_blank_values=True).items()},
            "headers": {k.lower(): v for k, v in self.headers.items()},
            "body": raw_body,
        }
        payload = json.dumps({"success": True, "data": echo})
        body = payload.encode("utf-8")
        self.send_response(200)
        self.send_header("content-type", "application/json")
        self.send_header("content-length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    do_GET = _echo
    do_POST = _echo
    do_PUT = _echo
    do_PATCH = _echo
    do_DELETE = _echo


@pytest.fixture(scope="module")
def server_url() -> str:
    server = HTTPServer(("127.0.0.1", 0), _Handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    try:
        host, port = server.server_address
        yield f"http://127.0.0.1:{port}"
    finally:
        server.shutdown()
        server.server_close()
        thread.join(timeout=5)


@pytest.fixture()
def client(server_url: str) -> HttpClient:
    return HttpClient(base_url=server_url)


def test_get_unwraps_envelope_by_default(client: HttpClient) -> None:
    data = client.get("/echo")
    assert data["method"] == "GET"
    assert data["path"] == "/echo"


def test_post_put_patch_delete_methods(client: HttpClient) -> None:
    assert client.post("/echo")["method"] == "POST"
    assert client.put("/echo")["method"] == "PUT"
    assert client.patch("/echo")["method"] == "PATCH"
    assert client.delete("/echo")["method"] == "DELETE"


def test_query_params_list_and_none_skipped(client: HttpClient) -> None:
    data = client.get("/echo", query={"tag": ["a", "b"], "skip": None, "one": "x"})
    assert data["query"]["tag"] == ["a", "b"]
    assert data["query"]["one"] == ["x"]
    assert "skip" not in data["query"]


def test_default_headers_reach_server(client: HttpClient) -> None:
    data = client.get("/echo")
    assert data["headers"]["accept"] == "application/json"
    assert data["headers"]["x-sdk-client"] == "tinyhumans-python"


def test_auth_headers_reach_server(server_url: str) -> None:
    http = HttpClient(
        base_url=server_url,
        token="tok",
        api_key="key",
        headers={"x-custom": "c"},
    )
    data = http.get("/echo")
    assert data["headers"]["authorization"] == "Bearer tok"
    assert data["headers"]["x-api-key"] == "key"
    assert "x-admin-service-token" not in data["headers"]
    assert data["headers"]["x-custom"] == "c"


def test_str_body_sent_verbatim(client: HttpClient) -> None:
    data = client.post("/echo", body="raw-string")
    assert data["body"] == "raw-string"
    assert data["headers"]["content-type"] == "application/json"


def test_dict_body_json_encoded(client: HttpClient) -> None:
    data = client.post("/echo", body={"a": 1})
    assert json.loads(data["body"]) == {"a": 1}


def test_unwrap_envelope_false_returns_full_envelope(client: HttpClient) -> None:
    body = client.get("/echo", unwrap_envelope=False)
    assert body["success"] is True
    assert "data" in body


def test_204_returns_none(client: HttpClient) -> None:
    assert client.get("/status/204") is None


def test_error_response_raises(client: HttpClient) -> None:
    with pytest.raises(TinyHumansError) as excinfo:
        client.get("/status/error")
    err = excinfo.value
    assert err.status == 422
    assert err.body == {"success": False, "error": "boom", "errorCode": "BAD"}
    assert "boom" in str(err)


def test_plain_text_response_returned_as_string(client: HttpClient) -> None:
    assert client.get("/text") == "just plain text"


def test_error_without_message_uses_default() -> None:
    err = TinyHumansError(500, None)
    assert err.status == 500
    assert "HTTP 500" in str(err)
