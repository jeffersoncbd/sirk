## Resumo
Verifica se um bloco `Delete` de uma requisição externa já foi concluído, detectando a sequência Output/Input/Delete no histórico.

## Funcionamento
Percorre o histórico em janelas de 3 blocos; para cada janela, faz pattern matching exigindo `Output` seguido de `Input` e `Delete`. Quando o padrão casa, compara a requisição armazenada no output anterior com a requisição atual via `is_some_and(|previous| previous.as_ref() == Ok(request))`, retornando `true` na primeira correspondência e `false` caso contrário. Nenhum efeito colateral: apenas leitura e comparação do slice `blocks`.

## Importações
- `ExternalDeleteRequest`: tipo da requisição de exclusão externa, usado na comparação dos valores.
- `external_delete_request`: extrai do `Output` a requisição externa armazenada para comparação.
- `Block`: enum de blocos do histórico que define o padrão Output/Input/Delete.
