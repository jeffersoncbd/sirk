## Resumo
`GitStatusTree::list` varre um repositório Git e retorna os caminhos modificados, adicionados, excluídos, renomeados, copiados, mesclados e não rastreados dentro de um diretório.

## Funcionamento
O diretório é canonicalizado (falhando se não existir ou não for diretório) e validado como árvore de trabalho Git via `git rev-parse --show-prefix`, que também define o prefixo a ser removido dos caminhos. Em seguida executa `git status --porcelain=v1 -z --untracked-files=all`, exigindo saída bem-sucedida. Cada registro NUL-delimitado é validado (código XY de 2 bytes + espaço), entradas `R`/`C` consomem o registro de destino, e caminhos são convertidos para relativos à raiz, descartando os vazios ou listados em `tree_ignored`. Entradas não excluídas são confirmadas via `symlink_metadata` para aceitar apenas arquivos e symlinks (diretórios e submodules ficam de fora; `NotFound` é tolerado, típico de exclusões). Ao final os caminhos são ordenados e deduplicados em `Self { root, files }`. Todos os erros retornam `String` descritiva com o prefixo `GIT-STATUS-TREE`; não há escrita no disco.

## Importações
- `std::fs`: verificação de arquivos via `symlink_metadata` (uso apenas de leitura).
- `std::io`: distingue `ErrorKind::NotFound` na inspeção de caminhos.
- `std::path::Path`: parâmetro do diretório raiz e verificação `is_dir`.
- `super::GitStatusTree`: struct que recebe a raiz e a lista de arquivos.
- `super::git::git`: executa `rev-parse` e `status` e expõe o status de saída.
- `super::path::path_from_bytes`: converte bytes do Git em `PathBuf`.
- `super::tree_ignored::tree_ignored`: fornece o conjunto de exclusões `.treeignore`.
