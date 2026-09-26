## Resumo
Cria um `CodexAdapter` vinculado a um executável configurável.

## Funcionamento
Converte o argumento em `String` via `Into` e o grava no campo `executable` do struct. Sem validações, `Result`, `Option` ou efeitos colaterais: a construção nunca falha.

## Importações
- `CodexAdapter`: Tipo do struct que recebe o caminho do executável.
