Cargo.toml - Define metadados, biblioteca, executável e dependências da crate Rust `new-harness`.
src/adapters/codex.rs - Adapta requisições genéricas para comandos seguros de execução do harness Codex.
src/agents.rs - Define, carrega e valida agentes Rust a partir de arquivos Markdown com metadados YAML.
src/harness.rs - Define contratos comuns para adaptar CLIs de agentes e representar execuções e erros de compatibilidade.
src/history.rs - Gerencia o armazenamento, leitura e persistência atômica do histórico de execuções em transcripts v2.
src/input.rs - Abstrai a entrada do usuário e implementa leitura interativa via terminal, com respostas e cancelamento.
src/lib.rs - Organiza e expõe publicamente os principais módulos internos da crate Rust.
src/main.rs - Ponto de entrada da CLI, responsável por executar e retomar workflows e criar agentes interativamente.
src/runner.rs - Executa workflows sequenciais com agentes, ferramentas e loops, persistindo histórico para retomada.
src/services/bash.rs - Executa comandos externos via Bash, preservando argumentos, diretório, saída e status do processo.
src/tools/custom.rs - Executa scripts Bash locais com argumentos validados e retorna o `stdout` de execuções bem-sucedidas.
src/tools/edit.rs - Implementa edições seguras em arquivos, validando versões, gerando diffs e persistindo alterações controladas.
src/tools/new_agent.rs - Cria, valida e salva novos agentes Markdown a partir de configurações do usuário e geração automatizada.
src/tools/read.rs - Lê com segurança arquivos regulares UTF-8 dentro do diretório permitido, rejeitando acessos inválidos.
src/tools/tree.rs - Lista arquivos de um repositório Git, respeitando regras de exclusão padrão e do `.treeignore`.
src/tools/write.rs - Cria ou substitui arquivos UTF-8 com segurança dentro do diretório de execução autorizado.
src/workflow.rs - Define, carrega e valida workflows YAML, incluindo etapas, loops, ferramentas e referências de saída.
