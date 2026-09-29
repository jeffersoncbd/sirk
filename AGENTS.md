# S.I.R.K. development instructions

## Read only what the task needs

- [README.md](README.md) is the concise user manual.
- The entire `docs/` directory is read-only. You may consult files under
  `docs/`, but never create, edit, rename, or delete anything there.
- [docs/TREE.md](docs/TREE.md) is the trusted source-file and architecture index.
  At the start of a task, read only this file from `docs/`; do not enumerate or
  load the rest of that directory. Use it to locate the relevant modules, then
  consult only their documentation when the task requires it. For
  `src/foo/bar.rs`, read `docs/src/foo/bar.md`; `Cargo.toml` maps to
  `docs/Cargo.md`.
- Consult [REFERENCE.md](REFERENCE.md) for detailed agent behavior, and the
  relevant sections of [CONTRACTS.md](CONTRACTS.md) for compatibility
  requirements. Neither needs to be loaded in full each session.
- Inspect the relevant current source before editing. Documentation is the
  navigation reference; reconcile any discrepancy with code and tests, and
  report documentation that became stale without modifying `docs/`.

These instructions apply to development. Agent definition generators should
perform their assigned task without repository maintenance or build checks
unless requested. `.agents/` contains product data, not development instructions.

## Before editing

- Check `git status --short` and relevant diffs. Preserve user changes, especially
  workflows, `.agents/`, and ignore rules.
- Read the agent definitions and Rust workflow relevant to the task. The bundled
  documentation workflow is [documentation.rs](flows/documentation.rs).
- Stay within the requested feature. Known limitations, the deprecated
  `serde_yaml 0.9` dependency, and the broken `build.sh` are not implicit tasks.
- The project is English-only. Write every project artifact in English,
  regardless of the language used in the conversation with an AI agent.
  This includes source code, identifiers, function and test names, comments,
  documentation, specifications, agent definitions, messages, errors, logs,
  command output, and generated files. Do not create, retain, or copy
  project content in another language; translate user-supplied text before
  placing it in a project artifact.

## Implementation boundaries

Every source file must contain at most one function or method definition. This
rule is mandatory for all new or modified code, including tests and helper
functions. When adding another function or method, place it in a separate file
and expose it through the appropriate module; never add a second function or
method to the same file.

Keep orchestration out of `main.rs` and external process execution in
`BashService`. Build executable/argument lists through `Invocation`; never
interpolate prompts or paths into shell code.
Preserve binary capture for Git's NUL-delimited paths.

Keep Codex read-only with `approval_policy="never"` and `--skip-git-repo-check`.
Do not add automatic write approvals or `write` agent metadata. Close child stdin;
interaction goes through `UserInput`. Check the installed harness CLI's help
before changing flags. New adapters implement `HarnessAdapter` and register in
`src/adapters/mod.rs`; verify their permissions and output semantics.

Before changing agent execution or tools, consult the corresponding
[contracts](CONTRACTS.md) and module documents indexed by [TREE](docs/TREE.md).

## Build and verify

Build and test Rust on the host from the project root. For code changes, run:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
cargo build
```

Use `cargo fmt` if needed. Documentation-only changes
require document and link checks, not Rust builds. Avoid `build.sh`: it removes
the root binary, assumes Cargo is available, and continues after build failure.

Use fake harness callbacks and `new_agent::create_with` for tests without model
access. Cover meaningful agent behavior and permissions for the affected
feature. Distinguish fake harness tests from live model integrations. Run live
checks only where the harness is installed and authenticated. Previous
integration runs do not validate later changes.

After a successful build, refresh the root executable with
`cp target/debug/sirk ./sirk` when delivering code to run, or state that it
still needs refreshing.

## Documentation and handoff

Keep README limited to setup, everyday commands, and a short usage overview.
Never modify files under `docs/`, even when behavior, contracts, file
responsibilities, or inventory change. Report any resulting documentation gap
in the handoff. Do not duplicate the architecture map here or load all module
docs by default.

Report changes, relevant checks, and limitations. Avoid stale container IDs,
fixed test counts, or historical success claims as ongoing guarantees.
