# S.I.R.K.

GitHub repository: [jeffersoncbd/sirk](https://github.com/jeffersoncbd/sirk).

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
the service's HTTP API. Supported adapters are Codex, NVIDIA API, OpenCode,
Ollama, Ollama Web, and OpenRouter. Claude Code is not implemented.

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
its endpoint. NVIDIA API uses `curl`, `NVIDIA_API_KEY`, and an explicit model
slug such as `deepseek-ai/deepseek-v4.1-flash`; set the key in the execution
directory's `.env` file or environment. Bash is required; TREE also requires
Git and a working tree.

## Run the service and SDK

Start the HTTP service with `./sirk http` (default `127.0.0.1:8080`). An
explicit address such as `0.0.0.0:8080` allows connections from another host
or container. HTTP provides `GET /health`, `GET /openapi.yaml`, `GET /swagger`,
`POST /v1/flows`, `POST /v1/agent/run`, `POST /v1/tree`,
`POST /v1/git/status`, and `POST /v1/git/add`.
`/openapi.yaml` returns the API specification as plain text; open `/swagger`
in a browser to view it in Swagger UI. TREE and Git requests supply the
server-visible project `directory`; agent requests also supply `agent` and
`input`. Create a flow with `POST /v1/flows` before making agent, TREE, or Git
requests, then send its returned `flowId` in the `X-Sirk-Flow-Id` header. One
flow creates `history/<flow-id>/flow.log`, `usage.log`, and a `conversations/`
directory. The flow log records complete model prompts and responses. When an
adapter reports token usage, its response is followed by an `==> USAGE` entry;
`usage.log` aggregates calls and token totals. Each agent request starts a
conversation only when an agent with `ASK_TOOL: allow` returns `ASK:`;
conversation JSON files preserve user, agent, and tool messages used to
continue that interaction. HTTP has no authentication, so expose it only on
trusted networks.

The [Rust SDK](https://github.com/jeffersoncbd/sirk-rust-sdk) connects to the HTTP service:

```rust
use sirk_sdk::Sirk;

let sirk = Sirk::connect()?;
let explanation = sirk.agent("code-explainer", "Explain this module")?;
```

The SDK must create a flow before its first agent, TREE, or Git request and
send the returned ID in `X-Sirk-Flow-Id`. A standalone agent result has no
`conversationId`. After an `ask` response, send its `conversationId` with the
user's answer so the service can restore the conversation. There is no CLI
resume operation.

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
`cargo run --bin run-flow -- documentation` while the HTTP service is running.
Run the profile interview with
`cargo run --bin run-flow -- profile-interviewer`. To enable
the documentation pre-commit hook, run `git config core.hooksPath .githooks`.
The hook requires a running service and authenticated Codex; a failure cancels
the commit and leaves generated files in the working tree.

See the [agent reference](REFERENCE.md) for permissions and tool requests, and
the [development instructions](AGENTS.md) for contributor checks. Agent calls
have no automatic retries or timeouts. JSON harness streams (`json: true`) are
not supported.
