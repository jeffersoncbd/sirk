# new-harness

A programmable orchestrator for coding-agent harnesses.

Define workflows in YAML, keep agent instructions in Markdown, and follow the
conversation in the terminal and an editable transcript. The application
invokes existing harness CLIs through Bash; it does not call model APIs directly.
`new-harness` is a temporary project name.

Currently supported: Codex, sequential workflows, user questions, TREE, READ,
scoped LOOP iterations, model-generated agent definitions, and transcript-based
resumption. OpenCode and Claude Code are planned, not implemented.

Development instructions and architecture are in [AGENTS.md](AGENTS.md).
Agent definitions used by workflows are separate files in `.agents/`.

## Build and run

Build inside the supplied VS Code Dev Container, whose project directory is
`/workspaces/new-harness` and development user is `vscode`:

```bash
cd /workspaces/new-harness
cargo build
```

On the host, from the project root:

```bash
cp target/debug/new-harness ./new-harness
./new-harness run example.yml
```

Repeat the copy after rebuilding, or run `./target/debug/new-harness` directly.
Running a container-built binary on the host requires compatible architecture
and system libraries.

The harness CLI must be installed and authenticated where the binary runs.
In the current setup, Rust runs in the container and Codex runs on the host.
If Codex is configured inside the container, `cargo run -- run example.yml`
also works there. Harness installation and authentication are not automatic.
TREE additionally requires Git and a working tree.

## CLI usage

```bash
./new-harness
./new-harness run example.yml
./new-harness resume history/run-<id>.log
./new-harness --newAgent
./new-harness --help
```

Without arguments, the CLI opens a workflow command session:

```text
new-harness> run example.yml
new-harness> example.yml
new-harness> resume history/run-<id>.log
new-harness> /help
new-harness> /quit
```

`/exit` and EOF also close the session. Workflow failures return to the
interactive prompt. Non-interactive success exits with code 0; failure exits
with code 2.

The working directory determines where agents run and where `.agents/` and
`history/` are located, even if the workflow YAML is elsewhere. Resume uses
the directory saved in the transcript. Stdout is captured and streamed;
technical stderr remains visible. The orchestrator owns terminal input.
There is no history-path banner or workflow-success footer.

## Agent definitions

Create `.agents/<name>.md`, where the name uses ASCII letters, digits,
underscores, or hyphens. Example:

```markdown
---
adapter: codex
model: gpt-6-sol
ask: "What shall we plan?"
---

You are a software planner. Produce an actionable implementation plan.
If clarification is needed, respond only with ASK: followed by your question.
```

| Field | Behavior |
| --- | --- |
| `adapter` | Required; currently `codex`. |
| `model` | Optional; omitted uses the harness default. Trimmed and lowercased on load. |
| `ask` | Optional initial question, asked before the first model call. |
| `json` | Defaults to false. True is rejected until event normalization exists. |

The Markdown body contains the agent instructions and must not be empty.
Unknown fields, including `write`, are rejected. Inline agent definitions in
workflow YAML are not supported. `GPT-6-Astra`, for example, is normalized to
`gpt-6-astra`; the CLI does not discover or verify available models for you.

Agents execute with a read-only sandbox and no approval escalation. The
orchestrator itself writes transcripts and newly generated agent definitions.
Harnesses may also maintain their own session files and logs.

## Generate an agent with a model

Run `./new-harness --newAgent` (alias `--new-agent`). The wizard asks for:

1. A description of the desired agent.
2. The adapter and model the **created agent** will use.
3. The adapter and model that will **generate the definition**.
4. The agent name, without the `.md` extension.

For example, a generator using `gpt-6-astra` can create instructions for an
agent configured to run with `gpt-6-sol`. The generator receives your description
and the target configuration, then returns a complete Markdown definition.
Its instructions should use the language of your description.

