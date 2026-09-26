## Resumo
Detecta se o histórico termina com um par `Output`/`Input` de edição externa e devolve o `Pending` correspondente.

## Funcionamento
Aplica um padrão de slice exigindo que os dois últimos blocos sejam `Output` e `Input`, nessa ordem; se não casar, retorna `None` (não há edição pendente). Quando casa, valida a requisição via `external_edit_request`: se ela falhar, o erro é propagado em `Some(Err)`; se passar, desserializa o JSON do input em `Pending`, convertendo falhas de serde em mensagem `"invalid EDIT_TOOL history: ..."`. Sem efeitos colaterais: apenas leitura do slice.

## Importações
- `super::external_edit_request`: Valida a requisição de edição antes de desserializar o pending.
- `crate::history::Block`: Enum dos blocos do histórico usado no pattern matching final.
- `serde_json`: Desserializa o input pendente de `Pending`.
