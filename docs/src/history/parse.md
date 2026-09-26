## Resumo
Interpreta um transcript de histórico v2 e o converte em snapshot, passos e rótulos.

## Funcionamento
Exige o prefixo `TITLE` (erro caso contrário) e separa metadados do corpo por `\n---\n`, desserializando o YAML do snapshot. Percorre o corpo linha a linha: separadores e marcadores (`==> ASK`, `==> INPUT`, `<== OUTPUT`, `==> TREE`, `==> READ`, `==> EDIT`, `==> DELETE`, `Step N`) fecham o bloco ativo via `finish`; marcadores iniciam um bloco (rotulados ou não) e `Step N` cria um novo vetor de blocos. Linhas sem bloco ativo e não vazias geram erro; linhas iniciado com `\` perdem a indentação. Retorna `(Snapshot, Vec<Vec<Block>>, Vec<String>)` ou mensagem de erro.

## Importações
- `super::{Block, History, ParsedHistory, ...}`: tipos internos e marcadores do formato.
- `super::finish::finish`: finaliza o bloco ativo acumulando-o em `steps`.
- `serde_yaml`: desserializa os metadados YAML em `Snapshot`.