The definition is validated before it is saved to `.agents/<name>.md`.
Failed generation, invalid or empty definitions, and changed target metadata
do not create an agent. Existing files are not overwritten. Invalid adapters
and names are retried; `/cancel` or EOF cancels the wizard. The generator must
be installed and authenticated in the execution environment.

This wizard is standalone, not a workflow tool or an agent-callable tool.
It does not create a resumable workflow transcript. Answers are single-line;
you can edit the generated Markdown afterward.

## Workflows and outputs

```yaml
version: 1
steps:
  - agent: to-introduce-oneself
    output: intro
  - agent: greet
    input: "{{ outputs.intro }}"
```

Each step specifies exactly one of `agent` or `tool`. Steps run in order.
All referenced agents, including those inside loops, are validated before
execution. Unknown YAML fields and references to unavailable outputs are rejected.

For agent steps, `input` is optional. Omit it or use `input: ""` when no extra
input is needed. `null` is not accepted. Tool input requirements depend on the
tool. Omitting `output` still displays and saves the result.

Outside a loop, `output: intro` makes the result available as
`{{ outputs.intro }}`. These output names must be unique and use ASCII letters,
digits, underscores, or hyphens. Inside a loop, outputs belong to that
iteration's `loop` scope. Both `{{ outputs.name }}` and `{{outputs.name}}`
are accepted. Inserted content is never interpreted again as a template.

Each agent step has its own conversation. Every turn starts a fresh harness
process with the explicit conversation reconstructed in its prompt. A final
answer completes the step; a tool request or clarification keeps it pending.
Failures stop subsequent steps.

## Tools

### TREE

Lists project files as a valid JSON array of sorted, unique relative paths,
one string per line, without a header:

```json
[
  "Cargo.toml",
  "README.md",
  "src/main.rs"
]
```

An empty listing returns `[]`. JSON escaping preserves quotes, backslashes,
and control characters; non-UTF-8 paths produce an explicit serialization error.

```yaml
- tool: TREE
  output: tree
```

TREE accepts no nonempty input. An agent can request it by responding with
exactly `TREE`, ignoring surrounding whitespace. Sentences, code fences,
`tree`, and `TREE: ...` do not invoke it. The result is returned to the same
agent, which continues its response.

TREE requires a Git working tree and follows Git's standard excludes:
nested `.gitignore` files, negation, `.git/info/exclude`, and configured global
excludes. Already tracked files remain visible despite matching `.gitignore`.

Use `.treeignore` to hide files from TREE while keeping them tracked by Git:

```gitignore
/.agents
/.devcontainer
Cargo.lock
new-harness
```

These additional rules use Git ignore syntax, including nested files, directory
patterns, wildcards, and `!` exceptions. They apply to tracked and untracked
files. A `.treeignore` exception cannot restore an untracked file excluded by
Git's standard rules. Git tracking and the index are not changed.

Git metadata, deleted files, empty directories, and submodule contents are
omitted. Symlinks are listed without traversal.

### READ

Returns only the UTF-8 content of a regular file, with no header or added
newline. An empty file returns empty text.

```yaml
- tool: READ
  input: src/main.rs
  output: source
```

Agents request it with a standalone, single-line response:

```text
READ: src/main.rs
```

Paths are literal, without shell expansion, surrounding quotes, or code fences.
Spaces are supported. Relative paths resolve from the execution directory;
absolute paths must remain inside it. Symlinks resolving outside it are rejected.
Ignore rules affect TREE listings, not explicit READ requests. Git is not
required. Workflow paths can reference earlier outputs.

### LOOP

LOOP is **workflow-only** and accepts an array of strings. It executes the
steps under `iter` sequentially for each item. Agents cannot invoke LOOP.

The current [example.yml](example.yml) follows this pattern:

```yaml
version: 1
steps:
  - tool: TREE
    output: tree
  - tool: LOOP
    input: "{{ outputs.tree }}"
    iter:
      - tool: READ
        input: "{{ loop.item }}"
        output: content
      - agent: code-explainer
        input: "{{ loop.content }}"
        output: explain
```

