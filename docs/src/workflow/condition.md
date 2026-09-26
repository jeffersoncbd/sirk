## Resumo
Converte texto em `bool`, aceitando apenas `"true"` ou `"false"` (após `trim`).

## Funcionamento
A função aplica `trim()` na entrada e compara com as literais exatas: correspondências viram `Ok(true)`/`Ok(false)`; qualquer outro valor resulta em `Err` com a mensagem `"IF input must be true or false"`. Não há efeitos colaterais; a diferença entre `Result` e `Option`、集中 na mensagem de erro, sinaliza entrada inválida vinda de um workflow.

## Importações
- Nenhuma: apenas tipos da biblioteca padrão (`&str`, `bool`, `Result`, `String`).
