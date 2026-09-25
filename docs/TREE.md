Cargo.toml - Define metadados, biblioteca, executável e dependências do projeto Rust `new-harness`.
src/adapters/codex.rs - Adapta requisições genéricas para comandos seguros de execução do harness Codex.
src/agents.rs - Define, carrega e valida configurações de agentes Rust em arquivos Markdown com front matter YAML.
src/harness.rs - Define contratos comuns para adaptar CLIs de agentes, padronizando execuções e erros de compatibilidade.
src/history.rs - Gerencia o armazenamento, leitura e persistência atômica do histórico de execuções da aplicação.
src/input.rs - Define abstração de entrada do usuário e implementação terminal para leitura, validação e cancelamento.
src/lib.rs - Expõe e organiza publicamente os principais módulos internos da crate Rust.
src/main.rs - Gerencia a CLI do new-harness, executando workflows, retomando históricos e criando agentes.
src/runner.rs - Executa workflows com agentes, ferramentas e loops, persistindo o histórico para retomada segura.
src/services/bash.rs - Executa comandos externos via Bash, preservando argumentos, diretório, saída padrão e status de encerramento.
src/tools/custom.rs - Executa custom tools Bash locais com validação segura, argumentos posicionais e retorno do stdout.
src/tools/new_agent.rs - Cria agentes Markdown standalone, validando sua configuração e salvando-os sem sobrescrever existentes.
src/tools/read.rs - Lê arquivos UTF-8 regulares dentro de um diretório permitido, bloqueando caminhos inseguros.
src/tools/tree.rs - Lista arquivos de um repositório Git, respeitando regras de exclusão padrão e do `.treeignore`.
src/tools/write.rs - Implementa a ferramenta WRITE para criar ou substituir arquivos UTF-8 com validação de caminhos e sobrescrita controlada.
src/workflow.rs - Define, carrega e valida workflows YAML, incluindo etapas, loops, ferramentas e referências de saída.