Input may be a JSON array returned by another step or a native YAML string
array such as `input: [a.txt, b.txt]`. Numbers, objects, nulls, mixed arrays,
and nested arrays are rejected. An empty array runs no iterations.
`iter` must be nonempty and can contain tools, agents, or nested loops.

Each iteration starts with its own read-only `loop.item`. Inside `iter`,
`output: content` assigns `loop.content`, available to following steps in
that iteration. Locals can be reassigned, but `loop.item` cannot be overwritten.
The older `output: "{{ loop.content }}"` spelling is accepted for compatibility.

Outer `outputs.*` remain readable. A nested loop gets a fresh `loop` scope and
restores the enclosing scope afterward. Iteration variables are released when
the iteration ends and do not become global outputs. LOOP has no aggregate
output. The transcript retains results for recovery.

## Questions to the user

An agent's `ask` question is shown before its first call, even when its step
also has an input. Both the supplied input and the user's answer enter the
conversation.

During execution, an agent can respond exclusively with:

```text
ASK: Which database should the application use?
```

The CLI asks the user, saves the answer, and calls the agent again with the
preceding conversation. TREE, READ, and clarification rounds can be combined
within a step. Only a final response is published as its named output.

Terminal answers are single-line; empty answers are retried. `/cancel` or EOF
stops the run with its current transcript available for resume.
Try `./new-harness run planner.yml` for the planner example.

## History, editing, and resumption

Each run silently creates `history/run-<timestamp>-<pid>.log`. This editable
transcript is the execution state; there is no separate state document.
Its header saves the working directory, workflow, and agent configurations.
Resume uses those saved values rather than reloading changed source files.

The body uses step headings and simple message markers:

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

`==> TREE` and `==> READ` contain results returned to an agent. Each marker
occupies a whole line; content continues until the next marker or EOF. One
blank line separates blocks. Literal marker lines, separator lines, lines
starting with `Step `, and leading backslashes are escaped with a backslash.

Loop headings identify the parent step, iteration, and body step:
`Step 2.3.1 — READ` is body step 1 in iteration 3 of workflow step 2.
The loop's resolved array is saved once and local scopes are reconstructed from
its saved child results when resuming.

```bash
./new-harness resume history/run-<id>.log
```

- A pending question waits for an answer.
- A pending agent input invokes the model.
- A pending tool request executes the tool.
- Saved tool results are reused, even if the underlying files changed.
- Completed steps and iterations are skipped.
- A completed workflow returns without rerunning anything.

To regenerate a response, stop the CLI, delete its `<== OUTPUT` marker and
everything after it, save, and resume. For an agent's tool result, delete its
`==> TREE` or `==> READ` block and everything afterward to rerun the tool.
After editing a loop array, remove the affected child history and subsequent
steps. Remaining records after a pending step are rejected; arbitrary semantic
edits cannot be detected automatically. Do not edit a running transcript.

Inputs are saved before calls; complete successful results are saved before
continuing. Failed or interrupted calls leave pending work to retry. Empty
agent responses fail, while empty READ results are valid. A crash after a call
but before saving its result can cause that call to repeat.

The `.log` file is authoritative. A companion `.log.lock` coordinates access
and contains no conversation state; leftover `.log.tmp` files are ignored.
Legacy logs without the v2 format header cannot be resumed. Existing v2
conversations remain supported.

## Current limitations

- Only Codex is implemented; there is no parallel execution, branching, or
  automatic retry policy.
- Conversation context grows with each turn. Native harness session state,
  hidden instructions, and verified model identity are not captured.
- There are no output-size limits, timeouts, log rotation, or complete
  process-tree cancellation.
- Terminal input is single-line; there is no multiline editor or PTY management.
- Raw harness stderr and stdout may display the same response twice; only
  stdout is used as the agent result.
- JSON harness event streams are not normalized, so `json: true` is rejected.
- Agent generation is a one-shot operation, not a resumable conversation.
