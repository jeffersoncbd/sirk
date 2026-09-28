## Resumo
Lê um arquivo UTF-8 se ele existir.

## Funcionamento
Retorna o conteúdo em `Some`, `None` se o arquivo não existir ou um erro descritivo se a leitura falhar.

## Importações
- `std::fs`: Lê o arquivo.
- `std::path::Path`: Representa o caminho do arquivo.
