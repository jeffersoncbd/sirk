Cargo.toml - Configura a crate Rust, seus alvos de compilação, metadados e dependências externas.




src/adapters/codex.rs - Adapta requisições genéricas do harness para invocações seguras do comando `codex exec`.

src/agents.rs - Define, valida e carrega agentes a partir de arquivos Markdown com metadados YAML e instruções.
src/harness.rs - Define a abstração comum para adaptar CLIs de agentes, construir execuções e tratar suas respostas e erros.


src/history.rs - Gerencia a criação, leitura, gravação e bloqueio de históricos persistentes de execução no formato v2.
src/input.rs - Define a interface de entrada do usuário e lê respostas do terminal, validando cancelamentos e entradas vazias.
src/lib.rs - Organiza e expõe publicamente os principais módulos da crate Rust, sem implementar lógica de negócio.
src/main.rs - Gerencia a CLI do new-harness, iniciando sessões, executando workflows e retomando históricos.
src/runner.rs - Coordena a execução, retomada e persistência de workflows sequenciais com agentes e ferramentas.

src/services/bash.rs - Executa processos Bash com argumentos seguros, transmite e captura suas saídas textual ou binária.

src/tools/custom.rs - Executa scripts Bash personalizados do diretório `tools`, validando caminhos e argumentos com segurança.

src/tools/edit.rs - Implementa edição segura de arquivos em workflows, com validação, versionamento, diffs e commits atômicos.
src/tools/git_status_tree.rs - Descobre e filtra arquivos alterados no Git, aplicando exclusões e retornando caminhos relativos únicos.

src/tools/new_agent.rs - Cria interativamente arquivos de definição de agentes, validando seu conteúdo antes de salvá-los.

src/tools/read.rs - Lê arquivos UTF-8 dentro do diretório permitido, aplicando validações de segurança e regras do `.readignore`.

src/tools/tree.rs - Enumera arquivos rastreados e não ignorados de uma árvore Git, aplicando filtros e retornando caminhos únicos.

src/tools/write.rs - Implementa a operação WRITE, criando arquivos UTF-8 com validação segura de caminhos e opções de sobrescrita.
src/workflow.rs - Modela, valida e renderiza workflows YAML declarativos, incluindo etapas, escopos, outputs e edições.
src/adapters/opencode.rs - Adapta requisições do harness para comandos seguros e executáveis pela CLI do OpenCode.

src/adapters/ollama.rs - Adapta requisições genéricas do harness para invocações válidas do comando Ollama.


.env.example - Documenta variáveis de ambiente para configurar integrações com os serviços Ollama e OpenRouter.









src/adapters/ollama_web.rs - Adapta solicitações do harness para chamadas HTTP ao Ollama via curl e extrai o texto gerado.

src/adapters/openrouter.rs - Implementa o adaptador OpenRouter, criando requisições curl e interpretando respostas JSON da API.

