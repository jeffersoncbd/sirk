# S.I.R.K. agent reference

S.I.R.K. serves individual agents over HTTP. The caller owns workflow
sequencing, project files, Git, and user interaction. Agent input is passed
through as literal text, including `{{ ... }}` sequences.

## HTTP

`sirk http [address]` binds to `127.0.0.1:8080` by default. Create a flow
before making execution requests:

```http
POST /v1/flows
Content-Type: application/json

{"directory":"/workspace/project"}
```

The server returns HTTP 201 with `{"flowId":"flow-..."}` and creates
`history/<flow-id>/` with `flow.log`, `usage.log`, and `conversations/`. Every
later
`POST /v1/agent/run`, `POST /v1/tree`, `POST /v1/git/status`, and
`POST /v1/git/add` request must include that value in `X-Sirk-Flow-Id`. The
flow ID is valid only for the directory used to create it. Health, OpenAPI,
and Swagger requests do not use a flow ID.

`POST /v1/agent/run` then uses the flow header:

```http
POST /v1/agent/run
Content-Type: application/json
X-Sirk-Flow-Id: flow-...

{"directory":"/workspace/project","agent":"code-explainer","input":"Explain this module"}
```

The directory must be visible to the server. A standalone successful call
returns HTTP 200 with `{"result":"..."}`. An `ASK_TOOL` request returns a
`conversationId` with its `ask` response. Invalid requests return 400, unknown
routes 404, unsupported methods 405, and execution errors 500 with
`{"error":"..."}`. HTTP has no authentication or TLS; use a trusted network
for external binds.

The Rust SDK's `Sirk::connect()` checks the default HTTP service and sends the
calling process's current directory. `Sirk::connect_to(endpoint, directory)`
selects another endpoint and server-visible directory. The SDK owns
confirmation helpers; agent tool requests remain in S.I.R.K.

`POST /v1/tree` accepts `{"directory":"/workspace/project"}` and the required
`X-Sirk-Flow-Id` header, then returns
`{"paths":["src/lib.rs"]}`. It lists tracked and non-ignored untracked regular
files relative to the requested directory, respecting `.treeignore`. TREE is
implemented by S.I.R.K.; clients use this endpoint instead of a filesystem
helper. `POST /v1/git/status` accepts the same body and header and
returns `{"paths":["src/lib.rs"]}`. It lists modified, untracked, and deleted
regular files relative to the requested directory, respecting `.treeignore`.
`POST /v1/git/add` accepts the same body and header, stages all changes below that
directory with Git, and returns `{"status":"ok"}`. These endpoints are for
SDK workflows and are not agent tools.

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
not shell syntax. `TREE_TOOL: allow`, `READ_TOOL: allow`, `ASK_TOOL: allow`,
`EDIT_TOOL: allow`, and `DELETE_TOOL: allow` grant the corresponding agent
requests. The optional
`DELETE_WITHOUT_CONFIRM: allow` requires `DELETE_TOOL: allow`.

`ask` is accepted as an initial question, but HTTP has no interactive answer
channel; such a call fails when input is required. `json: true` is not
supported. Unknown metadata, including `write`, is rejected. Codex runs in a
read-only sandbox with no approval escalation. OpenCode is invoked without
`--auto`; configure its permissions in the OpenCode environment.

`./sirk --new-agent` (also `--newAgent`) generates and validates an agent
definition. It asks separately for the created agent's adapter/model and the
generator's adapter/model. Existing definitions are never overwritten.

## Agent tool requests

With `READ_TOOL: allow`, the agent can answer with a standalone `READ: <path>`
line. READ returns the
exact UTF-8 contents of a regular file inside the execution directory. To read
a page, it can answer with `READ:` followed by one JSON object with `path`,
positive one-based `offset`, and positive `limit`; omitted values default to
the first line and 2,000 lines. It rejects symlinks resolving outside that
directory. `.readignore` blocks paths matching its Git-style patterns; blank
lines and `#` comments are ignored. For agents with `EDIT_TOOL: allow`, READ
results include the original source line numbers.

With `TREE_TOOL: allow`, an agent can answer with exactly `TREE`. It receives a
JSON array of sorted, unique relative paths. Git ignores apply to untracked
files, and `.treeignore` additionally hides tracked or untracked paths. TREE
requires a Git working tree.

With `ASK_TOOL: allow`, an agent can answer with `ASK: <question>` and nothing
else. S.I.R.K. ends the current run and returns
`{"conversationId":"conversation-...","ask":"<question>"}` and creates the
conversation log. The caller presents it to the user, then sends the answer in
a new agent request with that `conversationId`. S.I.R.K. restores the saved
messages and includes them in the next model prompt. A completed continuation
returns the same conversation ID with `result`; completed conversations cannot
be continued.

With `EDIT_TOOL: allow`, the agent can answer with `EDIT:` followed by one JSON
object containing `filePath`, `oldString`, `newString`, and optional
`replaceAll`. The exact old string must occur once unless `replaceAll` is true.
S.I.R.K. reads the current UTF-8 file, enforces `.readignore`, verifies it has
not changed before writing, and returns a plain diff. The legacy line-based
form with `path`, `operation`, and `input` remains supported for existing
agents.

With `EDIT_TOOL: allow`, the agent can also answer with `WRITE:` followed by
one JSON object containing `filePath` and `content`. It creates or atomically
replaces one UTF-8 regular file, creating needed directories inside the
execution directory. WRITE enforces `.readignore`, rejects paths outside that
directory and symbolic links, records its preparation before mutation, and
returns a plain diff. S.I.R.K. does not expose Bash, Python, or any other
command-execution tool to agents.

With `DELETE_TOOL: allow`, the agent can answer with `DELETE:` followed by a
JSON object containing `path`. HTTP cannot collect confirmation, so deletion
needs `"force":true` and `DELETE_WITHOUT_CONFIRM: allow`. DELETE accepts only
an existing regular file inside the execution directory.

Only one tool request is processed per agent response. A failed provider call
or empty response ends the call with an error. No provider session is resumed;
each turn receives explicit context reconstructed from the conversation JSON.

## Conversation logs

Each flow stores its artifacts under `history/<flowId>/`. Each provider call
appends `==> INPUT` followed by the complete
prompt delivered to the model and `<== OUTPUT` followed by the complete
normalized model response to `flow.log`; `usage.log` contains aggregate model
usage. S.I.R.K. creates `conversations/<conversationId>.json` only when an
agent returns an authorized `ASK:` request. Each file contains the agent,
status, and user, assistant, and tool messages used to restore that
conversation. The flow is locked while an agent call writes its artifacts. The
CLI has no resume command.
