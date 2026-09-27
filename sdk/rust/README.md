# S.I.R.K. Rust SDK

This package is a standalone client for the S.I.R.K. line-delimited JSON-RPC
protocol. It has no source or package dependency on the S.I.R.K. CLI.

```rust
use sirk_sdk::Sirk;

let mut sirk = Sirk::start("./sirk", ".")?;
let response = sirk.agent("code-explainer", "Explain this module")?;
```

`Sirk::start` launches `sirk rpc` in the supplied project directory. Filesystem,
Git, user interaction, and workflow control remain the responsibility of the
host program. The public RPC surface currently contains only `agent.run`.

Each JSON-RPC message occupies one line:

```json
{"jsonrpc":"2.0","id":1,"method":"agent.run","params":{"agent":"code-explainer","input":"Explain this module"}}
{"jsonrpc":"2.0","id":1,"result":"This module ..."}
```

Requests are processed sequentially. READ, TREE, EDIT, and DELETE requests made
by the agent remain internal to the CLI. An agent interaction that requires user
input currently returns an RPC error; bidirectional input is not part of protocol
version 1.
