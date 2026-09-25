.env.example - Documenta variáveis de ambiente para configurar integrações com os serviços Ollama e OpenRouter.
Cargo.toml - Configura a crate Rust, seus alvos de compilação, metadados e dependências externas.
src/adapters/codex.rs - Adapta requisições genéricas do harness para invocações seguras do comando `codex exec`.
src/adapters/mod.rs - Registra, reexporta e resolve adaptadores de harness por nome, listando as opções disponíveis.
src/adapters/ollama.rs - Adapta requisições genéricas do harness para invocações válidas do comando Ollama.
src/adapters/ollama_web.rs - Adapta solicitações do harness para chamadas HTTP ao Ollama via curl e extrai o texto gerado.
src/adapters/opencode.rs - Adapta requisições do harness para comandos seguros e executáveis pela CLI do OpenCode.
src/adapters/openrouter.rs - Implementa o adaptador OpenRouter, criando requisições curl e interpretando respostas JSON da API.
src/agents.rs - Define, valida e carrega agentes a partir de arquivos Markdown com metadados YAML e permissões configuráveis.
src/harness.rs - Define a abstração comum para adaptar CLIs de agentes, construir execuções e tratar suas respostas e erros.
src/history.rs - Gerencia a criação, leitura, gravação e recuperação persistente dos históricos de execução em formato v2.
src/input.rs - Define a interface síncrona de entrada do usuário e implementa perguntas e confirmações via terminal.
src/lib.rs - Organiza e expõe publicamente os principais módulos da crate Rust, sem implementar lógica de negócio.
src/main.rs - Ponto de entrada que interpreta comandos e executa ou retoma workflows em modo interativo ou direto.
src/runner.rs - Executa e retoma workflows, coordenando agentes, ferramentas, interações e histórico persistido.
src/services/bash.rs - Executa processos Bash com argumentos seguros, transmite e captura suas saídas textual ou binária.
src/services/mod.rs - Define invocações estruturadas e expõe os serviços e resultados da execução de comandos Bash.
src/tools/custom.rs - Executa scripts Bash personalizados do diretório `tools`, validando caminhos e argumentos com segurança.
src/tools/delete.rs - Remove arquivos regulares com segurança, validando caminhos dentro do diretório de execução.
src/tools/edit.rs - Implementa edição segura de arquivos em workflows, validando versões, gerando diffs e confirmando alterações.
src/tools/git_status_tree.rs - Descobre, filtra e ordena arquivos alterados no Git, retornando caminhos relativos elegíveis.
src/tools/mod.rs - Centraliza o reconhecimento, despacho e formatação das ferramentas de leitura e listagem do workflow.
src/tools/new_agent.rs - Cria interativamente arquivos de definição de agentes, validando seu conteúdo antes de salvá-los.
src/tools/read.rs - Lê arquivos UTF-8 dentro do diretório permitido, aplicando validações de segurança e regras do `.readignore`.
src/tools/tree.rs - Enumera arquivos rastreados e não ignorados de uma árvore Git, aplicando filtros e retornando caminhos únicos.
src/tools/write.rs - Implementa a operação WRITE, criando arquivos UTF-8 com validação segura de caminhos e opções de sobrescrita.
src/workflow.rs - Define modelos e valida regras, referências, escopos e edições de workflows YAML declarativos.
