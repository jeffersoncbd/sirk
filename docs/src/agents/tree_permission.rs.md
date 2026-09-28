## Resumo
Converte o valor `allow` em `true` durante a desserialização.

## Funcionamento
Desserializa a entrada como texto e retorna `Ok(true)` somente se ela for igual a `allow`; qualquer outro valor resulta em erro informando que `TREE_TOOL` deve ser `allow`.

## Importações
- `serde`: desserializa a entrada e cria o erro personalizado.
