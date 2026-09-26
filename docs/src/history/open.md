## Resumo
Abre um arquivo de histórico, adquire o lock e carrega seu conteúdo já parseado.

## Funcionamento
Canonicaliza o caminho recebido (convertendo erro de FS em `String`), adquire o lock do recurso via `Self::lock` e, em seguida, lê todo o conteúdo com `fs::read_to_string`. O texto é então delegado a `Self::parse`, que retorna o snapshot, os passos e os labels. Qualquer falha — canonicalização, lock, leitura ou parse — é propagada como `Err(String)`. Em sucesso, constrói e retorna a struct `History` já preenchida, mantendo o lock retido como guarda (`_lock`).

## Importações
- `super::History`: Struct alvo do método (`impl History`), dona os campos de estado.
- `std::fs`: Usada em `read_to_string` para carregar o arquivo do disco.
- `std::path::Path`: Recebe o caminho de entrada e permite `canonicalize()`.
