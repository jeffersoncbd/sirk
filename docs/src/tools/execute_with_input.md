## Resumo
Despacha a execução de uma ferramenta de nome `name` no diretório informado, usando `input` apenas na `READ`.

## Funcionamento
Faz um `match` sobre o nome da ferramenta: para `TREE` e `GIT-STATUS-TREE`, lista os arquivos via `Tree::list`/`GitStatusTree::list` e formata a saída com `format_paths`; para `READ`, repassa `input` a `read::read`; qualquer outro nome retorna `Err` describing a ferramenta desconhecida. O diretório é usado apenas para localizar os arquivos, sem escrita em disco.

## Importações
- `std::path::Path`: Tipo do diretório alvo das operações de listagem/leitura.
- `super::format_paths::format_paths`: Padroniza a saída textual da lista de arquivos.
- `super::git_status_tree::GitStatusTree`: Enumera caminhos com alterações do Git.
- `super::read::read`: Executa a leitura do arquivo com base no `input`.
- `super::tree::Tree`: Enumera a árvore de arquivos ignorando regras do Git.
