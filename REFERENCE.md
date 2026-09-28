# S.I.R.K. agent reference

S.I.R.K. serves individual agents over HTTP or JSON-RPC. The caller owns
workflow sequencing, project files, Git, and user interaction. Agent input is
passed through as literal text, including `{{ ... }}` sequences.

## Transports

`sirk rpc` reads one JSON-RPC 2.0 request per line from stdin and writes one
response per line to stdout. It uses the process working directory. The method
is `agent.run`, with `agent` and `input` string parameters.

`sirk http [address]` binds to `127.0.0.1:8080` by default. It provides
`GET /health` and `POST /v1/agent/run`:

```http
POST /v1/agent/run
Content-Type: application/json

{"directory":"/workspace/project","agent":"code-explainer","input":"Explain this module"}
```

The directory must be visible to the server. A successful call returns HTTP
200 with `{"result":"..."}`. Invalid requests return 400, unknown routes 404,
unsupported methods 405, and execution errors 500 with `{"error":"..."}`.
HTTP has no authentication or TLS; use a trusted network for external binds.

The Rust SDK's `Sirk::connect()` checks the default HTTP service and sends the
calling process's current directory. `Sirk::connect_to(endpoint, directory)`
selects another endpoint and server-visible directory. The SDK owns host-side
Git, TREE, and confirmation helpers; agent tool requests remain in S.I.R.K.

## Agent definitions

Create `.agents/<name>.md`. Names use ASCII letters, digits, underscores, or
hyphens. The Markdown body contains nonempty instructions:

```markdown
---
adapter: codex
model: gpt-6-sol
call_prefix: [docker, exec, -i, harness]
TREE_TOOL: allow
---

Explain the requested module and cite the relevant functions.
```

`adapter` is required. `model` is optional and is trimmed and lowercased;
omitting it uses the adapter default. `call_prefix` is an optional command
token or list of literal arguments prepended to the adapter invocation. It is
not shell syntax. `TREE_TOOL: allow`, `EDIT_TOOL: allow`, and
`DELETE_TOOL: allow` grant the corresponding agent requests. The optional
`DELETE_WITHOUT_CONFIRM: allow` requires `DELETE_TOOL: allow`.

`ask` is accepted as an initial question, but HTTP and RPC have no interactive
answer channel; such a call fails when input is required. `json: true` is not
supported. Unknown metadata, including `write`, is rejected. Codex runs in a
read-only sandbox with no approval escalation. OpenCode is invoked without
`--auto`; configure its permissions in the OpenCode environment.

`./sirk --new-agent` (also `--newAgent`) generates and validates an agent
definition. It asks separately for the created agent's adapter/model and the
generator's adapter/model. Existing definitions are never overwritten.

## Agent tool requests

The agent can answer with a standalone `READ: <path>` line. READ returns the
exact UTF-8 contents of a regular file inside the execution directory. It
rejects symlinks resolving outside that directory. `.readignore` blocks paths
matching its Git-style patterns; blank lines and `#` comments are ignored.
For agents with `EDIT_TOOL: allow`, READ results include source line numbers.

With `TREE_TOOL: allow`, an agent can answer with exactly `TREE`. It receives a
JSON array of sorted, unique relative paths. Git ignores apply to untracked
files, and `.treeignore` additionally hides tracked or untracked paths. TREE
requires a Git working tree.

With `EDIT_TOOL: allow`, the agent can answer with `EDIT:` followed by one JSON
object containing `path`, `operation`, and `input`. Operations are `insert`,
`delete`, `replace`, `prepend`, and `append`; insert also needs `line`, while
delete and replace need inclusive `start` and `end` coordinates. Coordinates
start at 1. The agent must not supply `version`; S.I.R.K. reads the current
file and verifies it before writing. A successful edit returns a plain diff
to the agent. The target must be an existing regular UTF-8 file inside the
execution directory.

With `DELETE_TOOL: allow`, the agent can answer with `DELETE:` followed by a
JSON object containing `path`. HTTP and RPC cannot collect confirmation, so
deletion needs `"force":true` and `DELETE_WITHOUT_CONFIRM: allow`. DELETE
accepts only an existing regular file inside the execution directory.

Only one tool request is processed per agent response. A failed provider call
or empty response ends the call with an error. No provider session is resumed;
each turn receives the explicit conversation reconstructed from the current
call's log.

## Conversation logs

Each agent call writes `history/run-<id>.log` under its execution directory.
The log records the loaded agent configuration and complete conversation using
`==> INPUT`, `<== OUTPUT`, `==> TREE`, `==> READ`, `==> EDIT`, and `==> DELETE`
markers. Input is saved before a provider call, and successful results are
saved after completion. Logs use atomic replacement and an exclusive lock.
They are diagnostic records; the CLI has no resume command.
