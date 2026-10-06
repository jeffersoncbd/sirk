# S.I.R.K. development contracts

Compatibility requirements for the agent service and SDK transports. Contributor
workflow and checks are in [AGENTS.md](AGENTS.md).

## Language, files, and permissions

- Every project artifact is English-only, regardless of the language used to
  request the work. Translate user-supplied text before placing it in source
  code, identifiers, tests, documentation, messages, errors, logs, agent
  definitions, or generated files.
- Resolve agent definitions and flow transcripts from the execution directory.
  Agent input is literal text; do not expand `{{ ... }}` or other template
  syntax.
- Normalize model names when loading Markdown metadata. Child stdin is closed.
- Keep Codex read-only with `approval_policy="never"` and
  `--skip-git-repo-check`. Do not add automatic write approvals or `write`
  agent metadata. Only explicit agent permissions enable external edits or
  deletion.

## SDK HTTP transport

- `sirk http` binds to `127.0.0.1:8080` by default. An explicit address may
  override the bind; external exposure does not imply authentication or TLS.
- Axum serves the HTTP API. `src/http/routes.rs` is the sole route registry;
  controllers contain endpoint behavior without registering paths or methods.
- Utoipa generates `openapi.yaml` from the route registry, annotated
  controllers, and DTOs under `src/http/schemas/`. The versioned YAML file must
  exactly match the generated document and must not be edited by hand.
- HTTP exposes `GET /health`, `GET /openapi.yaml`, `GET /swagger`,
  `POST /v1/flows`, `POST /v1/agent/run`, `POST /v1/tree`,
  `POST /v1/git/status`, and `POST /v1/git/add`.
  `/openapi.yaml` returns the embedded OpenAPI document as plain text.
  `/swagger` returns an HTML Swagger UI configured to load that document.
  Creating a flow requires the server-visible execution directory and returns
  `flowId`. Agent, TREE, and Git requests require `X-Sirk-Flow-Id`; it must
  identify an existing flow for the supplied directory. Agent requests also
  contain an agent ID and literal input. TREE and Git requests otherwise
  contain only the server-visible execution directory.
- A successful agent response contains `conversationId` and exactly one
  outcome field: `result` for a completed run, or `ask` when an agent with
  `ASK_TOOL: allow` requests one user answer from the caller. Omit
  `conversationId` in a request to create a conversation; include it with the
  user's answer to continue a conversation that is awaiting input.
- The Rust SDK connects to HTTP without starting a CLI process. Its default
  endpoint is `http://127.0.0.1:8080`; `Sirk::connect()` sends the caller's
  current directory. Custom endpoints preserve supplied server-visible paths.
- TREE returns sorted, unique UTF-8 paths for tracked and non-ignored untracked
  files, excluding `.treeignore` matches. Git status returns sorted, unique
  UTF-8 paths for modified, untracked, and deleted regular files, excluding
  `.treeignore` matches. Git add stages all changes below the supplied directory.
  These HTTP endpoints are not agent tools.

## Agent tools

- TREE returns a pretty-printed JSON array of relative paths and `[]` for no
  files. Git ignores apply to untracked files; `.treeignore` independently
  removes matching paths, including tracked paths. Preserve path bytes during
  listing and reject non-UTF-8 paths at JSON rendering.
- READ returns exact UTF-8 contents, including empty text. Agents with
  `READ_TOOL: allow` can request it; agents with `EDIT_TOOL: allow` receive
  line-numbered READ results. READ only accepts
  regular files inside the execution directory; `.readignore` controls access.
- Recognize ordinary requests only as standalone `TREE` from agents with
  `TREE_TOOL: allow`, or one-line `READ: <path>` responses from agents with
  `READ_TOOL: allow`.
- EDIT, WRITE, and DELETE require their explicit agent permissions. EDIT and
  WRITE require `EDIT_TOOL: allow`; persist their preparation before mutation,
  protect against intervening file changes, and return plain diffs without ANSI
  formatting. DELETE removes one regular file inside the execution directory;
  a noninteractive request requires both `DELETE_TOOL: allow` and
  `DELETE_WITHOUT_CONFIRM: allow` with `force: true`.
- ASK requires `ASK_TOOL: allow`. An `ASK: <question>` response ends the
  current agent run and returns the question to the caller; the caller owns
  collecting and submitting any answer in a later request.
- Do not expose host-side SDK filesystem or Git helpers as agent tools.

## Conversations and history

- Reconstruct explicit conversation context from the persisted conversation
  messages for each provider turn; do not adopt provider session resumption.
- A flow stores `flow.log`, `usage.log`, and `conversations/` under
  `history/<flow-id>/`. Before each provider call, append the
  complete rendered prompt; after it, append the complete normalized model
  response and any provider-reported input and output token counts. Do not
  record agent-definition metadata, resumable state, or tool-only entries. Use
  file and directory synchronization plus exclusive locks.
- Store each conversation at
  `history/<flow-id>/conversations/<conversation-id>.json`. Bind it to one flow
  and agent, persist user, assistant, and tool messages atomically, and only
  continue a conversation whose status is awaiting user input.
- Empty agent responses fail; empty READ results are valid. Flow transcripts
  are not resumed through the CLI.
- Each flow's `usage.log` groups provider
  invocation counts by adapter, includes token totals only when reported, and
  lists each completed agent invocation in its adapter group in call order.

## Agent generation

- `--newAgent` / `--new-agent` is standalone and is not exposed to models.
- Ask separately for the created agent's adapter/model and the generator's
  adapter/model. Validate the complete generated definition before writing.
- Reject changed metadata, invalid definitions, failed processes, and
  overwrites. Retain model lowercase normalization.
