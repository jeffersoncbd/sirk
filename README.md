# new-harness

Run coding-agent workflows defined in YAML, with agent instructions in Markdown
and an editable history for resumption. Currently supports the Codex CLI;
OpenCode and Claude Code are not implemented.

## Setup

Build in the supplied VS Code Dev Container:

```bash
cd /workspaces/new-harness
cargo build
```

On the host, from the project root:

```bash
cp target/debug/new-harness ./new-harness
./new-harness --help
```

Repeat the copy after rebuilding. The host must support the binary's architecture
and system libraries. Install and authenticate Codex where you run the binary.
Bash is required; TREE also requires Git and a working tree.

## First workflow

Create `.agents/planner.md`:

```markdown
---
adapter: codex
---

Produce a concise implementation plan for the user's request.
```

Create `workflow.yml`:

```yaml
version: 1
steps:
  - agent: planner
    input: "Plan a command-line task tracker."
    output: plan
```

Run from the directory containing `.agents/`:

```bash
./new-harness run workflow.yml
./new-harness resume history/run-<id>.log
./new-harness --new-agent
```

`--new-agent` (also `--newAgent`) generates an agent interactively without
replacing existing files. It asks separately for the created agent's and the
generator's adapter/model. Optional agent metadata: `model` and an initial `ask`
question. Omit `model` to use the harness default.

Without arguments, `./new-harness` opens a command prompt accepting `run <file>`,
a YAML path, `resume <log>`, `/help`, and `/quit` (also `/exit` or EOF).
User answers are single-line; `/cancel` or EOF interrupts a workflow.
Non-interactive commands exit with 0 on success and 2 on failure.

## Workflow basics

Steps run sequentially. Each specifies `agent`, `tool`, or `custom-tool`.
Use `output: name` and `{{ outputs.name }}` to pass results to later steps.
Inside LOOP, use `{{ loop.item }}` and `{{ loop.name }}`; locals belong to one
iteration. Templates do not expand inserted content again.

| Tool | Purpose |
| --- | --- |
| TREE | List files as JSON, respecting Git ignores and `.treeignore`. |
| READ | Read exact UTF-8 text; optional `version-output` supports EDIT. |
| WRITE | Create files and parent directories; `force` replaces, `skip` preserves existing files. |
| EDIT | Insert, delete, replace, prepend, or append text; line edits require a READ version. |
| CUSTOM-TOOL | Run `tools/<name>.sh` with a string array of arguments via `custom-tool: name`. |
| LOOP | Run `iter` steps for each string in an array. |
| IF | Select `is_true` or `is_false` steps from a strict boolean condition. |

Agents run read-only and can request `TREE`, `READ: <path>`, or `ASK: <question>`.
Other tools are workflow-only. File tools operate inside the execution directory.
Agent definitions and history also resolve there, regardless of the YAML's path.

Each run saves `history/run-<id>.log`. Resume uses its saved directory and
configuration, reuses completed results, and retries pending work. To regenerate
an agent response, stop the run, remove its `<== OUTPUT` block and everything
after it, then resume. Consult the recovery reference before editing tool or
branch records. Never edit a running transcript.

## Documentation

- [Workflow reference](docs/REFERENCE.md): full YAML examples, tool options,
  agent generation, history editing, and limitations.
- [Documentation index](docs/TREE.md): trusted map of source files and their
  module documentation; consult individual documents as needed.
- [Development instructions](AGENTS.md): contributor workflow and checks.

Execution is sequential, without automatic retries or timeouts. JSON harness
streams (`json: true`) are not supported.
