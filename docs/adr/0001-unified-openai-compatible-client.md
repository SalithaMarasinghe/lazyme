# 0001: Unified OpenAI-Compatible LLM Client

Status: accepted

We use a single OpenAI-compatible `/chat/completions` HTTP client with configurable base URLs, models, and presets (Groq, OpenRouter, Gemini's OpenAI compatibility endpoint, DeepSeek, and custom/local URLs like Ollama) instead of integrating four separate provider-specific SDKs.

## Considered Options

- **Unified OpenAI-compatible client**: One request/response schema and error-handling path; adding a new compatible provider or local server requires only a URL + model preset.
- **Provider-specific SDKs per vendor**: Unlocks vendor-only features (such as native Gemini-only APIs or provider-specific caching/structured outputs), at the cost of maintaining four client adapters, auth flows, and test suites.

## Consequences

- Provider-specific parameters outside the OpenAI chat-completions subset are not supported in v1 unless added explicitly as optional extensions later.
- API keys are never stored in config files alongside URLs/models; they live in Windows Credential Manager via the OS keyring.
