# 0002: Clipboard Round-Trip for Text Capture and Paste

Status: accepted

LazyMe captures Raw Input from the focused application and pastes the Engineered Prompt back via simulated clipboard round-trip shortcuts (releasing stuck hotkey modifiers, capturing text via clipboard, and restoring the user's previous clipboard after a short post-paste delay) instead of Windows UI Automation (UIA).

## Considered Options

- **Clipboard round-trip**: Works consistently across browsers, Electron apps (VS Code, Slack, Discord), terminals, and voice-dictation boxes behind a single OS-capture trait.
- **Windows UI Automation (UIA) first**: Avoids touching the clipboard, but Chromium/Electron and terminal controls expose inconsistent text-pattern support and still require clipboard fallback.

## Consequences

- An OS-adapter trait isolates capture/paste mechanics so UIA or macOS/Linux adapters can be added later without changing core orchestration.
- Clipboard restoration requires a brief delay (~150–250 ms) after paste so the target application finishes reading the clipboard before the previous contents are restored.
- Applications running elevated (As Administrator) cannot receive simulated keystrokes from a non-elevated LazyMe process due to Windows UIPI restrictions; this must be surfaced clearly as an error.
