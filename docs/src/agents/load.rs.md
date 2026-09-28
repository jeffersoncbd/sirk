## Resumo
Carrega e valida um agente a partir de um arquivo Markdown.

## Funcionamento
Rejeita IDs inválidos, lê `{id}.md` no diretório informado e converte falhas de leitura ou análise em mensagens de erro. Retorna o agente analisado ou um `String` com o motivo da falha.

## Importações
- `super::Agent`: Tipo de agente carregado.
- `super::valid_id`: Valida o identificador do agente.
- `std::fs`: Lê o conteúdo do arquivo.
- `std::path::Path`: Representa o diretório de origem.
