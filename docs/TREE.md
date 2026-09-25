# Documentation index

Trusted navigation map for the source and its module documentation. Read only
entries relevant to the task; module documents are not required session context.
For `src/foo/bar.rs`, the document is `docs/src/foo/bar.md`.

- [User manual](../README.md)
- [Workflow reference](REFERENCE.md): syntax, tools, examples, and recovery.
- [Development instructions](../AGENTS.md): workflow and checks.
- [Development contracts](CONTRACTS.md): compatibility requirements by subsystem.

## Source files

- [Cargo.toml](../Cargo.toml) — [Documentation](Cargo.md): Define metadados, biblioteca, executável e dependências da crate Rust `new-harness`.
- [src/adapters/codex.rs](../src/adapters/codex.rs) — [Documentation](src/adapters/codex.md): Adapta requisições genéricas para comandos seguros de execução do harness Codex.
- [src/agents.rs](../src/agents.rs) — [Documentation](src/agents.md): Define, carrega e valida agentes Rust a partir de arquivos Markdown com metadados YAML.
- [src/harness.rs](../src/harness.rs) — [Documentation](src/harness.md): Define contratos comuns para adaptar CLIs de agentes e representar execuções e erros de compatibilidade.
- [src/history.rs](../src/history.rs) — [Documentation](src/history.md): Gerencia o armazenamento, leitura e persistência atômica do histórico de execuções em transcripts v2.
- [src/input.rs](../src/input.rs) — [Documentation](src/input.md): Abstrai a entrada do usuário e implementa leitura interativa via terminal, com respostas e cancelamento.
- [src/lib.rs](../src/lib.rs) — [Documentation](src/lib.md): Organiza e expõe publicamente os principais módulos internos da crate Rust.
- [src/main.rs](../src/main.rs) — [Documentation](src/main.md): Ponto de entrada da CLI, responsável por executar e retomar workflows e criar agentes interativamente.
- [src/runner.rs](../src/runner.rs) — [Documentation](src/runner.md): Executa workflows sequenciais com agentes, ferramentas e loops, persistindo histórico para retomada.
- [src/services/bash.rs](../src/services/bash.rs) — [Documentation](src/services/bash.md): Executa comandos externos via Bash, preservando argumentos, diretório, saída e status do processo.
- [src/tools/custom.rs](../src/tools/custom.rs) — [Documentation](src/tools/custom.md): Executa scripts Bash locais com argumentos validados e retorna o `stdout` de execuções bem-sucedidas.
- [src/tools/edit.rs](../src/tools/edit.rs) — [Documentation](src/tools/edit.md): Implementa edições seguras em arquivos, validando versões, gerando diffs e persistindo alterações controladas.
- [src/tools/new_agent.rs](../src/tools/new_agent.rs) — [Documentation](src/tools/new_agent.md): Cria, valida e salva novos agentes Markdown a partir de configurações do usuário e geração automatizada.
- [src/tools/read.rs](../src/tools/read.rs) — [Documentation](src/tools/read.md): Lê com segurança arquivos regulares UTF-8 dentro do diretório permitido, rejeitando acessos inválidos.
- [src/tools/tree.rs](../src/tools/tree.rs) — [Documentation](src/tools/tree.md): Lista arquivos de um repositório Git, respeitando regras de exclusão padrão e do `.treeignore`.
- [src/tools/write.rs](../src/tools/write.rs) — [Documentation](src/tools/write.md): Cria ou substitui arquivos UTF-8 com segurança dentro do diretório de execução autorizado.
- [src/workflow.rs](../src/workflow.rs) — [Documentation](src/workflow.md): Define, carrega e valida workflows YAML, incluindo etapas, loops, ferramentas e referências de saída.
