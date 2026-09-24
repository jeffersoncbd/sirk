# Development instructions

This file guides Codex and other coding assistants working on this repository.
Read [README.md](README.md) for the product, schemas, examples, and user commands.
Keep development guidance here and user-facing documentation in the README.

These instructions apply to development tasks. When invoked as a workflow agent
or definition generator, perform that assigned task; do not start repository
maintenance or build checks unless requested. Files in `.agents/` are product
data loaded by the orchestrator, not development instructions.

## Start a development session

- Inspect the current source and existing changes before editing. Use
  `git status --short` and relevant diffs when Git metadata is available.
- Read `example.yml` and agent files relevant to the requested change. The user
  actively edits examples, `.agents/`, and ignore rules; preserve those edits.
- Keep work within the requested feature. Known limitations are not a backlog
  automatically authorized for implementation.
- Do not assume a previous successful integration run validates later changes.

## Environment and checks

Build and test Rust inside the VS Code Dev Container. Do not install Rust on
the host. The project is mounted at `/workspaces/new-harness`.

Discover the active container each session; IDs change:

```bash
docker container ls
```

Always use `docker exec -u vscode` to avoid root-owned artifacts. For code
changes, run these checks with the active project container:

```bash
docker exec -u vscode -w /workspaces/new-harness <container-id> cargo fmt --check
docker exec -u vscode -w /workspaces/new-harness <container-id> cargo test
docker exec -u vscode -w /workspaces/new-harness <container-id> cargo clippy --all-targets -- -D warnings
docker exec -u vscode -w /workspaces/new-harness <container-id> cargo build
```

Use `cargo fmt` in the same environment when formatting is needed.
Documentation-only changes need document and link checks, not a Rust rebuild.

The current runtime setup uses authenticated Codex on the host. Container
compilation does not make that CLI available inside the container. Run live
integrations only where the chosen harness is installed and authenticated.
Tests use fake harness callbacks, temporary projects, and real local Bash/Git
commands; they should not require model access.

When refreshing the root binary after a successful build, run on the host:

```bash
cp target/debug/new-harness ./new-harness
```

The root executable is a separate copy. Update it when delivering code changes
for the user to run, or clearly say it still needs refreshing. Host execution
requires compatible architecture and system libraries.

Avoid `build.sh` until fixed: it removes the root binary, assumes Cargo is
available in its environment, and does not stop on build failure.

## Architecture and boundaries

| Module | Responsibility |
| --- | --- |
| `src/main.rs` | CLI argument handling, command loop, standalone agent-creation entry point. |
| `src/workflow.rs` | YAML schema, recursive validation, references, loop input and scope rules. |
| `src/agents.rs` | Markdown front matter and instructions; model normalization, including snapshots. |
| `src/runner.rs` | Sequential conversations, recursive LOOP execution, scope lifetime, resume validation. |
| `src/history.rs` | Snapshot, transcript blocks and hierarchical labels, parsing, locks, atomic saves. |
| `src/input.rs` | Shared `UserInput` interface and terminal implementation. |
| `src/harness.rs` | Provider-neutral `RunRequest` and `HarnessAdapter` contracts. |
| `src/adapters/` | Harness-specific CLI arguments and adapter registration. |
| `src/services/bash.rs` | Quoted Bash invocation, streamed/captured output, child lifecycle. |
| `src/tools/mod.rs` | Agent-callable tool recognition, shared dispatch, TREE JSON rendering. |
| `src/tools/tree.rs` | Git-backed listing and additional `.treeignore` filtering. |
| `src/tools/read.rs` | UTF-8 file reads within the execution directory. |
| `src/tools/custom.rs` | Project-local Bash script resolution, positional arguments, and stdout capture. |
| `src/tools/new_agent.rs` | Standalone model-generated agent creation and validation. |

Keep orchestration out of `main.rs`, provider behavior out of workflow schemas,
and external process execution in `BashService`. Build an executable and
argument list through `Invocation`; never interpolate prompts or paths as
executable shell code. Preserve binary capture for Git's NUL-delimited paths.

To add an adapter, implement `HarnessAdapter`, register it in
`src/adapters/mod.rs`, and verify permissions and output semantics.
Check the installed harness CLI's help before changing flags.

## Decisions to preserve

### Language, files, and permissions

- Tool-owned code, comments, messages, tests, logs, and documentation are English.
  Preserve user-authored instructions and responses in their original language.
- Resolve agent definitions and history from the execution working directory,
  not the workflow file's parent. Resume uses the saved directory.
- Validate every referenced agent, including nested loop bodies, before running
  any workflow step.
- Model names are trimmed and lowercased when loading Markdown and snapshots.
- Keep Codex read-only with `approval_policy="never"` and
  `--skip-git-repo-check`. Do not restore a `write` metadata option or automatic
  write approvals. The orchestrator may write history and generated definitions.
