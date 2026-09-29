
.env.example - Documents environment variables for configuring Ollama and OpenRouter service URLs and API keys.
Cargo.toml - Defines the `sirk` package, its entry points, and dependencies used by the library and executable.
openapi.yaml - Specifies S.I.R.K.’s HTTP routes, request and response formats, errors, and server behavior.
src/adapters/codex.rs - Configures request execution through `codex exec`, building its arguments and prompt.
src/adapters/codex/default.rs - Defines the default CodexAdapter configuration using the `codex` command.
src/adapters/codex/new.rs - Creates a `CodexAdapter` that stores the supplied executable as text.
src/adapters/mod.rs - Centralizes available adapters, re-exports adapter resolution, and lists accepted names.
src/adapters/ollama.rs - Converts supported requests into `ollama run` invocations with a model, prompt, and working directory.
src/adapters/ollama/default.rs - Creates and returns a default `OllamaAdapter` identified by the name `ollama`.
src/adapters/ollama/new.rs - Initializes an OllamaAdapter by storing the supplied executable as text.
src/adapters/ollama_web.rs - Adapts requests into non-streaming calls to the Ollama web API and extracts the response.
src/adapters/ollama_web/api_key.rs - Gets the Ollama API key, prioritizing adapter configuration, the environment, and the dotenv file.
src/adapters/ollama_web/default.rs - Defines default Ollama Web adapter values using `curl` with no base URL or API key.
src/adapters/ollama_web/dotenv.rs - Gets a nonempty key value from a directory's `.env` file, returning no value when absent.
src/adapters/ollama_web/endpoint.rs - Builds the Ollama Web generation endpoint from the available configuration.
src/adapters/ollama_web/new.rs - Creates an `OllamaWebAdapter` with supplied values and normalizes optional URL and API key fields.
src/adapters/ollama_web/nonempty.rs - Returns the original text in `Some` if it contains a non-whitespace character; otherwise, returns `None`.
src/adapters/opencode.rs - Declares `OpenCodeAdapter`, which represents requests as OpenCode CLI calls.
src/adapters/opencode/default.rs - Creates the default OpenCode adapter configured to use the `opencode` command.
src/adapters/opencode/id.rs - Defines, through a macro, the method that identifies the adapter by the static name `opencode`.
src/adapters/opencode/invocation.rs - Builds an OpenCode invocation for a harness request with its options, prompt, and working directory.
src/adapters/opencode/new.rs - Creates an OpenCodeAdapter that stores the supplied executable as text.
src/adapters/opencode/tests.rs - Declares and organizes OpenCode adapter test modules with supporting symbols and types.
src/adapters/opencode/tests/does_not_enable_automatic_permission_approval.rs - Verifies that the OpenCode adapter does not enable automatic permission approval.
src/adapters/opencode/tests/translates_generic_options_to_opencode_run.rs - Verifies that generic options convert to the expected OpenCode invocation.
src/adapters/openrouter.rs - Converts harness requests into non-streaming OpenRouter API calls and extracts response content.
src/adapters/openrouter/api_key.rs - Gets the OpenRouter API key from available configuration or returns an error if none is found.
src/adapters/openrouter/default.rs - Defines the OpenRouter adapter using `curl` with no base URL or API key configured.
src/adapters/openrouter/dotenv.rs - Reads a nonempty key value from a directory's `.env` file when present.
src/adapters/openrouter/endpoint.rs - Configures the OpenRouter chat endpoint from the available URL, applying the required path and default.
src/adapters/openrouter/new.rs - Creates an OpenRouter adapter with an executable and optional base URL and API key, discarding empty values.
src/adapters/openrouter/nonempty.rs - Returns the original string if it contains non-whitespace content; otherwise, returns `None`.
src/adapters/resolve.rs - Finds a harness adapter by name and returns `None` when no match exists.
src/agent_service.rs - Exposes `run::run` internally as the entry point for agent execution.
src/agent_service/conversation.rs - Runs the agent conversation, processing responses and tools until a final response is received.
src/agent_service/delete_request.rs - Processes file-deletion requests, checks authorization for forced execution, and prevents repeats.
src/agent_service/edit_request.rs - Prepares and applies an edit request, records it in history, and prevents repeating completed edits.
src/agent_service/execute.rs - Executes an invocation and returns stdout only on success, converting failures into messages.
src/agent_service/prompt.rs - Builds the agent prompt from tool instructions, permissions, and filtered conversation history.
src/agent_service/run.rs - Runs a conversation with an agent loaded for a directory after validating its adapter and creating history.
src/agents.rs - Validates metadata and instructions in a text definition and converts it into an agent.
src/agents/call_prefix.rs - Deserializes `call_prefix` as an argument list, accepting a string or list and treating absence as an empty list.
src/agents/delete_permission.rs - Deserializes `"allow"` as `true` and rejects other values with a descriptive error.
src/agents/edit_permission.rs - Converts the string `allow` to `true` and rejects other values with an error.
src/agents/id.rs - Validates that a nonempty identifier contains only ASCII letters, digits, underscores, or hyphens.
src/agents/load.rs - Loads and validates an agent from a Markdown file, returning it or explaining the failure.
src/agents/model.rs - Deserializes an optional model and normalizes it by trimming whitespace and converting it to lowercase.
src/agents/tree_default.rs - Always returns `true` to allow the TREE tool, without validation or side effects.
src/agents/tree_permission.rs - Deserializes `allow` as `true` and rejects other values with an error stating the expected value.
src/bin/documentation.rs - Connects to S.I.R.K. and runs the documentation flow, returning its result.
src/git_service.rs - Centralizes and exposes Git add and repository status operations.
src/git_service/add.rs - Adds all changes in the project associated with the supplied directory to Git and returns errors on failure.
src/git_service/project.rs - Resolves a Git directory and returns its root and relative prefix, converting failures into error messages.
src/git_service/run.rs - Runs Git commands in a directory and returns stdout bytes, contextualizing execution and status failures.
src/git_service/status.rs - Lists changed Git paths under the requested directory in order without duplicates, excluding ignored files.
src/harness.rs - Re-exports interface types that define the harness integration contracts.
src/history.rs - Defines mutable conversation history, including its path, snapshot, blocks, and lock file.
src/history/create.rs - Creates and saves empty history for a snapshot in a file named with the time and process PID.
src/history/drop.rs - Releases the `History` lock when the value is dropped, ignoring unlock errors.
src/history/lock.rs - Acquires an exclusive lock on the history lock file.
src/history/reserved.rs - Identifies lines containing reserved history markers without side effects.
src/history/save.rs - Persists the history snapshot to disk by serializing metadata and blocks as YAML.
src/http.rs - Defines HTTP support modules and re-exports `serve` as the public entry point.
src/http/handle.rs - Handles health and API documentation routes, dispatching agent and Git requests with appropriate HTTP responses.
src/http/openapi.rs - Embeds the OpenAPI specification as a string constant for use by the HTTP module.
src/http/serve.rs - Runs a threaded HTTP server that forwards requests to a handler and returns its responses.
src/http/swagger.rs - SWAGGER_UI embeds an HTML page that loads Swagger UI and displays the API specification at `/openapi.yaml`.
src/http/tests.rs - Groups modules that test HTTP behavior.
src/http/tests/enforces_agent_delete_permissions.rs - Verifies that files are deleted only when the agent's explicit permissions are enabled.
src/http/tests/handles_agent_edits.rs - Tests an HTTP request that runs an agent able to edit a file and return the expected response.
src/http/tests/handles_agent_requests.rs - Verifies through an HTTP request that agent execution returns the expected status and result.
src/http/tests/handles_git_add_requests.rs - Tests that `/v1/git/add` stages a file in the supplied directory.
src/http/tests/handles_git_status_requests.rs - Verifies that Git status returns visible files while excluding `.treeignore` matches.
src/http/tests/preserves_literal_template_input.rs - Verifies that `{{ outputs.plan }}` is preserved literally in agent input sent through HTTP.
src/http/tests/rejects_invalid_requests.rs - Tests HTTP handler responses for documented routes, unknown routes, unsupported methods, and invalid request bodies.
src/http/types.rs - Defines HTTP content-type constants and request and response data types.
src/input.rs - Reads user responses and confirmations from the interactive terminal.
src/interfaces/harness.rs - Defines the shared interface and types for adapters that run coding agents.
src/interfaces/harness/display.rs - Formats `HarnessError` variants as readable messages, including the adapter and available details.
src/interfaces/history.rs - Defines `Snapshot` and `Block` for representing and serializing history records.
src/interfaces/history/marker.rs - Returns the fixed text marker corresponding to a history block type.
src/interfaces/history/text.rs - Returns a reference to the text associated with a history block without modifying it.
src/interfaces/input.rs - Defines the shared interface for asking the user questions and awaiting confirmations.
src/interfaces/invocation.rs - Prepends a command to an invocation while preserving the original program and execution settings.
src/interfaces/mod.rs - Centralizes and re-exports shared harness, history, user input, and invocation types.
src/lib.rs - Declares crate modules, exposing them publicly except for private `agent_service` and `git_service` modules.
src/main.rs - Passes command arguments for execution and sets the process exit code based on the result.
src/main/create_agent.rs - Creates an agent in the current directory using terminal input and displays the created path.
src/main/http.rs - Starts the HTTP server at the supplied address or the default and returns any errors.
src/main/print_usage.rs - Prints the usage text supplied by the `usage` module.
src/main/run.rs - Dispatches arguments to agent creation, the HTTP command, or help output.
src/main/tests.rs - Groups application execution and usage tests with access to `run` and `usage`.
src/main/tests/accepts_no_arguments_without_starting_a_prompt.rs - Verifies that calling `run` without arguments succeeds; it does not describe a production flow.
src/main/tests/rejects_an_invalid_command.rs - Confirms that the invalid command `"invalid"` is rejected with a matching error message.
src/main/tests/rejects_removed_workflow_commands.rs - Verifies that `run` rejects invalid or removed commands with the expected error message.
src/main/tests/usage_lists_service_commands.rs - Tests that `usage()` includes expected commands and omits obsolete ones.
src/main/usage.rs - Displays S.I.R.K. usage help, available commands, the default address, agent creation, and input cancellation.
src/services/bash.rs - Defines the shared service for executing processes through Bash and representing captured status and output.
src/services/bash/default.rs - Configures the default BashService to run commands using the `bash` executable.
src/services/bash/execute_bytes_to.rs - Runs a Bash invocation and returns its status with captured output bytes.
src/services/bash/execute_streaming.rs - Runs an invocation, streams its output to stdout, and returns the result or an I/O error.
src/services/bash/execute_to.rs - Runs an invocation, sends UTF-8 stdout to a destination, preserves its status, and propagates errors.
src/services/bash/new.rs - Constructs a BashService and stores the supplied value as a String in `executable`.
src/services/bash/render.rs - Converts an `Invocation` into a command line, quoting the program and arguments for the environment.
src/services/mod.rs - Collects and re-exports modules and types used to build and run commands.
src/services/quote.rs - Converts `value` into shell-safe text while preserving references to valid environment variables.
src/tools/delete.rs - Safely removes a regular file inside the execution directory, validating the path and synchronizing its parent.
src/tools/edit.rs - Defines types representing file edit operations, their parameters, and previous file state.
src/tools/edit/apply_to.rs - Applies a validated edit and returns updated contents or an error.
src/tools/edit/commit.rs - Applies a pending edit to its target file, protects against concurrent changes, and returns a diff.
src/tools/edit/diff.rs - Generates a unified diff between original and edited contents to present prepared changes.
src/tools/edit/display.rs - Renders and prints a diff, using colors when stdout is a terminal and `NO_COLOR` is unset.
src/tools/edit/pending_validate.rs - Validates a `Pending` record against its request, previous file state, and operation.
src/tools/edit/prepare.rs - Prepares a file edit by validating the operation and recording previous contents or their absence.
src/tools/edit/read_optional.rs - Reads a UTF-8 file and returns its contents, absence, or a read error.
src/tools/edit/render.rs - Renders diffs, highlighting additions and removals with colors and escaping content control characters.
src/tools/edit/request_validate.rs - Validates that edit request fields are compatible with the requested operation.
src/tools/edit/sync_parent.rs - Synchronizes the parent directory of a path to disk and converts failures into messages.
src/tools/edit/target.rs - Resolves and validates an edit target path, ensuring it stays within the execution directory.
src/tools/edit/tests.rs - Creates an edit request with its operation, input, and version calculated from previous contents.
src/tools/edit/tests/append_and_prepend_are_exact_and_versions_are_checked.rs - Validates append and prepend operations, including empty input, and detects version conflicts.
src/tools/edit/tests/diff_and_color_rendering_preserve_plain_results.rs - Tests diff generation and rendering, including colors, escapes, and missing final newlines.
src/tools/edit/tests/line_edits_preserve_bytes_and_handle_eof.rs - The file’s documented behavior was not provided, so I can’t summarize its purpose.
src/tools/edit/version.rs - Calculates the SHA-256 hash of a string's UTF-8 bytes and returns a lowercase hexadecimal digest.
src/tools/execute_with_input.rs - Runs TREE or READ in the supplied directory, formatting listings or returning requested contents.
src/tools/format_paths.rs - Formats file paths as a pretty-printed JSON array, reporting conversion and serialization errors with the tool name.
src/tools/mod.rs - Organizes agent tools and re-exports execution and request operations.
src/tools/new_agent.rs - `create_with` gathers and validates agent settings, generates the definition, and safely saves it under `.agents`.
src/tools/new_agent/adapter.rs - Prompts for an available adapter and repeats the question if the input does not match.
src/tools/new_agent/answer.rs - Repeats prompts until receiving a nonempty value and propagates interaction errors.
src/tools/new_agent/create.rs - Generates an agent in the supplied directory and returns the saved path.
src/tools/read.rs - Organizes the read tool's internal modules and exposes its main functions for external use.
src/tools/read/component.rs - Compares a pattern and a value as character sequences and returns whether they match.
src/tools/read/component_match.rs - Compares character sequences with `*` and `?` wildcards and reports whether they match.
src/tools/read/components.rs - Compares pattern and path components, supporting `**` and prefix matching.
src/tools/read/enumerate.rs - Prefixes each content line with numbering starting at 1 while preserving its original terminators.
src/tools/read/enumerated_content.rs - Reconstructs original contents from numbered output by removing numbers and joining valid lines.
src/tools/read/ignore.rs - Determines whether a path matches an ignore pattern by comparing its components.
src/tools/read/ignored.rs - Checks whether a path matches exclusion rules defined in `.readignore`.
src/tools/read/run.rs - Reads UTF-8 files inside the execution directory while blocking invalid or ignored paths.
src/tools/request.rs - Parses file tree and read requests, identifying the request type and requested path.
src/tools/tree.rs - Represents a file tree root and its sorted, deduplicated relative paths.
src/tools/tree/list.rs - Lists existing, non-ignored files under a Git working directory.
src/tools/tree/path.rs - Converts bytes to a `PathBuf`, preserving them on Unix and requiring UTF-8 on other platforms.