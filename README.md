# S.I.R.K.

**S.I.R.K.** honors Alexander Shchukin, Dmitri Ivanov, Liliya Rodina, and Georgi
Kreier, researchers at the Vavilov Institute who died of starvation during the
Siege of Leningrad while protecting its collection of crop seeds. They chose to
preserve seeds that held promise for reducing hunger rather than consume them to
survive. Their work helped safeguard agricultural diversity that continues to
support crop research and food supplies around the world. Their legacy lives on
in the effort to feed billions of people worldwide.
[Read the account at the Science History Institute](https://www.sciencehistory.org/stories/magazine/the-tragedy-of-the-worlds-first-seed-bank/).

Run coding agents behind language-native workflows, with agent instructions in
Markdown and an editable history. Legacy YAML workflows remain supported during
the SDK migration. Supports the Codex, OpenCode, Ollama, Ollama Web, and
OpenRouter adapters; Claude Code is not implemented.

## Setup

Build from the project root in the supplied VS Code Dev Container:

```bash
cargo build
```

On the host, from the project root:

```bash
cp target/debug/sirk ./sirk
./sirk --help
```

Repeat the copy after rebuilding. The host must support the binary's architecture
and system libraries. Install and authenticate the selected CLI where you run
the binary. The `openrouter` adapter uses `curl`, an `OPENROUTER_API_KEY`, and
an explicit model slug such as `~openai/gpt-sol-latest`; configure the key in the
execution directory's `.env` file or environment. `OPENROUTER_URL` may override
its API endpoint.
Bash is required; TREE also requires Git and a working tree.

## Language SDK protocol

Language SDKs start `sirk rpc` in the project directory and exchange one
JSON-RPC 2.0 message per line over stdin/stdout. The public protocol currently
exposes only `agent.run`; workflow control, filesystem access, Git, and user
interaction belong to the host language. Agent permissions still control the
CLI's internal READ, TREE, EDIT, and DELETE operations.

The standalone [Rust SDK](sdk/rust/README.md) has no source or package dependency
on the CLI:

```rust
use sirk_sdk::Sirk;

let mut sirk = Sirk::start("./sirk", ".")?;
let explanation = sirk.agent("code-explainer", "Explain this module")?;
```

Enable the repository's documentation pre-commit hook once per clone:

```bash
git config core.hooksPath .githooks
```

The hook runs `./sirk run documentation` before every commit.
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
./sirk run workflow
./sirk resume history/run-<id>.log
./sirk --new-agent
```

`--new-agent` (also `--newAgent`) generates an agent interactively without
replacing existing files. It asks separately for the created agent's and the
generator's adapter/model. Optional agent metadata: `model` and an initial `ask`
question. Omit `model` to use the harness default.

Run `./sirk --help` to show the available commands. Flow names resolve
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
