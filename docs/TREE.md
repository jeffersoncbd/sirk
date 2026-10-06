
.env.example - Configures optional URLs and API keys for Ollama, OpenRouter, and NVIDIA adapters.
Cargo.toml - Defines the `sirk` Rust package, its library and executable, and their dependencies.
openapi.yaml - Defines the S.I.R.K. HTTP API endpoints, schemas, validation rules, and error responses.
src/adapters/codex.rs - Configures request execution through `codex exec`, building its arguments and prompt.
src/adapters/codex/default.rs - Defines the default CodexAdapter configuration using the `codex` command.
src/adapters/codex/new.rs - Creates a `CodexAdapter` that stores the supplied executable as text.
src/adapters/mod.rs - Defines and re-exports adapter modules, adapter types, a resolver, and the accepted adapter names.
src/adapters/nvidia_api.rs - Configures the NVIDIA chat completions adapter with an executable and optional API key.
src/adapters/nvidia_api/adapter.rs - The NVIDIA API harness adapter builds invocations and parses their stdout responses.
src/adapters/nvidia_api/api_key.rs - api_key resolves the NVIDIA API key from adapter settings, the environment, or dotenv, returning a configuration error if absent.
src/adapters/nvidia_api/default.rs - Provides default settings for NvidiaApiAdapter, using curl without an API key.
src/adapters/nvidia_api/dotenv.rs - Reads a named nonempty value from a directory’s `.env` file, returning none when absent and a configuration error on read or parse failure.
src/adapters/nvidia_api/invocation.rs - Builds a non-streaming NVIDIA API invocation from a run request, including its JSON body and authorization.
src/adapters/nvidia_api/new.rs - Creates a `NvidiaApiAdapter` with the executable name and only a nonempty API key.
src/adapters/nvidia_api/nonempty.rs - Returns the original string as `Some` if it contains non-whitespace characters; otherwise returns `None`.
src/adapters/nvidia_api/response.rs - Parses NVIDIA API JSON into generated text and includes token usage when both counts are available.
src/adapters/nvidia_api/tests.rs - Declares test modules for dotenv, invocation, options, and response.
src/adapters/nvidia_api/tests/dotenv.rs - The file tests that NvidiaApiAdapter::invocation reads an API key from a temporary `.env` file.
src/adapters/nvidia_api/tests/invocation.rs - Tests verify NVIDIA chat requests use the expected endpoint and payload while keeping the API key in the environment.
src/adapters/nvidia_api/tests/options.rs - Tests verify that the NVIDIA API adapter rejects requests without a model or with event streams enabled.
src/adapters/nvidia_api/tests/response.rs - Tests verify that `response` extracts the first choice’s content and token usage, and errors when choices are missing.
src/adapters/ollama.rs - Converts supported requests into `ollama run` invocations with a model, prompt, and working directory.
src/adapters/ollama/default.rs - Creates and returns a default `OllamaAdapter` identified by the name `ollama`.
src/adapters/ollama/new.rs - Initializes an OllamaAdapter by storing the supplied executable as text.
src/adapters/ollama_web.rs - Builds non-streaming Ollama requests and extracts response text and token usage from API responses.
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
src/adapters/openrouter.rs - Builds non-streaming OpenRouter chat requests and extracts response text and token usage.
src/adapters/openrouter/api_key.rs - Gets the OpenRouter API key from available configuration or returns an error if none is found.
src/adapters/openrouter/default.rs - Defines the OpenRouter adapter using `curl` with no base URL or API key configured.
src/adapters/openrouter/dotenv.rs - Reads a nonempty key value from a directory's `.env` file when present.
src/adapters/openrouter/endpoint.rs - Configures the OpenRouter chat endpoint from the available URL, applying the required path and default.
src/adapters/openrouter/new.rs - Creates an OpenRouter adapter with an executable and optional base URL and API key, discarding empty values.
src/adapters/openrouter/nonempty.rs - Returns the original string if it contains non-whitespace content; otherwise, returns `None`.
src/adapters/resolve.rs - Maps recognized harness names to their default adapter, returning `None` for unknown names.
src/adapters/resolve/tests.rs - The tests verify that `resolve` recognizes four adapter IDs, all listed in `AVAILABLE`.
src/agent_service.rs - Exposes the transport-independent agent execution API and its outcome types for crate use.
src/agent_service/conversation.rs - Runs the configured agent through tool calls until it returns a final response, asks a validated question, or fails.
src/agent_service/delete_request.rs - Validates deletion requests, prevents repeats, and performs authorized forced file deletion.
src/agent_service/edit_request.rs - Prepares and commits edit requests while enforcing path permissions and preventing duplicate completed edits.
src/agent_service/execute.rs - Executes an invocation and returns stdout only on success, converting failures into messages.
src/agent_service/outcome.rs - Defines agent outcomes as results or input requests, paired with an execution record that may include a conversation ID.
src/agent_service/prompt.rs - Constructs an agent prompt from its instructions, enabled tools, and filtered conversation history.
src/agent_service/run.rs - Runs or resumes an agent conversation, saves its outcome and usage, and returns any errors as strings.
src/agent_service/write_request.rs - Converts a JSON payload into a write operation and delegates it to `edit_request`.
src/agents.rs - Agent::parse validates YAML front matter and Markdown instructions, then builds an `Agent` or returns descriptive errors.
src/agents/ask_permission.rs - `deserialize` returns `true` for `"allow"` and a custom error for every other value.
src/agents/call_prefix.rs - Deserializes `call_prefix` as an argument list, accepting a string or list and treating absence as an empty list.
src/agents/delete_permission.rs - Deserializes `"allow"` as `true` and rejects other values with a descriptive error.
src/agents/edit_permission.rs - Converts the string `allow` to `true` and rejects other values with an error.
src/agents/id.rs - Validates that a nonempty identifier contains only ASCII letters, digits, underscores, or hyphens.
src/agents/load.rs - Loads and validates an agent from a Markdown file, returning it or explaining the failure.
src/agents/model.rs - Deserializes an optional model and normalizes it by trimming whitespace and converting it to lowercase.
src/agents/read_permission.rs - Deserializes the string `"allow"` as `true` and returns an error for any other value.
src/agents/tests.rs - The file registers nine agent test modules by mapping each module to its source file.
src/agents/tests/enables_ask_tool.rs - Tests that an agent configuration enables `ask_tool` and serializes it as `ASK_TOOL: true`.
src/agents/tests/enables_delete_permissions.rs - Tests that agent configuration enables both delete permissions when explicitly set to `allow`.
src/agents/tests/enables_edit_tool.rs - Tests that an agent with edit access enabled serializes that setting.
src/agents/tests/enables_read_tool.rs - The test verifies that `READ_TOOL: allow` enables reading and serializes as `READ_TOOL: true`.
src/agents/tests/enables_tree_tool.rs - Tests that an Agent enables and serializes the tree tool when explicitly allowed.
src/agents/tests/normalizes_models.rs - The test verifies that model names are lowercased when parsing Markdown and restoring a YAML snapshot.
src/agents/tests/reads_call_prefix.rs - Agent::parse converts call prefixes from a single string or YAML list into an ordered argument list.
src/agents/tests/reads_metadata.rs - Agent::parse builds an agent from an ID and CRLF-formatted metadata and Markdown, with defaults for unspecified options.
src/agents/tests/rejects_invalid_definitions.rs - The test verifies that malformed agent definitions and unsafe IDs are rejected.
src/agents/tree_default.rs - Always returns `true` to allow the TREE tool, without validation or side effects.
src/agents/tree_permission.rs - Deserializes `allow` as `true` and rejects other values with an error stating the expected value.
src/bin/generate-openapi.rs - Generates the OpenAPI document as YAML and writes it to `openapi.yaml`.
src/bin/run-flow.rs - Runs the named documentation or profile interviewer flow through Sirk, reporting invalid or unknown flow names.
src/git_service.rs - Centralizes and exposes Git add and repository status operations.
src/git_service/add.rs - Adds all changes in the project associated with the supplied directory to Git and returns errors on failure.
src/git_service/project.rs - Resolves a Git directory and returns its root and relative prefix, converting failures into error messages.
src/git_service/run.rs - Runs Git commands in a directory and returns stdout bytes, contextualizing execution and status failures.
src/git_service/status.rs - Lists changed Git paths under the requested directory in order without duplicates, excluding ignored files.
src/harness.rs - Re-exports harness integration types from `crate::interfaces` for backward compatibility.
src/history.rs - History stores conversation state, usage data, file paths, and a file handle that keeps a lock.
src/history/conversation.rs - The file defines serializable types for conversation metadata, status, and user, assistant, and tool messages.
src/history/conversation_blocks.rs - Conversation::blocks converts each message’s content into its corresponding input, output, or tool-specific block.
src/history/conversation_id.rs - Generates a unique conversation ID from the current time, process ID, and an atomic counter.
src/history/conversation_replace_blocks.rs - Replaces conversation messages with cloned messages mapped from the provided blocks, skipping `Ask` blocks.
src/history/conversation_save.rs - Atomically saves conversation data as pretty-printed JSON, syncing the file and parent directory.
src/history/create.rs - Creates an empty history for an existing flow while retaining its snapshot and lock.
src/history/drop.rs - Releases the `History` lock when the value is dropped, ignoring unlock errors.
src/history/flow_id.rs - Generates a process-scoped flow ID from the current timestamp and an incrementing counter.
src/history/lock.rs - Opens and exclusively locks the lock file beside the history file, returning the locked file or an error.
src/history/new_flow.rs - Creates a history flow with its flow and usage files and conversations directory, returning its ID or an error string.
src/history/open_conversation.rs - Opens a validated conversation by ID or creates and persists a new conversation for a flow and agent.
src/history/path.rs - Builds the canonical path to a flow’s history log, returning filesystem errors as strings.
src/history/record_input.rs - Appends an input record to the history file and syncs it to storage.
src/history/record_model_call.rs - Records a model call and accumulates its token usage by adapter.
src/history/record_output.rs - Appends marked output to the history file and syncs the file and its parent directory.
src/history/record_resume.rs - Updates the resume file with per-adapter call and token totals plus the current agent and model.
src/history/record_usage.rs - Appends an adapter’s input and output token counts to the history file and syncs the file and its parent directory.
src/history/record_usage_summary.rs - Updates adapter usage totals and records the current agent and model in the usage summary file.
src/history/resume_call.rs - ResumeCall stores a call’s adapter, count, token totals, and usage availability.
src/history/resume_path.rs - Builds the canonical path to a flow’s resume log, returning filesystem errors as strings.
src/history/usage_call.rs - UsageCall records an adapter’s call count, token totals, and usage data availability.
src/history/usage_path.rs - Builds the canonical path to a flow’s usage log, converting filesystem errors to strings.
src/history/valid_conversation_id.rs - Validates conversation IDs by requiring the `conversation-` prefix followed by hexadecimal digits or hyphens.
src/history/valid_flow_id.rs - Checks whether a string starts with `flow-` and contains only ASCII hexadecimal digits or hyphens afterward.
src/history/validate_flow.rs - Validates a flow ID and confirms its resolved path points to a file.
src/http.rs - Exposes the HTTP server entry point and re-exports the OpenAPI document.
src/http/content_type.rs - Defines shared UTF-8 media type constants for HTML, JSON, and plain text responses.
src/http/controllers.rs - Declares HTTP controller submodules and makes them available to the parent module.
src/http/controllers/agent_run.rs - Runs an agent for a valid flow and returns its result or question as JSON.
src/http/controllers/flow_create.rs - Creates a flow transcript in the requested directory and returns its ID.
src/http/controllers/git_add.rs - Stages changes in a validated directory and returns an HTTP response describing the result.
src/http/controllers/git_status.rs - git_status validates a flow ID, then returns changed Git paths for the requested directory.
src/http/controllers/health.rs - Returns a JSON health response with status `ok`.
src/http/controllers/method_not_allowed.rs - Returns an HTTP 405 JSON error response when a method is not allowed.
src/http/controllers/not_found.rs - Returns a JSON 404 response with a “Not found” error for unmatched requests.
src/http/controllers/openapi.rs - Serves the binary’s OpenAPI specification as plain text.
src/http/controllers/swagger.rs - Serves the embedded Swagger UI as an HTML response.
src/http/controllers/tree.rs - Lists files in a validated directory and returns their paths as JSON, with errors for invalid requests or listing failures.
src/http/flow_id.rs - Validates the `X-Sirk-Flow-Id` header and returns its value or a JSON error response.
src/http/json_response.rs - Converts a serializable value into an HTTP response with a JSON content type.
src/http/openapi.rs - Embeds the OpenAPI specification as a string constant for use by the HTTP module.
src/http/routes.rs - Builds the HTTP router and OpenAPI document, registering API and fallback handlers.
src/http/schemas.rs - This module exposes the request and response schema submodules to its parent.
src/http/schemas/requests.rs - This module exposes the agent-run and directory request types within `crate::http`.
src/http/schemas/requests/agent_run.rs - AgentRunRequest defines the execution directory, agent, input, and optional conversation ID for an agent run.
src/http/schemas/requests/directory.rs - DirectoryRequest deserializes a server-visible directory path and documents it in the OpenAPI schema.
src/http/schemas/responses.rs - Re-exports HTTP response schema types within `crate::http`.
src/http/schemas/responses/agent_run.rs - `AgentRunResponse` serializes an agent conversation ID with an optional result or question and supports OpenAPI schemas.
src/http/schemas/responses/error.rs - Represents an HTTP error response with a human-readable message.
src/http/schemas/responses/flow.rs - FlowResponse serializes a flow identifier for HTTP responses, intended for reuse in later request headers.
src/http/schemas/responses/git_status.rs - Represents sorted, unique Git status paths relative to the requested directory in an HTTP response.
src/http/schemas/responses/health.rs - Defines the health endpoint’s success response with a status field.
src/http/schemas/responses/status.rs - Defines a serializable HTTP status response with a fixed success indicator and OpenAPI schema.
src/http/schemas/responses/success.rs - SuccessStatus defines the serializable `ok` status for successful HTTP responses.
src/http/schemas/responses/tree.rs - TreeResponse serializes sorted, unique paths relative to the requested directory and rejects unknown fields.
src/http/serve.rs - Runs the HTTP server at the supplied address on a multi-threaded Tokio runtime, returning errors as strings.
src/http/spec.rs - Returns the OpenAPI document generated from the HTTP routes.
src/http/swagger.rs - SWAGGER_UI embeds an HTML page that loads Swagger UI and displays the API specification at `/openapi.yaml`.
src/http/tests.rs - Declares HTTP test modules and maps each module to its test file under `tests/`.
src/http/tests/creates_flow_directory_layout.rs - The test verifies that `flow` creates the expected log files and conversations directory.
src/http/tests/enforces_agent_delete_permissions.rs - Tests that file deletion occurs only when both required deletion permissions are enabled.
src/http/tests/flow.rs - Creates a flow for the specified directory and returns its flow ID.
src/http/tests/handles_agent_edits.rs - Tests that an HTTP agent run appends text to a file and returns the expected result.
src/http/tests/handles_agent_requests.rs - Tests that the agent run endpoint returns output and records it in the flow transcript.
src/http/tests/handles_git_add_requests.rs - Tests that `/v1/git/add` stages a file in the specified Git repository directory.
src/http/tests/handles_git_status_requests.rs - Tests that `/v1/git/status` excludes files ignored by `.treeignore`.
src/http/tests/handles_tree_requests.rs - Tests that `POST /v1/tree` returns repository paths while excluding files listed in `.treeignore`.
src/http/tests/logs_provider_token_usage.rs - Tests that provider token usage is logged while API responses omit token counts and conversation IDs.
src/http/tests/openapi_is_current.rs - This file contains no production behavior; it only tests the OpenAPI document.
src/http/tests/persists_conversation_tool_context.rs - Verifies that tool output persists and is available across requests in the same conversation.
src/http/tests/preserves_literal_template_input.rs - Verifies the HTTP endpoint passes template-like input to the agent unchanged.
src/http/tests/rejects_invalid_requests.rs - Tests HTTP routes for expected responses to valid requests, unknown paths, unsupported methods, and malformed POSTs.
src/http/tests/rejects_unauthorized_agent_question.rs - Tests that an agent question without permission is rejected with an HTTP 500 error.
src/http/tests/rejects_unauthorized_agent_read.rs - Tests that file-read requests without permission return HTTP 500 with the expected error.
src/http/tests/request.rs - Builds a JSON HTTP request, optionally adds a flow ID header, and dispatches it through the application router.
src/http/tests/resumes_agent_conversation.rs - Tests resuming an agent conversation from persisted messages.
src/http/tests/returns_agent_question.rs - Tests that an authorized agent question is returned by the HTTP endpoint and recorded in the flow transcript and conversation history.
src/input.rs - Reads user responses and confirmations from the interactive terminal.
src/interfaces/harness.rs - Defines the interfaces for adapting run requests into CLI invocations and wrapping output as responses.
src/interfaces/harness/display.rs - Formats `HarnessError` variants as readable messages, including the adapter and available details.
src/interfaces/history.rs - Defines serializable snapshot and content block types for recording agent activity and file operations.
src/interfaces/history/marker.rs - Returns the fixed text marker associated with each history block variant.
src/interfaces/history/text.rs - Returns a reference to the text stored in a history block.
src/interfaces/input.rs - Defines the shared interface for asking the user questions and awaiting confirmations.
src/interfaces/invocation.rs - Prepends a command to an invocation while preserving the original program and execution settings.
src/interfaces/mod.rs - Defines and re-exports shared interface types for use across application modules.
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
src/tools/edit.rs - Defines edit request and pending state types, re-exporting edit display and version functions.
src/tools/edit/apply_to.rs - Applies validated edit requests to text, handling writes, replacements, and line-based edits.
src/tools/edit/commit.rs - Commits a validated prepared edit to its target file and returns the resulting diff.
src/tools/edit/diff.rs - Generates a unified diff between original and edited contents to present prepared changes.
src/tools/edit/display.rs - Renders and prints a diff, using colors when stdout is a terminal and `NO_COLOR` is unset.
src/tools/edit/pending_validate.rs - Validates a pending edit record and returns the first error if its contents or operation are invalid.
src/tools/edit/prepare.rs - Resolves an edit target and validates the operation against its current contents.
src/tools/edit/read_optional.rs - Reads a UTF-8 file and returns its contents, absence, or a read error.
src/tools/edit/render.rs - Renders diffs, highlighting additions and removals with colors and escaping content control characters.
src/tools/edit/request_validate.rs - Validates edit request paths, operation-specific fields, and optional SHA-256 versions.
src/tools/edit/sync_parent.rs - Synchronizes the parent directory of a path to disk and converts failures into messages.
src/tools/edit/target.rs - Resolves edit target paths and rejects invalid paths or targets outside the execution directory.
src/tools/edit/tests.rs - Builds a file edit request using the file’s prior contents to compute its version.
src/tools/edit/tests/append_and_prepend_are_exact_and_versions_are_checked.rs - Validates append and prepend operations, including empty input, and detects version conflicts.
src/tools/edit/tests/diff_and_color_rendering_preserve_plain_results.rs - Tests diff generation and rendering, including colors, escapes, and missing final newlines.
src/tools/edit/tests/line_edits_preserve_bytes_and_handle_eof.rs - The file’s documented behavior was not provided, so I can’t summarize its purpose.
src/tools/edit/version.rs - Calculates the SHA-256 hash of a string's UTF-8 bytes and returns a lowercase hexadecimal digest.
src/tools/execute_with_input.rs - Executes TREE or READ in the given directory and returns the result, reporting errors for failed or unknown operations.
src/tools/format_paths.rs - Formats file paths as a pretty-printed JSON array, reporting conversion and serialization errors with the tool name.
src/tools/mod.rs - Organizes agent tools and re-exports execution and request operations.
src/tools/new_agent.rs - Creates and saves a validated agent definition from user input and generated runtime metadata.
src/tools/new_agent/adapter.rs - Prompts for an available adapter and repeats the question if the input does not match.
src/tools/new_agent/answer.rs - Repeats prompts until receiving a nonempty value and propagates interaction errors.
src/tools/new_agent/create.rs - Generates an agent in the supplied directory and returns the saved path.
src/tools/read.rs - Exposes file-reading and pagination functions for external use.
src/tools/read/allowed.rs - Validates that a requested path is nonempty, safely contained, resolvable, and not ignored.
src/tools/read/component.rs - Compares a pattern and a value as character sequences and returns whether they match.
src/tools/read/component_match.rs - Compares character sequences with `*` and `?` wildcards and reports whether they match.
src/tools/read/components.rs - Compares pattern and path components, supporting `**` and prefix matching.
src/tools/read/enumerate.rs - enumerate numbers file content lines from one and returns the formatted text.
src/tools/read/enumerate_from.rs - Formats content lines with numbered prefixes starting at a supplied offset and returns them after a header.
src/tools/read/enumerated_content.rs - Reconstructs original contents from numbered output by removing numbers and joining valid lines.
src/tools/read/ignore.rs - Determines whether a path matches an ignore pattern by comparing its components.
src/tools/read/ignored.rs - Checks whether a path matches exclusion rules defined in `.readignore`.
src/tools/read/page.rs - Reads and returns a requested line range from a file under the specified directory.
src/tools/read/page_tests.rs - The file contains only tests and has no production function to summarize.
src/tools/read/request.rs - Parses READ input into a file path, line offset, and limit, returning validation errors as strings.
src/tools/read/run.rs - Reads UTF-8 files inside the execution directory while blocking invalid or ignored paths.
src/tools/request.rs - Parses file tree and read requests, identifying the request type and requested path.
src/tools/tree.rs - Represents a file tree root and its sorted, deduplicated relative paths.
src/tools/tree/list.rs - Lists existing, non-ignored files under a Git working directory.
src/tools/tree/path.rs - Converts bytes to a `PathBuf`, preserving them on Unix and requiring UTF-8 on other platforms.