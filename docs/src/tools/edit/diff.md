## Resumo
Gera um diff unificado entre o conteúdo original e o resultado da aplicação de uma edição pendente.

## Funcionamento
Sem estado `before` (edição não preparada), retorna `Err("EDIT is not prepared")`. Em caso contrário, aplica a requisição sobre o texto original propagando o erro do `apply_to`. O cabeçalho usa `/dev/null` como arquivo de origem quando o arquivo não existia antes (criação), caso contrário o caminho depurado. O conteúdo não é escapado; apenas caracteres de controle do caminho são escapados pelo `{:?}`. Contexto de 3 linhas.

## Importações
- `super::Pending`: struct da edição pendente (`before`, `request`, `was_missing`)
- `similar`: `TextDiff` para gerar o diff unificado textual
