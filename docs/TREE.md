.env.example - Documenta variáveis de ambiente para configurar integrações com os serviços Ollama e OpenRouter.
Cargo.toml - Configura a crate Rust, seus alvos de compilação, metadados e dependências externas.
src/adapters/codex.rs - Adapta requisições genéricas do harness para invocações seguras do comando `codex exec`.
src/adapters/mod.rs - Registra, reexporta e resolve adaptadores de harness por nome, listando as opções disponíveis.
src/adapters/ollama.rs - Adapta requisições genéricas do harness para invocações válidas do comando Ollama.
src/adapters/ollama_web.rs - Adapta solicitações do harness para chamadas HTTP ao Ollama via curl e extrai o texto gerado.
src/adapters/opencode.rs - Adapta requisições do harness para comandos seguros e executáveis pela CLI do OpenCode.
src/adapters/openrouter.rs - Implementa o adaptador OpenRouter, criando requisições curl e interpretando respostas JSON da API.
src/agents.rs - Define, valida e carrega agentes a partir de arquivos Markdown com metadados YAML e permissões.
src/harness.rs - Define a abstração comum para adaptar CLIs de agentes, construir execuções e tratar suas respostas e erros.
src/history.rs - Gerencia a criação, leitura, gravação e recuperação persistente dos históricos de execução em formato v2.
src/input.rs - Define a interface síncrona de entrada do usuário e implementa perguntas e confirmações via terminal.
src/lib.rs - Organiza e expõe publicamente os principais módulos da crate Rust, sem implementar lógica de negócio.
src/main.rs - Interpreta comandos da CLI para criar agentes, executar workflows e retomar históricos.
src/runner.rs - Expõe a API pública de execução e retomada de workflows, delegando a lógica ao motor interno.
src/runner/bootstrap.rs - Inicializa, valida e retoma execuções de workflows, preparando agentes, histórico e diretório de trabalho.
src/runner/engine.rs - Organiza os submódulos do mecanismo de workflows e reexporta suas funções públicas de execução.
src/runner/execution.rs - Executa workflows sequencialmente, gerenciando histórico, ferramentas, agentes, loops, condições e entradas do usuário.
src/runner/external.rs - Interpreta e valida solicitações externas de leitura, edição e exclusão de arquivos no histórico.
src/runner/history_validation.rs - Valida estruturalmente transcripts de workflows, verificando steps, blocos, ferramentas, agentes e interações.
src/runner/tests.rs - Fornece fixtures e utilitários compartilhados para testar fluxos de execução, conversas e arquivos.
src/runner/tests/control_flow.rs - Testa o controle de fluxo, retomada e persistência de workflows com IF, LOOP e WRITE.
src/runner/tests/conversations.rs - Testa a execução, persistência e retomada de workflows com agentes e ferramentas READ e TREE.
src/runner/tests/edits.rs - Testa a execução, edição, interação, recuperação e retomada de workflows com arquivos temporários.
src/runner/tests/files.rs - Testa operações de arquivos em workflows, incluindo edição, escrita, exclusão, recuperação e permissões.
src/services/bash.rs - Executa processos Bash com argumentos seguros, transmite e captura suas saídas textual ou binária.
src/services/mod.rs - Define invocações estruturadas e expõe os serviços e resultados da execução de comandos Bash.
src/tools/custom.rs - Executa scripts Bash personalizados do diretório `tools`, validando caminhos e argumentos com segurança.
src/tools/delete.rs - Remove arquivos regulares com segurança, validando caminhos dentro do diretório de execução.
src/tools/edit.rs - Implementa edição segura de arquivos em workflows, validando versões, gerando diffs e confirmando alterações.
src/tools/git_status_tree.rs - Descobre, filtra e ordena arquivos alterados no Git, retornando caminhos relativos elegíveis.
src/tools/mod.rs - Centraliza o reconhecimento, despacho e formatação das ferramentas de leitura e listagem do workflow.
src/tools/new_agent.rs - Cria interativamente arquivos de definição de agentes, validando seu conteúdo antes de salvá-los.
src/tools/read.rs - Lê arquivos UTF-8 com regras de exclusão e enumera ou reconstitui seu conteúdo linha a linha.
src/tools/tree.rs - Enumera arquivos rastreados e não ignorados de uma árvore Git, aplicando filtros e retornando caminhos únicos.
src/tools/write.rs - Implementa a operação WRITE, criando arquivos UTF-8 com validação segura de caminhos e opções de sobrescrita.
src/workflow.rs - Define modelos, valida e renderiza workflows YAML declarativos com agentes, ferramentas, condicionais e loops.
