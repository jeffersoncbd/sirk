# S.I.R.K. Rust SDK

This package is a standalone HTTP client for the S.I.R.K. service. Start the
service before connecting:

```bash
sirk http
```

```rust
use sirk_sdk::Sirk;

let sirk = Sirk::connect()?;
let response = sirk.agent("code-explainer", "Explain this module")?;

let changed = sirk.tools().git().status()?;
let files = sirk.tools().tree()?;
// sirk.tools().await_confirm()?;
// sirk.tools().git().add()?;
```

`Sirk::connect` uses `http://127.0.0.1:8080` and sends the calling process's
current directory. Use `Sirk::connect_to` when the service has a different
address or the project has a different server-visible path:

```rust
let sirk = Sirk::connect_to("http://sirk:8080", "/workspace/project")?;
```

The SDK checks `GET /health` while connecting and sends agent calls to
`POST /v1/agent/run`. If it cannot reach the service, its error instructs the
user to start it with `sirk http`. Filesystem, Git, user interaction, and
workflow control remain the responsibility of the host program. READ, TREE,
EDIT, and DELETE requests made by the agent remain internal to S.I.R.K.

The SDK provides host-side project tools: `tools().git().status()` returns
changed paths, `tools().git().add()` stages all changes under the local project
directory, `tools().tree()` lists Git-visible files while applying `.treeignore`,
and `tools().await_confirm()` waits for Enter (or `/cancel`) on the terminal. These
tools use the SDK process's current directory, even when the agent service is
configured with a different server-visible directory. Workflow control remains
the responsibility of the host program.
