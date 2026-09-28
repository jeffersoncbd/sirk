# S.I.R.K. development contracts

Compatibility requirements for the agent service and SDK transports. Contributor
workflow and checks are in [AGENTS.md](AGENTS.md).

## Language, files, and permissions

- Tool-owned code, comments, messages, tests, logs, and documentation are
  English. Preserve user-authored instructions and responses in their original
  language.
- Resolve agent definitions and history from the execution directory. Agent
  input is literal text; do not expand `{{ ... }}` or other template syntax.
- Normalize model names when loading Markdown metadata. Child stdin is closed.
- Keep Codex read-only with `approval_policy="never"` and
  `--skip-git-repo-check`. Do not add automatic write approvals or `write`
  agent metadata. Only explicit agent permissions enable external edits or
  deletion.

## SDK HTTP transport

- `sirk http` binds to `127.0.0.1:8080` by default. An explicit address may
  override the bind; external exposure does not imply authentication or TLS.
- HTTP exposes `GET /health`, `POST /v1/agent/run`, `POST /v1/git/status`, and
  `POST /v1/git/add`. Agent requests contain the server-visible execution
  directory, agent ID, and literal input. Git requests contain only the
  server-visible execution directory.
- The Rust SDK connects to HTTP without starting a CLI process. Its default
  endpoint is `http://127.0.0.1:8080`; `Sirk::connect()` sends the caller's
  current directory. Custom endpoints preserve supplied server-visible paths.
- Git status returns sorted, unique UTF-8 paths for modified, untracked, and
  deleted regular files, excluding `.treeignore` matches. Git add stages all
  changes below the supplied directory. These SDK endpoints are not agent tools.

## Agent tools

- TREE returns a pretty-printed JSON array of relative paths and `[]` for no
  files. Git ignores apply to untracked files; `.treeignore` independently
  removes matching paths, including tracked paths. Preserve path bytes during
  listing and reject non-UTF-8 paths at JSON rendering.
- READ returns exact UTF-8 contents, including empty text. Agents with
  `EDIT_TOOL: allow` receive line-numbered READ results. READ only accepts
  regular files inside the execution directory; `.readignore` controls access.
- Recognize ordinary requests only as standalone `TREE` from agents with
  `TREE_TOOL: allow`, or one-line `READ: <path>` responses.
- EDIT and DELETE require their explicit agent permissions. Persist an EDIT
  preparation before mutation, protect against intervening file changes, and
  return plain diffs without ANSI formatting. DELETE removes one regular file
  inside the execution directory; a noninteractive request requires both
  `DELETE_TOOL: allow` and `DELETE_WITHOUT_CONFIRM: allow` with `force: true`.
- Do not expose host-side SDK filesystem or Git helpers as agent tools.

## Conversations and history

- Reconstruct explicit conversation context for each provider turn; do not
  silently adopt provider session resumption.
- Save input before invoking the provider and complete tool results after
  execution. Use atomic replacement, file and directory synchronization, and
  exclusive locks for logs.
- Failed calls leave a diagnostic pending record. Empty agent responses fail;
  empty READ results are valid. Logs are not resumed through the CLI.

## Agent generation

- `--newAgent` / `--new-agent` is standalone and is not exposed to models.
- Ask separately for the created agent's adapter/model and the generator's
  adapter/model. Validate the complete generated definition before writing.
- Reject changed metadata, invalid definitions, failed processes, and
  overwrites. Retain model lowercase normalization.
