## Resumo
Valida se o bloco na posição atual do histórico corresponde à ferramenta que o usuário pediu.

## Funcionamento
Tenta extrair a chamada de ferramenta da mensagem do usuário via `crate::tools::request`; se não houver requisição recognizable, considera a mensagem válida sem consumir blocos. Para requisições, define a ferramenta esperada (`TREE` para árvores, `READ` no demais) e inspeciona o bloco em `*position`: ausência de bloco retorna `Ok(false)` (sem erro, apenas sem match), enquanto um tipo de bloco divergente produz `Err` com mensagem descritiva. Em caso de correspondência, avança a posição em 1 e retorna `Ok(true)`.

## Importações
- `crate::history::Block`: Representa os blocos do histórico, permitindo casar `Tree`/`Read` com a ferramenta esperada.
