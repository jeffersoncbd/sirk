## Resumo
Valida e prepara uma edição de arquivo externo, garantindo que o arquivo existe e que a faixa operada está dentro do conteúdo.

## Funcionamento
Lê o arquivo alvo via `crate::tools::read::read`; se a leitura falhar, o erro é propagado como `String`. Conta as linhas (preservando a última linha sem `\n`) e aplica duas regras: `Insert` com `line` maior que `line_count + 1` retorna erro, pois ultrapassa o EOF; `Delete`/`Replace` com `end` maior que `line_count` também retornam erro. Em seguida, registra em `request.version` o hash do conteúdo lido, monta um `Pending` com o texto original (`before`) e `was_missing: false`, chama `pending.validate()` e devolve `Ok(pending)`. Nenhum conteúdo é escrito ao disco.

## Importações
- `std::path::Path`: Recebe o diretório base usado para resolver o arquivo alvo.
