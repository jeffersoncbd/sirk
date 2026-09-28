## Resumo
Sincroniza em disco o diretório pai do caminho informado.

## Funcionamento
Obtém o diretório pai, abre-o e chama `sync_all`; converte falhas de abertura ou sincronização em `String`. Espera que o caminho tenha diretório pai.

## Importações
- `std::fs::File`: abre o diretório para sincronização.
- `std::path::Path`: representa o caminho recebido.
