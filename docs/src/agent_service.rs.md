## Resumo
Expõe `run::run` como ponto de entrada da execução de agentes.

## Funcionamento
Declara os módulos internos do serviço e reexporta `run` com visibilidade restrita à crate; não define uma função neste arquivo.

## Importações
- `conversation`: Módulo interno de conversa.
- `delete_request`: Módulo interno para pedidos de exclusão.
- `edit_request`: Módulo interno para pedidos de edição.
- `execute`: Módulo interno de execução.
- `prompt`: Módulo interno de prompts.
- `run::run`: Função reexportada para uso na crate.
