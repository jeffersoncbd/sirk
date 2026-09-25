Cargo.toml - Configura o pacote Rust, sua biblioteca, executável e dependências para serialização, hashing e comparação de textos.
src/adapters/codex.rs - Adapta requisições genéricas para executar o CLI Codex com opções seguras e somente leitura.
src/agents.rs - Define, valida e carrega agentes a partir de arquivos Markdown com metadados YAML e instruções.
src/harness.rs - Define contratos comuns para executar harnesses de coding agents e representar erros de configuração.
src/history.rs - Gerencia a criação, leitura, gravação e bloqueio de históricos persistentes de execução no formato v2.
src/input.rs - Define a interface de entrada do usuário e lê respostas do terminal, validando cancelamentos e entradas vazias.
src/lib.rs - Organiza e expõe publicamente os principais módulos da crate Rust, sem implementar lógica de negócio.
src/main.rs - Gerencia a CLI do new-harness, iniciando sessões, executando workflows e retomando históricos.
src/runner.rs - Executa workflows com agentes e ferramentas, persistindo o histórico e permitindo retomar execuções.

src/services/bash.rs - Executa comandos Bash com argumentos protegidos, transmitindo e capturando saídas textuais ou binárias.
src/tools/custom.rs - Executa com segurança scripts Bash locais, validando caminhos, argumentos e status, e captura sua saída.
src/tools/edit.rs - Implementa edição segura de arquivos em workflows, com validação, versionamento, diffs e commits atômicos.
src/tools/git_status_tree.rs - Lista arquivos existentes no status do Git, aplicando exclusões e retornando caminhos relativos ordenados.

src/tools/new_agent.rs - Cria interativamente definições de agentes, gera suas instruções, valida metadados e grava arquivos Markdown.
src/tools/read.rs - Valida caminhos dentro da raiz permitida e lê arquivos UTF-8, retornando conteúdo ou erros descritivos.
src/tools/tree.rs - Lista arquivos rastreados e não ignorados da árvore Git, aplicando regras de exclusão e ordenação.
src/tools/write.rs - Implementa a operação WRITE, criando arquivos UTF-8 com validação segura de caminhos e opções de sobrescrita.
src/workflow.rs - Modela, valida e renderiza workflows YAML declarativos, incluindo etapas, escopos, outputs e edições.

