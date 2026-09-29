# S.I.R.K.

**S.I.R.K.** honors Alexander Shchukin, Dmitri Ivanov, Liliya Rodina, and Georgi
Kreier, researchers at the Vavilov Institute who died of starvation during the
Siege of Leningrad while protecting its collection of crop seeds. They chose to
preserve seeds that held promise for reducing hunger rather than consume them to
survive. Their work helped safeguard agricultural diversity that continues to
support crop research and food supplies around the world. Their legacy lives on
in the effort to feed billions of people worldwide.
[Read the account at the Science History Institute](https://www.sciencehistory.org/stories/magazine/the-tragedy-of-the-worlds-first-seed-bank/).

S.I.R.K. runs Markdown-defined coding agents through HTTP. Language SDKs own
workflow control, filesystem access, and user interaction. Git operations use
the service's HTTP API. Supported adapters are Codex, OpenCode, Ollama, Ollama
Web, and OpenRouter. Claude Code is not implemented.

## Setup

Build on the host from the project root:

```bash
cargo build
cp target/debug/sirk ./sirk
./sirk --help
```

Repeat the copy after rebuilding. Install and authenticate the selected CLI
where the service runs. OpenRouter uses `curl`, `OPENROUTER_API_KEY`, and an
explicit model slug such as `~openai/gpt-sol-latest`; set the key in the
execution directory's `.env` file or environment. `OPENROUTER_URL` can override
its endpoint. Bash is required; TREE also requires Git and a working tree.

## Run the service and SDK

Start the HTTP service with `./sirk http` (default `127.0.0.1:8080`). An
explicit address such as `0.0.0.0:8080` allows connections from another host
or container. HTTP provides `GET /health`, `GET /openapi.yaml`, `GET /swagger`,
`POST /v1/agent/run`, `POST /v1/tree`, `POST /v1/git/status`, and
`POST /v1/git/add`.
`/openapi.yaml` returns the API specification as plain text; open `/swagger`
in a browser to view it in Swagger UI. TREE and Git requests supply the
server-visible project `directory`; agent requests also supply `agent` and
`input`. HTTP has no authentication, so expose it only on trusted networks.

The [Rust SDK](sdk/rust/README.md) connects to the HTTP service:

```rust
use sirk_sdk::Sirk;

let sirk = Sirk::connect()?;
let explanation = sirk.agent("code-explainer", "Explain this module")?;
```

Each call saves a conversation log under `history/`. There is no CLI command
to resume a log.

Create an agent in `.agents/<name>.md`:

```markdown
---
adapter: codex
---

Explain the requested code clearly and concisely.
```

`./sirk --new-agent` (alias `--newAgent`) can generate a definition
interactively. It asks separately for the created agent's and generator's
adapter and model. Omit `model` in a hand-written definition to use the
adapter's default.

The repository's Rust documentation workflow is
[flows/documentation.rs](flows/documentation.rs). Run it with
`cargo run --bin documentation` while the HTTP service is running. To enable
the documentation pre-commit hook, run `git config core.hooksPath .githooks`.
The hook requires a running service and authenticated Codex; a failure cancels
the commit and leaves generated files in the working tree.

See the [agent reference](REFERENCE.md) for permissions and tool requests, and
the [development instructions](AGENTS.md) for contributor checks. Agent calls
have no automatic retries or timeouts. JSON harness streams (`json: true`) are
not supported.
