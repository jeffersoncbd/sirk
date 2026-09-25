### Resumo

Este arquivo funciona como o ponto de entrada da crate Rust para organizar e expor os principais módulos do projeto. Ele declara nove módulos públicos, permitindo que sejam acessados por outras partes da crate ou por crates externas.

### Funcionamento

Cada instrução `pub mod` associa um módulo ao arquivo ou diretório correspondente, normalmente seguindo a convenção de nomes do Rust. A declaração `pub` torna o módulo público.

O arquivo não implementa lógica de negócio diretamente; sua responsabilidade é estrutural, compondo a API modular da crate.

### Componentes principais

- `adapters`: adaptadores para integração com diferentes componentes ou implementações.
- `agents`: definição e gerenciamento de agentes.
- `harness`: funcionalidades relacionadas ao harness de execução.
- `history`: gerenciamento de histórico ou transcrições.
- `input`: tratamento de entradas do usuário ou do sistema.
- `runner`: execução dos fluxos ou tarefas principais.
- `services`: serviços auxiliares e integrações externas.
- `tools`: ferramentas disponibilizadas aos agentes ou ao fluxo de execução.
- `workflow`: definição e processamento de workflows.

### Dependências e integrações

O arquivo depende implicitamente da estrutura de módulos da própria crate. Para que essas declarações funcionem, devem existir arquivos como `adapters.rs` ou diretórios como `adapters/mod.rs`, e o mesmo se aplica aos demais módulos.

Não há dependências externas, imports, funções ou tipos definidos diretamente neste conteúdo.

### Observações

O comportamento interno de cada módulo não pode ser determinado apenas por este arquivo. Ele atua essencialmente como um índice público e ponto de organização da crate.
