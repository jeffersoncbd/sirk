### Resumo

Este arquivo funciona como ponto de entrada da crate, declarando e expondo publicamente os principais módulos do projeto Rust. Ele organiza a estrutura interna da aplicação e permite que outros módulos ou crates acessem esses componentes.

### Funcionamento

Cada declaração `pub mod` informa ao compilador que existe um módulo correspondente e o torna público:

- `adapters`
- `agents`
- `harness`
- `history`
- `input`
- `runner`
- `services`
- `tools`
- `workflow`

O arquivo não contém lógica de execução, funções, structs ou tratamento de erros. Sua responsabilidade é apenas registrar a composição modular da crate.

### Componentes principais

- `pub mod adapters;` — expõe os adaptadores de integração.
- `pub mod agents;` — expõe funcionalidades relacionadas aos agentes.
- `pub mod harness;` — expõe contratos ou abstrações do harness.
- `pub mod history;` — expõe gerenciamento de histórico.
- `pub mod input;` — expõe interfaces de entrada de dados.
- `pub mod runner;` — expõe a execução de workflows ou tarefas.
- `pub mod services;` — expõe serviços auxiliares da aplicação.
- `pub mod tools;` — expõe ferramentas usadas pelo sistema.
- `pub mod workflow;` — expõe estruturas e regras de workflows.

### Dependências e integrações

O arquivo integra os módulos internos da própria crate. Os detalhes das responsabilidades e dependências externas não estão presentes no conteúdo fornecido e dependem dos arquivos correspondentes.

### Observações

As declarações usam `pub`, portanto os módulos ficam acessíveis externamente à crate. O arquivo provavelmente corresponde a um módulo raiz, como `lib.rs` ou `main.rs`, mas o caminho não foi informado.
