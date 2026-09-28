## Resumo
Valida a consistência de um registro `Pending` antes de aceitá-lo.

## Funcionamento
Valida a requisição e retorna seu erro, se houver. Se o arquivo estava ausente, exige conteúdo anterior vazio e operação `Append` ou `Prepend`; quando há conteúdo anterior, verifica se a operação pode ser aplicada a ele. Retorna `Ok(())` se todas as verificações passarem.

## Importações
- `super`: Fornece `Operation` e `Pending`.
