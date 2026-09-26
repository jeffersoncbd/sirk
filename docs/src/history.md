## Resumo
Define a estrutura `History`, que persiste e recupera o transcript editável de uma execução.

## Funcionamento
`History` agrupa um `Snapshot` (diretório, workflow e agentes), os `steps` gravados como listas de `Block` e seus `labels`, mantendo um `File` de lock para garantir acesso exclusivo. As operações delegates aos submódulos (`create`, `open`, `save`, `parse`, `finish`, `drop`, `lock`, `reserved`) manejam o formato em texto marcado por `TITLE`/`SEPARATOR` e o cabeçalho `v2`; erros de arquivo, formato ou lock são propagados via `Result`.

## Importações
- `crate::interfaces::{Block, Snapshot}`: tipos de conteúdo do transcript e do estado inicial da execução
- `std::fs::File`: mantém o lock de escrita exclusivo sobre o arquivo de histórico
- `std::path::PathBuf`: caminho do arquivo de transcript em disco
- `serde_yaml`: desserialização do workflow do `Snapshot` (apenas em testes)
