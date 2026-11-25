# 0003: Two-Stage Max Structured Clarification Protocol

Status: accepted

LazyMe enforces a strict maximum of two LLM calls per hotkey invocation using a structured JSON contract on Call 1 (`status: "ready"` with the `engineered_prompt`, or `status: "needs_clarification"` with 1–3 critical questions), followed by at most one synthesis call if clarification is needed.

## Considered Options

- **Two-stage max structured JSON**: Keeps the workflow strictly one-shot when clear (1 call) and at most one interactive Clarification Round when ambiguous (2 calls), enabling clickable option cards and auto-submit.
- **Open-ended multi-turn chat or freeform markdown streaming**: Lower perceived latency on token streaming, but breaks single-round UX, requires parsing mixed markdown/questions, and invites endless back-and-forth.

## Consequences

- Call 1 cannot stream the final prompt word-by-word to the UI because the payload must first distinguish `ready` vs. `needs_clarification`; fast models (e.g., Groq, Gemini Flash, DeepSeek) keep Call 1 latency low, and Call 2 returns the final Engineered Prompt directly.
- Provider endpoints/models configured by the user must reliably follow JSON output instructions; malformed JSON from a weak model must gracefully fall back or surface a clear error without losing the Raw Input.
