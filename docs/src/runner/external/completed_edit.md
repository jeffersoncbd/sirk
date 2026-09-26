## Resumo
Verifica se um `Request` de edição já foi executado anteriormente no histórico de blocos.

## Funcionamento
Percorre a sequência `blocks` com `windows(3)`, procurando a tríade `Output` → `Input` → `Edit`. Quando o padrão casa, o `Output` anterior é interpretado como resposta de uma requisição de edição externa via `external_edit_request`; a comparação com `Ok(request)` usa `is_some_and`, garantindo que só importa um pedido idêntico já concluído com sucesso. Qualquer janela fora do padrão é descartada, e a função retorna `true` na primeira correspondência (curto-circuito).

## Importações
- `super::external_edit_request`: Converte um `Block::Output` em requisição de edição opcional.
- `crate::history::Block`: Enum dos blocos do histórico; define o padrão Output/Input/Edit.
