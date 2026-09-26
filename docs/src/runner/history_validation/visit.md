## Resumo
Percorre os passos do workflow contra o histórico gravado, validando blocos e labels, e informa se o histórico ainda cobre todos os passos.

## Funcionamento
A função itera os `steps` mantendo um `cursor` sobre `history.steps`: a cada posição monta o id (`prefix` + número) e o label `Step {id} — {nome}`; se o bloco do histórico não existir, retorna `Ok(false)` (fim do histórico), e se o label salvo divergir, retorna `Err` orientando a remoção dos blocos posteriores. O cursor é advances e, conforme a ferramenta, delega a validação: `LOOP` e `IF` exigem um único bloco `Input` e recursam em `visit` para cada iteração/branch; `EDIT`, `ASK` e passos com agente (usando o snapshot de agente) usam os validadores correspondentes; demais passos usam `validate_step_blocks` aceitando predicado extra. `Ok(false)` em qualquer subnível aborta a validação; `Ok(true)` indica casamento completo. Erros de estrutura (blocos inesperados, condição/loop inválidos, agente ausente) são propagados como `String`.

## Importações
- `super::agent_blocks::validate_agent_blocks`: Valida blocos de agente com suas ferramentas de edição/exclusão.
- `super::ask_blocks::validate_ask_blocks`: Valida blocos Ask/Answer do passo.
- `super::edit_blocks::validate_edit_blocks`: Valida blocos de edição do passo.
- `super::step_blocks::validate_step_blocks`: Valida blocos comuns de um passo.
- `crate::history::{Block, History}`: Fornece a estrutura do histórico e seus tipos de bloco.
- `crate::workflow::{Step, condition, loop_items}`: Passo do workflow e avaliação de condição/itens de loop.
