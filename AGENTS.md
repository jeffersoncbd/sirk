# Development instructions

## Read only what the task needs

- [README.md](README.md) is the concise user manual.
- The entire `docs/` directory is read-only. You may consult files under
  `docs/`, but never create, edit, rename, or delete anything there.
- [docs/TREE.md](docs/TREE.md) is the trusted source-file and architecture index.
  Start there to locate the relevant modules instead of scanning all source or
  loading every document. For `src/foo/bar.rs`, read `docs/src/foo/bar.md`;
  `Cargo.toml` maps to `docs/Cargo.md`.
- Consult [REFERENCE.md](REFERENCE.md) for detailed user-facing syntax
  and behavior, and the relevant sections of [CONTRACTS.md](CONTRACTS.md)
  for compatibility requirements. Neither needs to be loaded in full each session.
- Inspect the relevant current source before editing. Documentation is the
  navigation reference; reconcile any discrepancy with code and tests, and
  report documentation that became stale without modifying `docs/`.

These instructions apply to development. Workflow agents and definition
generators should perform their assigned task without repository maintenance or
build checks unless requested. `.agents/` contains product data, not development
instructions.

## Before editing

- Check `git status --short` and relevant diffs. Preserve user changes, especially
  workflows, `.agents/`, and ignore rules.
- Read the workflow and agent definitions relevant to the task. The bundled
  workflow is [documentation.yml](documentation.yml).
- Stay within the requested feature. Known limitations, the deprecated
  `serde_yaml 0.9` dependency, and the broken `build.sh` are not implicit tasks.
- Write tool-owned code, comments, messages, tests, and logs in English.
  Preserve user-authored text in its original language.

## Implementation boundaries

Keep orchestration out of `main.rs`, provider behavior out of workflow schemas,
and external process execution in `BashService`. Build executable/argument lists
through `Invocation`; never interpolate prompts or paths into shell code.
Preserve binary capture for Git's NUL-delimited paths.

Keep Codex read-only with `approval_policy="never"` and `--skip-git-repo-check`.
Do not add automatic write approvals or `write` agent metadata. Close child stdin;
interaction goes through `UserInput`. Check the installed harness CLI's help
before changing flags. New adapters implement `HarnessAdapter` and register in
`src/adapters/mod.rs`; verify their permissions and output semantics.

Preserve supported v2 transcripts and workflow compatibility. Before changing
execution, tools, scopes, or recovery, consult the corresponding
[contracts](CONTRACTS.md) and module documents indexed by [TREE](docs/TREE.md).

## Build and verify

Build and test Rust only inside the VS Code Dev Container; do not install Rust on
the host. Discover the active container each session with `docker container ls`.
Always use `-u vscode` to avoid root-owned artifacts. For code changes, run:

```bash
docker exec -u vscode -w /workspaces/new-harness <container-id> cargo fmt --check
docker exec -u vscode -w /workspaces/new-harness <container-id> cargo test
docker exec -u vscode -w /workspaces/new-harness <container-id> cargo clippy --all-targets -- -D warnings
docker exec -u vscode -w /workspaces/new-harness <container-id> cargo build
```

Use `cargo fmt` in the same environment if needed. Documentation-only changes
require document and link checks, not Rust builds. Avoid `build.sh`: it removes
the root binary, assumes Cargo is available, and continues after build failure.

Use `runner::run_with`, `run_interactive_with`, `continue_with`, and
`new_agent::create_with` for tests without model access. Cover meaningful behavior
and recovery for the affected feature. Tests use fake harness callbacks and real
local Bash/Git; distinguish them from live model integrations. Run live checks
only where the harness is installed and authenticated (currently Codex on the
host). Previous integration runs do not validate later changes.

After a successful build, refresh the root executable on the host with
`cp target/debug/new-harness ./new-harness` when delivering code to run, or state
that it still needs refreshing. Host architecture and system libraries must be
compatible.

## Documentation and handoff

Keep README limited to setup, everyday commands, and a short usage overview.
Never modify files under `docs/`, even when behavior, contracts, file
responsibilities, or inventory change. Report any resulting documentation gap
in the handoff. Do not duplicate the architecture map here or load all module
docs by default.

Report changes, relevant checks, and limitations. Avoid stale container IDs,
fixed test counts, or historical success claims as ongoing guarantees.
