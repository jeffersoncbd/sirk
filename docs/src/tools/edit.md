## Resumo
Expõe operações de edição de arquivo (insert, delete, replace, prepend, append) com verificação de versão e geração de diff.

## Funcionamento
`Request` descreve a mutação desejada (caminho, operação, linhas/alcance, versão esperada e conteúdo) e, via `apply_to`, aplica a mudança sobre o conteúdo atual, devolvendo `Result` com erro em divergência de versão, faixa de linhas inválida ou arquivo ausente. `Pending` guarda o `Request` junto ao conteúdo anterior (`before`/`was_missing`) para permitir pré-visualização: `diff` produz o diff unificado e `render` o colore com ANSI quando solicitado, removendo sequências de escape não relacionadas a cores.

## Importações
- `serde::{Deserialize, Serialize}`: serializa/deserializa `Operation`, `Request` e `Pending`.
- `apply_to`: aplica a operação ao conteúdo, validando versão e linhas.
- `version`: gera a versão (hash) usada na checagem de conflito.
- `diff`: monta o diff unificado entre `before` e o resultado.
- `render`: colore o diff com ANSI ou devolve texto puro.
- `display` / `commit` / `prepare` / `target` / `sync_parent`: expõem o fluxo de edição ao restante do crate.
