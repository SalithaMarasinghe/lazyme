# AGENTS.md

Guidelines and workflows for AI coding agents working in `lazyme`.

## Behavioral guidelines (Karpathy Guidelines)

Always follow the `karpathy-guidelines` skill (`.agents/skills/karpathy-guidelines/SKILL.md`) alongside the Matt Pocock engineering skills (`mattpocock-skills`).

1. **Think Before Coding** — State assumptions explicitly. If uncertain or multiple interpretations exist, surface tradeoffs and ask rather than picking silently.
2. **Simplicity First** — Write the minimum code that solves the problem. No speculative features, premature abstractions, or unrequested configurability.
3. **Surgical Changes** — Touch only what is strictly necessary for the task. Do not refactor unrelated working code or leave behind unused orphans you created.
4. **Goal-Driven Execution** — Define verifiable success criteria before editing (e.g., failing test -> passing test, build/typecheck passing) and loop until verified.

## Agent skills

### Issue tracker

GitHub Issues via the `gh` CLI (`SalithaMarasinghe/lazyme`). See `docs/agents/issue-tracker.md`.

### Triage labels

Default five-role vocabulary (`needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`). See `docs/agents/triage-labels.md`.

### Domain docs

Single-context (`GLOSSARY.md` and `docs/adr/` at the repo root). See `docs/agents/domain.md`.