- Child stdin is closed; user interaction goes through `UserInput`.

### Tools and loops

- TREE must return a valid, pretty-printed JSON array with one path per line,
  no descriptive header, and `[]` for no files. Do not use Rust debug formatting
  as a substitute for JSON serialization.
- Use Git to evaluate standard ignores. Tracked files survive standard ignores.
  Evaluate `.treeignore` independently and subtract matches, including tracked
  files. Its negations must not revive files excluded by standard Git rules.
- Preserve path bytes in the low-level listing. JSON rendering rejects
  non-UTF-8 paths rather than silently changing them.
- READ returns exact UTF-8 contents, including empty text, without headers.
  It accepts only regular files whose resolved paths stay within the execution
  directory. TREE ignore rules are not READ access rules.
- CUSTOM-TOOL is YAML-only. Resolve `custom-tool: name` to `tools/name.sh`,
  require a string-array input, pass each item as one positional argument, and
  use exact UTF-8 stdout as the result. Keep script execution in `BashService`.
- Recognize agent requests only as standalone `TREE` or one-line
  `READ: <path>` responses. Use the same tool implementation for YAML steps.
- LOOP is YAML-only. Do not register it in the agent tool dispatcher or
  advertise it in model prompts.
- LOOP accepts string arrays, runs `iter` in order, and has no aggregate output.
  Each iteration gets fresh locals and read-only `loop.item`. Inside a loop,
  `output: content` assigns `loop.content`; outside it assigns
  `outputs.content`. Preserve compatibility with `{{ loop.content }}` output
  targets in older definitions.
- Nested loops restore the parent scope. Local assignments must not leak into
  another iteration or global outputs. Outer outputs remain readable.
- Inserted content is never recursively expanded as a template.

### Conversations and durable history

- Keep the transcript as the sole execution-state document. Save configuration
  once and reconstruct explicit context; do not silently adopt native provider
  session resumption.
- Keep body markers minimal: `==> ASK`, `==> INPUT`, `<== OUTPUT`,
  `==> TREE`, and `==> READ`. Preserve escaping, exact content, and hierarchical
  LOOP labels. Do not reintroduce per-turn numbering or repeated agent labels.
- Save input before invoking a tool/model. Save only complete successful results
  using atomic replacement, file/directory synchronization, and exclusive locks.
- Resume must validate all existing records before executing pending work.
  Reconstruct local scopes, reuse saved tool results, and reject later records
  after a pending step.
- Failed calls remain pending; empty agent responses fail, but empty READ
  results are valid. Preserve the possibility of retry after a call completed
  but its result was not saved.
- Preserve existing transcripts. Legacy non-v2 logs are not resumable; avoid
  breaking supported v2 logs when adding features.
- The CLI prints no history banner or success footer. The no-argument command
  loop remains available; it is not a general persistent chat UI.

### Agent generation

- `--newAgent` / `--new-agent` is standalone, not exposed to models or YAML.
- Ask separately for the created agent's adapter/model and the generator's
  adapter/model. Invoke the generator with its configuration and save the
  target agent's configuration.
- Generate the full definition from the description, validate it before
  writing, and keep canonical target metadata. Reject changed metadata,
  invalid definitions, and failed processes.
- Do not overwrite existing agents. Retain model lowercase normalization.

## Verification and handoff

- Use `runner::run_with`, `run_interactive_with`, `continue_with`, and
  `new_agent::create_with` to test without real model calls.
- Cover meaningful behavior: template substitution, scope isolation, pending
  iteration recovery, edited transcripts, exact file contents, Git ignore
  semantics, generator/runtime model separation, and non-overwrite behavior.
- Distinguish automated/fake-harness verification from a live Codex invocation.
  Do not claim real model success from a mocked test.
- Update README when user-visible behavior or usage changes; update this file
  when development practices or architecture change.
- Report changes, relevant checks, and limitations. Avoid stale container IDs,
  fixed test counts, or historical success claims as ongoing guarantees.

## Known implementation limitations

No parallel execution, branching, or automatic retries. Conversation prompts
and captured stdout grow in memory; there are no size limits, timeouts, or log
rotation. Process cleanup handles the direct child, not the complete descendant
tree. Terminal answers are single-line; no PTY management exists. JSON harness
events are not normalized, so `json: true` is rejected.

History records explicit requested configuration, not hidden harness
instructions, verified model identity, or the original workflow source path.
Arbitrary semantic edits require the user to truncate dependent history.

The YAML dependency is `serde_yaml 0.9`, which Cargo reports as deprecated.
Parser migration and the `build.sh` issue are separate maintenance work, not
implicit tasks.
