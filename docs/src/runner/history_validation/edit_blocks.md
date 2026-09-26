## Resumo
Valida se uma entrada de histórico do tipo EDIT está corretamente formada e já executada.

## Funcionamento
Retorna `Ok(false)` se não houver blocos. O primeiro bloco precisa ser `Block::Input` cujo texto seja um `Pending` desserializado de JSON (falha de desserialização vira `Err` com o erro original); em seguida o próprio `pending` é validado. Sem blocos posteriores, o registro está apenas preparado (`Ok(false)`). Com um único `Block::Output`, a tentativa só é bem-sucedida se o diff de saída for idêntico ao diff calculado pelo `Pending` a partir de `before` (`Ok(true)`). Qualquer outra combinação (saída divergente, múltiplos blocos, tipo inesperado) retorna `Err` orientando a remoção do resultado e dos registros seguintes.

## Importações
- `crate::history::Block`: enum dos blocos que compõem uma entrada de histórico
- `crate::tools::edit::Pending`: estado de edição preparado, usado para validar e reproduzir o diff
- `serde_json`: desserializa o `Pending` a partir do texto do bloco de entrada
