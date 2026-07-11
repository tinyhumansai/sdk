from __future__ import annotations

from typing import Any

from ..http import Json
from ._base import ApiNamespace

__all__ = ["InferenceApi"]


class InferenceApi(ApiNamespace):
    """OpenAI-compatible inference: models, completions, embeddings, speech, transcription.

    These endpoints return native OpenAI-format bodies and are NOT wrapped in the
    TinyHumans ``{ success, data }`` envelope, so every method defaults to
    ``unwrap_envelope=False``. Callers can override via keyword arguments.
    """

    def list_models(
        self, query: dict[str, Any] | None = None, **kwargs: Any
    ) -> Json:
        """List available models in OpenAI format."""
        merged = self._merge_query(query, kwargs.pop("query", None))
        kwargs.setdefault("unwrap_envelope", False)
        return self._http.get("/openai/v1/models", query=merged, **kwargs)

    def create_chat_completion(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Create a chat completion."""
        kwargs.setdefault("unwrap_envelope", False)
        return self._http.post("/openai/v1/chat/completions", body=body, **kwargs)

    def create_completion(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Create a text completion."""
        kwargs.setdefault("unwrap_envelope", False)
        return self._http.post("/openai/v1/completions", body=body, **kwargs)

    def create_transcription(self, body: Any, **kwargs: Any) -> Json:
        """Transcribe audio to text."""
        kwargs.setdefault("unwrap_envelope", False)
        return self._http.post("/openai/v1/audio/transcriptions", body=body, **kwargs)

    def create_speech(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Synthesize speech from text via ElevenLabs (mp3)."""
        kwargs.setdefault("unwrap_envelope", False)
        return self._http.post("/openai/v1/audio/speech", body=body, **kwargs)

    def create_embeddings(self, body: dict[str, Any], **kwargs: Any) -> Json:
        """Create text embeddings via Voyage AI."""
        kwargs.setdefault("unwrap_envelope", False)
        return self._http.post("/openai/v1/embeddings", body=body, **kwargs)
