from __future__ import annotations

from tinyhumans.api.inference import InferenceApi
from helpers import RecordingHttp


def test_list_models_sets_no_unwrap_and_query() -> None:
    http = RecordingHttp({"body": {"object": "list", "data": []}})
    api = InferenceApi(http)

    result = api.list_models({"with_display": "1"})

    call = http.last
    assert call["method"] == "GET"
    assert call["path"] == "/openai/v1/models"
    assert call["query"] == {"with_display": "1"}
    assert call["unwrap_envelope"] is False
    # Non-enveloped body is returned as-is.
    assert result == {"object": "list", "data": []}


def test_create_chat_completion_no_unwrap() -> None:
    http = RecordingHttp({"body": {"id": "chatcmpl-1", "choices": []}})
    api = InferenceApi(http)

    result = api.create_chat_completion({"model": "gpt", "messages": []})

    call = http.last
    assert call["method"] == "POST"
    assert call["path"] == "/openai/v1/chat/completions"
    assert call["body"] == {"model": "gpt", "messages": []}
    assert call["unwrap_envelope"] is False
    assert result == {"id": "chatcmpl-1", "choices": []}


def test_create_completion_no_unwrap() -> None:
    http = RecordingHttp({"body": {"id": "cmpl-1"}})
    api = InferenceApi(http)

    result = api.create_completion({"model": "gpt", "prompt": "hi"})

    call = http.last
    assert call["path"] == "/openai/v1/completions"
    assert call["unwrap_envelope"] is False
    assert result == {"id": "cmpl-1"}


def test_create_transcription_no_unwrap() -> None:
    http = RecordingHttp({"body": {"text": "hello"}})
    api = InferenceApi(http)

    result = api.create_transcription({"file": "f"})

    call = http.last
    assert call["path"] == "/openai/v1/audio/transcriptions"
    assert call["unwrap_envelope"] is False
    assert result == {"text": "hello"}


def test_create_speech_no_unwrap() -> None:
    http = RecordingHttp({"body": {"audio": "bytes"}})
    api = InferenceApi(http)

    result = api.create_speech({"text": "hi"})

    call = http.last
    assert call["path"] == "/openai/v1/audio/speech"
    assert call["unwrap_envelope"] is False
    assert result == {"audio": "bytes"}


def test_create_embeddings_no_unwrap() -> None:
    http = RecordingHttp({"body": {"data": [1, 2, 3]}})
    api = InferenceApi(http)

    result = api.create_embeddings({"model": "voyage", "input": "hi"})

    call = http.last
    assert call["path"] == "/openai/v1/embeddings"
    assert call["unwrap_envelope"] is False
    assert result == {"data": [1, 2, 3]}


def test_caller_can_override_unwrap() -> None:
    http = RecordingHttp({"data": {"unwrapped": True}})
    api = InferenceApi(http)

    api.create_embeddings({"model": "voyage", "input": "hi"}, unwrap_envelope=True)

    assert http.last["unwrap_envelope"] is True
