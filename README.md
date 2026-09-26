# new-harness

Run coding-agent workflows defined in YAML, with agent instructions in Markdown
and an editable history for resumption. Supports the Codex, OpenCode, Ollama,
Ollama Web, and OpenRouter adapters; Claude Code is not implemented.

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
and system libraries. Install and authenticate the selected CLI where you run
the binary. The `openrouter` adapter uses `curl`, an `OPENROUTER_API_KEY`, and
an explicit model slug such as `~openai/gpt-sol-latest`; configure the key in the
execution directory's `.env` file or environment. `OPENROUTER_URL` may override
its API endpoint.
Bash is required; TREE also requires Git and a working tree.

Enable the repository's documentation pre-commit hook once per clone:

```bash
git config core.hooksPath .githooks
```

The hook runs `./new-harness run documentation` before every commit.
It requires the root executable and an authenticated Codex installation, and a
workflow failure cancels the commit. Generated documentation remains available
in the working tree for a subsequent commit.

## First workflow

Create `.agents/planner.md`:

```markdown
---
adapter: codex
---

Produce a concise implementation plan for the user's request.
```

Create `flows/workflow.yml`:

```yaml
version: 1
steps:
  - agent: planner
    input: "Plan a command-line task tracker."
    output: plan
```

Run from the project directory containing `.agents/` and `flows/`:

```bash
./new-harness run workflow
./new-harness resume history/run-<id>.log
./new-harness --new-agent
```

`--new-agent` (also `--newAgent`) generates an agent interactively without
replacing existing files. It asks separately for the created agent's and the
generator's adapter/model. Optional agent metadata: `model` and an initial `ask`
question. Omit `model` to use the harness default.

Run `./new-harness --help` to show the available commands. Flow names resolve
to `flows/<flow-name>.yml`.
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
| GIT-STATUS-TREE | List changed, untracked, and deleted paths as JSON, filtered by `.treeignore`. |
| READ | Read UTF-8 text; `.readignore` blocks matching paths with `AccessDenied`; `enumerate: true` adds line numbers and `version-output` supports EDIT. |
| WRITE | Create files and parent directories; `force` replaces, `skip` preserves existing files. |
| DELETE | Delete one regular file; confirmation is required unless `force: true` is set. |
| EDIT | Insert, delete, replace, prepend, or append text; line edits require a READ version. |
| ASK | Ask the user for a nonempty answer; `input` is the question and `output` can pass on the answer. |
| AWAIT | Pause and wait for the user to press Enter before continuing. |
| CUSTOM-TOOL | Run `tools/<name>.sh` with a string array of arguments via `custom-tool: name`. |
| LOOP | Run `iter` steps for each string in an array. |
| IF | Select `is_true` or `is_false` steps from a strict boolean condition. |

Agents run read-only and can request `READ: <path>` or `ASK: <question>`. An
agent may request `TREE` only with explicit `TREE_TOOL: allow` metadata.
An agent with explicit `DELETE_TOOL: allow` metadata may also request deletion.
Other tools are workflow-only. File tools operate inside the execution directory.
Agent definitions and history also resolve there, regardless of the YAML's path.
When answering an `ASK`, the user may instead enter a standalone `READ: <path>`
or `TREE`; its read-only result is returned to the agent before it continues.

Use `.readignore` to prevent READ from exposing sensitive files. It accepts one
relative pattern per line; blank lines and `#` comments are ignored. A pattern
such as `.env` blocks that filename at any depth, while `secrets/` blocks a
directory and `*.pem` blocks matching filenames. A blocked request returns
`AccessDenied`.

Each run saves `history/run-<id>.log`. Resume uses its saved directory and
configuration, reuses completed results, and retries pending work. To regenerate
an agent response, stop the run, remove its `<== OUTPUT` block and everything
after it, then resume. Consult the recovery reference before editing tool or
branch records. Never edit a running transcript.

## Documentation

- [Workflow reference](REFERENCE.md): full YAML examples, tool options,
  agent generation, history editing, and limitations.
- [Documentation index](docs/TREE.md): trusted map of source files and their
  module documentation; consult individual documents as needed.
- [Development instructions](AGENTS.md): contributor workflow and checks.

Execution is sequential, without automatic retries or timeouts. JSON harness
streams (`json: true`) are not supported.
