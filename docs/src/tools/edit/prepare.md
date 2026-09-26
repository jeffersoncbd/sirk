## Resumo
Prepara uma operação de edição validando o arquivo alvo e armazenando seu conteúdo anterior.

## Funcionamento
Permite arquivo ausente apenas para `Append`/`Prepend`; resolve o caminho via `target`, lê o conteúdo com `read_optional` e, se o arquivo não existir e a operação for de linhas, retorna `Err`. Aplica a operação sobre o conteúdo (vazio por padrão) para validar, então registra `was_missing` e `before` no `Pending`.

## Importações
- `super::{Operation, Pending, ...}`: Tipos e helpers do módulo `edit` (operação, estado e leitura/caminho).
- `std::path::Path`: Representa o diretório base usado na resolução do arquivo alvo.
