# LazyMe

LazyMe is a lightweight desktop utility that captures raw typed or voice-dictated text via a global hotkey and transforms it into a structured, grounded prompt using the user's own LLM API key.

## Language

**Raw Input**:
The unmodified text captured from the user's active application before transformation.
_Avoid_: Prompt, query, utterance, source text

**Engineered Prompt**:
The structured prompt returned by the LLM from a Raw Input (and any Clarification Answers) without inventing unstated requirements.
_Avoid_: Output, completion, response

**Clarification Round**:
A single-pass set of at most three critical questions—each offering selectable choices and optional custom input—shown when a Raw Input has genuine ambiguities, auto-submitting once all questions are answered.
_Avoid_: Multi-turn chat, follow-up wizard, interrogation

**Preview Popup**:
The floating overlay window where the user reviews, edits, copies, or pastes an Engineered Prompt back into the target application (including by pressing the global hotkey again).
_Avoid_: Modal, toast, dialog

**Auto-Replace**:
The delivery mode that immediately replaces the captured text in the active application with the Engineered Prompt without showing the Preview Popup.
_Avoid_: Fast mode, silent paste

**Provider Profile**:
A user-configured LLM connection consisting of a provider preset or custom endpoint URL, a model identifier, and a securely stored API key.
_Avoid_: Account, integration, backend

**Conversion Record**:
A saved local history entry pairing a Raw Input with its resulting Engineered Prompt and timestamp.
_Avoid_: Log, session, transcript
