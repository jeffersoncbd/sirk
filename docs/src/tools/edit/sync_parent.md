## Resumo
Sincroniza (fsync) o diretório pai de um caminho, garantindo persistência de uma entrada no disco.

## Funcionamento
Abre o diretório contido em `path.parent()` — panicando se o caminho não tiver pai — e chama `sync_all()` para forçar a gravação de metadados. Erros de I/O são convertidos em `String` e propagados via `Result`,APE sem valor de retorno em caso de sucesso.

## Importações
- `std::fs::File`: Abre o diretório pai para permitir a sincronização dos metadados.
- `std::path::Path`: Representa o caminho alvo, fornecendo o diretório pai a sincronizar.
