# Product and API

- Remove Git functionality from the core. Projects that need Git operations
  should implement them independently.
- Replace flow IDs with UUIDs.
- Add an `ALLOW_READ: allow` permission, following the existing `EDIT_TOOL`
  permission pattern, and document the migration from the current default
  READ access.
- Bind the HTTP server to `0.0.0.0` by default so devcontainers and port
  forwarding work correctly.
- Require token-based authentication for HTTP requests. The token must be
  configurable through the service environment or configuration, must never
  appear in logs or command output, and must be sent by the SDK on every
  request.
- Set an explicit maximum HTTP request-body size and return a useful client
  error when it is exceeded.
- Improve error handling across the project:
  - Handle expected failure cases and remove panic-prone `unwrap` and `expect`
    calls from application paths.
  - Ensure propagated errors produce useful, actionable results instead of
    terminating the application.
  - Map API errors to appropriate status codes so client, configuration, and
    provider errors are not reported as generic HTTP 500 responses.

# Execution safety and local data

- Add execution controls:
  - Set a timeout for each provider call.
  - Terminate the child process and its process group when a call times out.
  - Limit the number of agent turns and tool calls per request.
  - Limit the amount of output captured from child processes.
  - Limit the number of concurrent calls.
- Define how flow transcripts are protected and retained. They contain full
  prompts and model responses and may include sensitive project data.
- Use restrictive filesystem permissions for newly created history files and
  directories where the platform supports them.
- Provide an explicit way to remove old flow transcripts, or configure a
  retention period.
- Add a `sirk doctor` command to check Bash, curl, installed harnesses,
  authentication, configured models, and other required local dependencies
  without exposing credentials.
- Add a readiness endpoint separate from `/health` to report whether required
  local service dependencies are available, without calling a model.
- Document the local-development deployment model, including devcontainers,
  mounted project volumes, port forwarding, authentication, and the effect of
  publishing a container port beyond a trusted network.

# Operation and maintainability

- Add structured logging.
- Add a request ID to correlate each HTTP request with its log entries.
- Add basic operational metrics, such as request counts, failures, and
  durations.
- Implement graceful shutdown that stops accepting requests, waits for or
  terminates active provider processes according to the configured deadline,
  and preserves consistent history files.
- Keep the generated API specification, contracts, README, and generated
  architecture documentation aligned with the implemented behavior.

# Testing, governance, and release

- Add real integration tests for every supported adapter, kept separate from
  fake-harness tests. Make live tests opt-in and ensure their credentials and
  costs cannot leak into ordinary local or CI runs.
- Add a project license and declare its SPDX identifier in `Cargo.toml` and
  the generated OpenAPI document.
- Add `SECURITY.md` with a vulnerability-reporting process and supported
  versions.
- Add `CONTRIBUTING.md` with the local development workflow and contribution
  expectations.
- Document the supported Rust toolchain or MSRV and supported execution
  platforms.
- Add CI and release automation on GitHub:
  - Run `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, and `cargo build` in CI.
  - Verify that the generated OpenAPI specification is current.
  - Audit dependencies for known vulnerabilities and license compatibility.
  - Build and verify binaries for supported platforms, and document unsupported platforms.
  - Publish versioned GitHub releases with binaries, checksums, provenance or signatures, release notes, a changelog, and upgrade instructions.
