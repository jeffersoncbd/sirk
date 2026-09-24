# new-harness

A programmable orchestrator for coding-agent harnesses.

`new-harness` is a temporary project name. The goal is to let users define
agent conversations and execution order in YAML, keep agent instructions in
readable Markdown files, and follow the conversation in the terminal and logs.
The application invokes existing harness CLIs through Bash instead of calling
model APIs directly.

This README is also the project handoff for future development sessions.
Read [Development and session handoff](#development-and-session-handoff)
before changing the implementation.

Each adapter translates a shared request into the provider's CLI invocation.
The first adapter is `codex`; OpenCode and Claude Code can implement the same
contract. External commands run through the reusable `BashService`.

All tool-owned code, messages, logs, and documentation are in English.
User-authored agent instructions, inputs, and agent responses retain their
original language and are never translated by the orchestrator.

## Current scope

The MVP runs sequential steps with the Codex adapter. Each conversation turn
starts a new harness process with the explicit conversation reconstructed in
its prompt. Steps remain independent; previous responses reach subsequent
agents only through explicit output references. The current example is
intentionally a simple conversation: one agent introduces itself and another
greets it.

Implemented: Markdown agent loading, early validation, optional step inputs,
named outputs, Bash execution with streamed output, an interactive command
loop, initial questions, clarification conversations, and editable logs used
as durable execution state. OpenCode and Claude Code support are intended
extensions, not implemented features.

## Build and run

Build Rust code inside the VS Code Dev Container. Do not install Rust on the
host for this project. The repository is bind-mounted at
`/workspaces/new-harness`; the configured development user is `vscode`.

From the VS Code terminal attached to the Dev Container:

```bash
cd /workspaces/new-harness
cargo build
```

The harness CLI must be installed, authenticated, and available in the
environment where you **run** the binary. In the development setup used so
far, Codex runs on the host and Rust runs in the container. Compiling inside
the container does not make the host's Codex installation available there.

From the project root on the host:

```bash
cp target/debug/new-harness ./new-harness
./new-harness run example.yml
```

You can also use `./target/debug/new-harness` directly. Copying is convenient
but creates a separate binary: repeat the copy after each build. Running the
container-built binary on the host requires compatible architecture and
system libraries; this workflow has been used on the current Linux host.

If Codex is configured inside the container, you can instead run
`cargo run -- run example.yml` there. There is no automatic installation or
authentication of harness CLIs.

## Usage

```bash
new-harness
new-harness run example.yml
new-harness resume history/run-<id>.log
```

These commands assume the binary is on `PATH`. Use `./new-harness` when
running the copy in the project root.

The interactive session accepts:

```text
new-harness> run example.yml
new-harness> example.yml
new-harness> resume history/run-<id>.log
new-harness> /help
new-harness> /quit
```

`/exit` and EOF also close the session. A failed workflow returns to the
interactive prompt. A failed non-interactive invocation exits with code 2;
successful execution exits with code 0.

Without arguments, the CLI opens an interactive workflow command session.
Agent stdout is captured and streamed to the terminal; stderr is inherited.
Child stdin is closed so the orchestrator owns user input. Agents run in the
directory where you started the CLI, even if the workflow file is elsewhere.
Resumption uses the working directory saved in the transcript.

## Workflow YAML

```yaml
version: 1
steps:
  - agent: to-introduce-oneself
    output: intro
  - agent: greet
    input: "{{ outputs.intro }}"
```

Agent definitions live in `.agents/<name>.md` in the working directory.
The filename without its extension identifies the agent. For example,
`.agents/greet.md`:

```markdown
---
adapter: codex
model: gpt-5.6-luna
---

You must greet everyone who introduces themselves.
```

The YAML front matter accepts `adapter` (required), `model`, `ask`, and `json`
(default: false; true is rejected for resumable conversations). The `write`
option is not accepted. The Markdown body holds
the agent instructions. All referenced agents are loaded and validated before
any step executes. Missing or invalid files stop the workflow immediately,
with the file path in the error message. Agent names allow ASCII letters,
digits, underscores, and hyphens. Inline `agents:` definitions in workflows
are no longer supported.

Steps execute in order. `output` names an agent's stdout so later steps can
reference it using `{{ outputs.name }}`. The `input` field is optional in
every step; when omitted or empty, only the Markdown instructions are sent.
Omit `input` or use `input: ""`; YAML `null` is not accepted for this string
field. Omitting `output` still displays and logs the response, but does not
make it available to later steps by name. Unknown YAML fields are rejected.

Agents exchange text inputs and outputs. The Codex adapter explicitly sets
`--sandbox read-only` and `approval_policy="never"` to prevent file writes
by agent commands and permission escalation. Codex may still maintain its own
session files and logs. The adapter also adds `--skip-git-repo-check` to allow
workflows outside Git repositories.

Workflow structure, adapters, and output references are validated before the
first agent runs. Output names must be unique and contain only ASCII letters,
digits, underscores, or hyphens. References must point to earlier steps.
`{{outputs.name}}` is also accepted. Inserted content is never reinterpreted
as a template.

`json: true` is rejected before execution until adapters support event
normalization. Raw events cannot serve as conversation messages.

Failures stop subsequent steps. Outputs remain in memory during execution and
are recorded in history. Output size limits, timeouts, and process-tree
cancellation are not implemented yet.

## Questions and conversation state

An agent can define `ask: "What shall we plan?"` in its Markdown front matter.
The orchestrator asks this question before invoking the agent, even when the
step also has workflow input. Both inputs are included in the context.
The agent can return `ASK: Your question` as its complete response to request
clarification. Only a prefix at the start of the response (after whitespace)
is recognized. After the user answers, another harness process receives the
instructions and all previous messages for that step. Only a final response
becomes a named output and allows subsequent steps to run.

Run the included planner example with `./new-harness run planner.yml`.

The same `UserInput` interface handles initial questions and clarifications.
Terminal answers are single lines; empty answers are retried. `/cancel` or EOF
stops execution with its current state saved. Resume the log to continue.
Each occurrence of an agent in a workflow starts an independent conversation.

## History and resumption

Each run creates a unique `history/run-<timestamp>-<pid>.log` file and prints
its path. This editable transcript is the execution state; there is no separate
state document. A YAML header stores the working directory, complete workflow,
and agent instructions/configuration once. Resumption uses these saved values,
not potentially changed workflow or agent files.

The conversation body contains only step headers and message blocks:

```text
============================================================
Step 1 — planner

==> ASK
What shall we plan?

==> INPUT
A scheduling application.

<== OUTPUT
ASK: For which business?

==> INPUT
A dental practice.
```

Each marker occupies its own line. Content extends to the next marker or EOF.
One blank line separates blocks. Literal marker lines, separator lines, lines
starting with `Step `, and lines starting with a backslash are escaped with a
leading backslash; the reader removes that escape. Responses are otherwise
preserved, not summarized. Turn numbers and repeated agent labels are omitted.
The input block holds the new message; the full prompt is reconstructed from
the saved instructions, workflow input when applicable, and conversation.

Run `new-harness resume history/run-<id>.log` to continue in the same file:

- A pending initial question or `ASK:` response waits for user input.
- An input without an output invokes the agent with the preceding context.
- A final output advances to the next step, restoring named outputs from the log.
- A fully completed transcript returns without invoking any agents.

To regenerate a response, stop the CLI, delete that `<== OUTPUT` marker and
everything after it, save, and resume. The retained input is sent again.
Leaving later turns or steps after a deleted output is rejected before any
execution. Do not edit the transcript while it is running. Changing earlier
content requires removing the subsequent results yourself; arbitrary semantic
changes cannot be automatically detected.

Inputs are saved before model calls. Complete successful responses are saved
before continuing, using a synced temporary file and atomic replacement.
Failed, empty, or interrupted responses are not committed; partial output and
errors remain visible in the terminal. If the process dies after a model call
but before saving its response, resumption repeats that call.
A `.log.lock` file coordinates exclusive access and contains no conversation
state; OS locks release on process exit. A leftover `.log.tmp` is ignored on
resume. Keep the original `.log` as the authoritative file.

Old logs remain untouched and cannot be resumed with this version. Validation
of the workflow and all agents happens before a new history file is created.
The orchestrator writes history even though agents run in read-only mode.

## Architecture

- `workflow::Workflow`: declarative format, validation, and output substitution.
- `agents::Agent`: Markdown agent definitions loaded from `.agents`.
- `runner`: reusable sequential orchestration, independent of the terminal session.
- `history`: editable transcripts, configuration snapshots, atomic storage, and parsing.
- `input::UserInput`: reusable terminal/test interaction boundary.
- `harness::RunRequest`: provider-neutral agent request.
- `harness::HarnessAdapter`: provider integration contract.
- `adapters::CodexAdapter`: translation to `codex exec`.
- `services::BashService`: executes `bash -lc`, streams and captures stdout,
  and quotes arguments to prevent prompts from being interpreted as shell code.
- `services::Invocation`: generic command description independent of harnesses.

The execution path is:

```text
main → Workflow::from_file → runner
                              ├─ load .agents/<name>.md and validate all steps
                              ├─ resolve earlier outputs into each prompt
                              ├─ adapter → Invocation → BashService → harness CLI
                              └─ history: snapshot and durable conversation blocks
```

`src/main.rs` owns CLI parsing and the interactive loop. Keep orchestration
in `src/runner.rs`, provider arguments in `src/adapters/`, and process
execution in `src/services/bash.rs`. `runner::run_with` accepts an execution
callback for testing without real model calls; it still writes history.
`run_interactive_with` and `continue_with` also accept a user-input implementation.
`resume` parses and validates a transcript before reconstructing pending work.

## Development and session handoff

### Established decisions

- Keep tool-owned code, comments, messages, logs, tests, and documentation in
  English. Preserve the language and content of user prompts and responses.
- Start simple. The root `example.yml` demonstrates conversation, not code
  review or implementation. Preserve user changes to it and to `.agents/`.
- Agents exchange text and must not modify project files. Do not restore a
  `write` setting or automatic write approvals. The orchestrator itself writes
  history. Read-only sandboxing does not mean the harness has no tools.
- Keep agent definitions in `.agents/<name>.md`, separate from workflow YAML.
  `.agents/` contains product data, not development-agent instructions.
- Resolve both `.agents/` and `history/` from the working directory, not from
  the YAML file's parent. Check every referenced agent before invoking any one.
- Execute external commands through the reusable Bash service. Adapters build
  an executable plus arguments; they must not interpolate prompts into shell
  code. Keep provider-specific behavior outside the workflow schema.
- Logs are editable execution state, readable by humans. Keep the conversation
  body minimal: one step heading, `==> ASK`, `==> INPUT`, and `<== OUTPUT`.
  Store configuration once in the header; infer turn order and execution status.
  Preserve marker escaping, atomic writes, and validation before resumption.
- Opening the binary without arguments should retain a terminal session.
  The existing command loop is only the initial interface, not a full chat UI.

### Container commands

Container IDs change when VS Code rebuilds the environment. Discover the
current container rather than reusing an ID from an earlier session:

```bash
docker container ls
```

Replace `<container-id>` below with the active project's Dev Container ID.
**Always use `docker exec -u vscode`** to avoid root-owned build artifacts:

```bash
docker exec -u vscode -w /workspaces/new-harness <container-id> cargo fmt --check
docker exec -u vscode -w /workspaces/new-harness <container-id> cargo test
docker exec -u vscode -w /workspaces/new-harness <container-id> cargo clippy --all-targets -- -D warnings
docker exec -u vscode -w /workspaces/new-harness <container-id> cargo build
```

Use `cargo fmt` in the same environment when formatting is needed. Add `-it`
to Docker invocations when a real interactive terminal is needed.

`build.sh` currently removes the root binary, runs Cargo, and copies the debug
binary back. It expects Rust in its execution environment and does not stop
on build failure. Prefer the explicit build and copy commands above until
that script is improved.

### Starting a new development session

1. Read this README, the current source, `example.yml`, and the referenced
   agent files. Inspect existing changes before editing; the user actively
   adjusts agent definitions and examples.
2. Check the active Dev Container and runtime environment. Build and test as
   `vscode`; run real harness integrations where that CLI is authenticated.
3. Keep changes within the requested feature and preserve the decisions above.
   When adding an adapter, implement `HarnessAdapter`, register it in
   `src/adapters/mod.rs`, and verify read-only behavior and output semantics.
4. Run relevant tests and the checks above for code changes. Existing tests use
   temporary projects, fake harness callbacks, and real local Bash commands;
   they do not require authentication or call a model.
5. Distinguish automated checks from real harness validation in the handoff.
   A passing mocked workflow test is not a successful live Codex run.
6. Update this README when schemas, usage, behavior, or limitations change.
   If a root binary copy is being used, rebuild and remind the user to refresh it.

The conversation example has been run successfully by the user on the host,
with two independent Codex sessions and output substitution. Later changes
must still be validated on their own merits; that run is not a blanket
integration guarantee.

### Known limitations and possible next work

These are open areas, not automatically authorized tasks:

- Only Codex is implemented. No parallel steps, branching, workflow loops, or
  automatic retries exist. Conversations resume from explicit text history;
  hidden harness state and native provider sessions are not restored.
- The terminal prints raw harness output. A response may appear twice because
  progress on stderr and final output on stdout are both visible; only stdout
  is passed to the next step.
- Stdout is captured in memory and must be UTF-8. There is no size limit,
  timeout, log rotation, or coordinated cancellation of process trees.
- On output write failure, BashService kills and waits for its direct child;
  this is not complete descendant-process cleanup.
- Child stdin is closed. User answers are single-line terminal input; there is
  no multiline editor, PTY management, or concurrent interactive input handling.
- JSON event streams are not normalized into final responses, so `json: true`
  is rejected for these resumable conversations.
- History records requested configuration, not verified model identity or
  hidden harness instructions. Context grows with each clarification turn.
  Manual edits must remove dependent turns and steps. Old transcripts cannot
  be resumed, and the original workflow source path is not recorded.
- The YAML dependency is currently `serde_yaml 0.9`; Cargo reports that release
  as deprecated. A parser migration is a separate maintenance task.
- Harness CLI options can change. Check the installed CLI's help when changing
  an adapter; do not assume older permission flags remain appropriate.
