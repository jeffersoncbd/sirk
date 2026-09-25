# Workflow reference

Detailed usage for agents, YAML tools, scopes, and recovery. Start with the
[quick-start manual](../README.md); use the sections below as needed.

- [Agents](#agent-definitions) and [generation](#generate-an-agent-with-a-model)
- [Workflows and outputs](#workflows-and-outputs)
- [Tools](#tools): TREE, READ, WRITE, EDIT, CUSTOM-TOOL, IF, LOOP
- [User questions](#questions-to-the-user)
- [History and resumption](#history-editing-and-resumption)
- [Limitations](#current-limitations)

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
| `adapter` | Required; `codex` or `opencode`. |
| `model` | Optional; omitted uses the harness default. Trimmed and lowercased on load. |
| `ask` | Optional initial question, asked before the first model call. |
| `json` | Defaults to false. True is rejected until event normalization exists. |

The Markdown body contains the agent instructions and must not be empty.
Unknown fields, including `write`, are rejected. Inline agent definitions in
workflow YAML are not supported. `GPT-6-Astra`, for example, is normalized to
`gpt-6-astra`; the CLI does not discover or verify available models for you.

Codex agents execute with a read-only sandbox and no approval escalation.
OpenCode is invoked without its dangerous `--auto` permission option, but its
CLI does not expose an equivalent read-only sandbox flag; configure its
permissions appropriately in the OpenCode environment. The orchestrator itself
writes transcripts and newly generated agent definitions. Harnesses may also
maintain their own session files and logs.

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

Each step specifies exactly one of `agent`, `tool`, or `custom-tool`. Steps run
in order. All referenced agents, including those inside loops, are validated
before execution. Unknown YAML fields and references to unavailable outputs are
rejected.

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

Workflow READ steps may additionally set `version-output: revision`. This assigns
the SHA-256 digest of the exact returned content to a separate output, for use in
EDIT's `version`. It does not change READ's returned text or add line numbers.
The version output follows the same global/loop scope rules as `output` and is
reconstructed from the saved READ result on resume. Use different names for
`output` and `version-output`.

### WRITE

Creates a UTF-8 file inside the execution directory. It is workflow-only and is
not advertised to agents:

```yaml
- tool: WRITE
  path: docs/overview.md
  input: |
    # Overview
    Generated documentation.
  output: written
```

`path` is required, may use output references, and any missing parent directories
inside the execution directory are created automatically. `input` is the exact
text written, including empty text. The default is safe creation: WRITE fails
when the target already exists. Set `force: true` to replace the existing regular
file:

```yaml
- tool: WRITE
  path: docs/overview.md
  input: "Replacement content"
  force: true
```

WRITE rejects paths outside the execution directory, directories, and an existing
target that is a symlink.
Set `skip: true` to leave an existing regular file unchanged and complete the
step successfully. Missing files are still created, including parent directories.
`skip` defaults to false and is only accepted on WRITE. Combining `skip: true`
with `force: true` is rejected; `force: false` with `skip: true` is valid.

```yaml
- tool: WRITE
  path: docs/overview.md
  input: "Initial documentation"
  skip: true
```

Skipping completes only this WRITE step: later workflow steps still execute,
and earlier steps (including model calls producing the input) have already run.
A skipped step is recorded as successful and is not retried on resume, even if
the file is subsequently removed.

It has an empty result, so `output` is normally omitted unless a later template
needs an explicit empty value. A failed WRITE stays pending in the transcript for
resume.

### EDIT

Edits a UTF-8 regular file inside the execution directory. EDIT is
workflow-only and is not advertised to agents. It returns a unified diff, which
can be captured with `output`. The terminal displays removals on red backgrounds
and additions on green backgrounds. Redirected output and history have no added
ANSI formatting; set `NO_COLOR` to disable colors in a terminal too.

```yaml
version: 1
steps:
  - tool: READ
    input: src/main.rs
    output: source
    version-output: revision

  - tool: EDIT
    path: src/main.rs
    operation: replace
    start: 10
    end: 15
    version: "{{ outputs.revision }}"
    input: |
      fn greeting() {
          println!("Hello");
      }
    output: changes
```

| Operation | Coordinates | Behavior |
| --- | --- | --- |
| `insert` | `line` | Insert before the specified line. |
| `delete` | `start`, `end` | Remove the inclusive range; omit `input` or use empty text. |
| `replace` | `start`, `end` | Replace the inclusive range with `input`. |
| `prepend` | None | Insert exact `input` at the start. |
| `append` | None | Insert exact `input` at the end. |

Lines start at 1. Ranges must be nonempty and inside the file. Inserting at
`N + 1` appends to a file with `N` lines; an empty file accepts `line: 1`.
A final newline terminates the last line, rather than creating another empty
line. Line boundaries use LF; existing CRLF bytes are preserved outside edits.
The specified lines include their line terminators when deleted or replaced.

`input` is inserted exactly, without automatic separators or newline conversion.
For example, appending `next\n` to a file containing `last` produces
`lastnext\n`. YAML `|` includes a final newline; `|-` omits it. `path`, `input`,
`version`, `line`, `start`, and `end` support templates, including `loop.*`
inside loops. Inserted values are not recursively expanded. Rendered coordinates
may contain surrounding whitespace but must otherwise be positive integers.
Numeric YAML coordinates remain supported.

Line-based operations require `version`, a SHA-256 hex digest from a previous
READ of the file. If the current content differs, EDIT fails without changing
the file. READ the file again before a subsequent coordinate-based edit;
coordinates always refer to that read's version. This release applies one
operation per step, not batches of edits.

Append and prepend may omit `version`, or provide one for the same version check:

```yaml
- tool: EDIT
  path: logs/execution.log
  operation: append
  input: |
    Execution completed.
```

Append and prepend create the target when it is absent, treating its initial
content as empty. Its parent directory must already exist; use WRITE when you
also need to create directories. Even an empty input creates an empty file.
Insert, delete and replace still require an existing target. EDIT rejects
directories, final-component symlinks, non-UTF-8 files, and resolved paths outside
the execution directory. It does not accept `force`. The replacement uses a
temporary file in the target directory, file/directory synchronization, and an
atomic rename for existing targets, preserving file permissions. New targets
are published atomically with a hard link from the temporary file, without
overwriting a file created in the meantime. This requires filesystem hard-link
support; new files use default permissions filtered by the process umask.
Replacement changes the file's inode;
ownership, extended attributes and hard-link relationships are not preserved.

Before writing, EDIT saves its resolved request, original content and whether
the file was absent as JSON in
the existing `==> INPUT` block. The replacement is reconstructed from that record.
On resume, a pending edit applies only if the file still matches the original;
if it already matches the replacement, the step completes without applying it
again. Any other content produces a conflict. The successful `<== OUTPUT` stores
the plain diff; completed edits are not re-executed or checked against later
file changes. Prepared records and their diffs are validated before pending work.
An absent target is distinct from an existing empty file: a pending creation
requires absence or the exact expected result, while a pending update cannot
recreate a file deleted since preparation. Older prepared records default to
an existing target. Creation diffs use `/dev/null` as the old file name.

The saved request is authoritative during recovery. To change a prepared edit,
remove its entire step record and dependent later records before resuming with
the edited snapshot. A no-op edit returns an empty diff. Prepared original
contents increase transcript size. Version checks detect stale reads and a
second check detects ordinary intervening writes, but do not lock out unrelated
writers: do not run concurrent editors against a target during EDIT.

### CUSTOM-TOOL

CUSTOM-TOOL runs a Bash script from the execution directory's `tools/` folder.
The `custom-tool` value is the script name without `.sh`, using ASCII letters,
digits, underscores, or hyphens. For example, `custom-tool: doc-name` runs
`tools/doc-name.sh`:

```yaml
- custom-tool: doc-name
  input:
    - "{{ loop.item }}"
  output: doc_name
```

Input must be a YAML list of strings. Each rendered item is passed as a distinct
positional argument in order: the first item is `$1`, the second is `$2`, and so
on. Arguments are passed literally, including spaces and shell metacharacters.
An empty list runs the script without positional arguments.

Scripts run with Bash from the execution directory and do not need the executable
permission bit. Their exact UTF-8 stdout becomes the step result and can be saved
with `output`. Stderr remains visible. A missing script, invalid UTF-8 stdout, or
nonzero exit stops the workflow and leaves the invocation pending for resume.
CUSTOM-TOOL is YAML-only and is not advertised to agents.

### IF

IF is workflow-only and executes one branch based on its input:

```yaml
- custom-tool: file-exists
  input: ["{{ loop.doc_name }}"]
  output: file_exists
- tool: IF
  input: "{{ loop.file_exists }}"
  is_true:
    - tool: READ
      input: "{{ loop.doc_name }}"
      output: explain
  is_false:
    - agent: code-explainer
      input: "{{ loop.content }}"
      output: explain
    - tool: WRITE
      path: "{{ loop.doc_name }}"
      input: "{{ loop.explain }}"
```

This example belongs inside a LOOP. Subsequent steps can read `loop.explain`
because both branches assign it. The bundled `tools/file-exists.sh` prints
`true` for a regular file (including a symlink to a regular file) and `false`
otherwise. READ still enforces its own path restrictions. Missing or empty
script arguments fail rather than producing a condition.

`input` accepts YAML booleans (`true`, `false`) or text resolving to those exact
lowercase words. Surrounding whitespace, including a script's trailing newline,
is ignored when interpreting the condition. Empty values, arrays, numbers,
`yes`, `no`, `0`, and `1` are rejected; there is no implicit truthiness or shell
expression evaluation. Boolean input is only supported on IF.

`is_true` and `is_false` are lists of steps. Either may be omitted or empty,
but at least one must be nonempty. Selecting an absent/empty branch does nothing.
Both branches are validated recursively, including every referenced agent,
before execution. Only the selected branch executes. Branches may contain
nested IF, LOOP, tools, and agents. IF does not accept `iter` or `output`.

Branches share the enclosing scope: outside loops, their outputs are global;
inside loops, they assign to the current iteration's locals. After IF, templates
may reference names already available before it or names assigned in both
branches. A new name assigned in only one branch cannot be referenced afterward.
The two branches may declare the same new global output, but possible duplicate
global assignments in sequential steps are rejected. Loop locals retain their
normal reassignment rules, and `loop.item` remains read-only. IF introduces no
new loop scope and has no aggregate result.

The resolved condition is saved once in IF's `==> INPUT` block. Resume reuses
that decision and completed child results instead of reevaluating the condition.
Child labels include the branch, for example `Step 2.true.1` or
`Step 1.3.2.false.1` for a branch nested in a loop iteration. Changing the saved
condition requires truncating its dependent child records and subsequent steps;
records from the opposite branch or after pending work are rejected.

### LOOP

LOOP is **workflow-only** and accepts an array of strings. It executes the
steps under `iter` sequentially for each item. Agents cannot invoke LOOP.

A workflow that explains each listed file can use this pattern:

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

- Codex and OpenCode are implemented; there is no parallel execution or
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
