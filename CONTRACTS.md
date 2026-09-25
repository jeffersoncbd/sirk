# Development contracts

Compatibility requirements for changes to the corresponding subsystem. Read only
the relevant sections alongside its module documentation from [TREE.md](TREE.md).
Development workflow and checks are in [AGENTS.md](../AGENTS.md).

- [Language, files, and permissions](#language-files-and-permissions)
- [Tools and loops](#tools-and-loops)
- [Conversations and durable history](#conversations-and-durable-history)
- [Agent generation](#agent-generation)

## Language, files, and permissions

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

## Tools and loops

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
- WRITE is YAML-only and is never advertised to agents. It creates missing
  parent directories and writes text to a regular path inside the execution
  directory; existing files fail unless the workflow explicitly sets `force: true`.
  With `skip: true`, existing regular files complete without modification; missing
  files are created normally. Reject force and skip when both are true. Skipped
  writes have an empty successful result and must not run again on resume.
- EDIT is YAML-only and edits existing regular UTF-8 files. Operations are insert,
  delete, replace, prepend and append. Line coordinates are 1-based, ranges are
  inclusive, and inserted text is exact. Coordinate operations require a SHA-256
  version from READ's optional version-output; append/prepend may omit it.
- Append/prepend create absent targets in existing parent directories. Persist
  absence separately from empty content, publish creation without overwriting a
  concurrently created file, and never recreate a deleted prepared-update target.
- Keep READ's text result exact when emitting version-output. Its digest follows
  ordinary output scope rules and is reconstructed from saved text on resume.
- Persist EDIT's request and prepared original content in its INPUT block before
  mutation. Resume distinguishes original, already-applied and conflicting file
  contents. Validate prepared records and completed diffs before pending work.
  Keep ANSI presentation out of saved diffs and preserve existing v2 markers.
- CUSTOM-TOOL is YAML-only. Resolve `custom-tool: name` to `tools/name.sh`,
  require a string-array input, pass each item as one positional argument, and
  use exact UTF-8 stdout as the result. Keep script execution in `BashService`.
- Recognize agent requests only as standalone `TREE` or one-line
  `READ: <path>` responses. Use the same tool implementation for YAML steps.
- LOOP is YAML-only. Do not register it in the agent tool dispatcher or
  advertise it in model prompts.
- IF is YAML-only, with is_true/is_false step lists and a strict true/false
  condition (surrounding whitespace accepted). Validate both branches, including
  agents, before execution. IF has no aggregate output and shares its enclosing
  scope; only names available on both paths may be referenced afterward.
- Persist IF's resolved condition in INPUT and include true/false in child labels.
  Resume the saved branch, restore its outputs and reject opposite-branch records
  or later records after pending work. Recurse through IF as well as LOOP when
  validating agents and history. Preserve legacy workflows and v2 logs.
- LOOP accepts string arrays, runs `iter` in order, and has no aggregate output.
  Each iteration gets fresh locals and read-only `loop.item`. Inside a loop,
  `output: content` assigns `loop.content`; outside it assigns
  `outputs.content`. Preserve compatibility with `{{ loop.content }}` output
  targets in older definitions.
- Nested loops restore the parent scope. Local assignments must not leak into
  another iteration or global outputs. Outer outputs remain readable.
- Inserted content is never recursively expanded as a template.

## Conversations and durable history

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

## Agent generation

- `--newAgent` / `--new-agent` is standalone, not exposed to models or YAML.
- Ask separately for the created agent's adapter/model and the generator's
  adapter/model. Invoke the generator with its configuration and save the
  target agent's configuration.
- Generate the full definition from the description, validate it before
  writing, and keep canonical target metadata. Reject changed metadata,
  invalid definitions, and failed processes.
- Do not overwrite existing agents. Retain model lowercase normalization.

