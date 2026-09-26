## Resumo
`Tree::list` monta a lista de arquivos de um repositório Git aplicando filtros `.gitignore` e `.treeignore`.

## Funcionamento
Canonicaliza o diretório (erro se não existir ou não for diretório) e executa `git ls-files` duas vezes: uma para listar rastreados e não ignorados, outra apenas para avaliar `.treeignore`. Os resultados são subtraídos como conjuntos de bytes separados por NUL, evitando que uma negação em `.treeignore` reintroduza arquivos ignorados pelo Git. Cada entrada é inspecionada via `symlink_metadata`: arquivos e symlinks entram na lista, diretórios/gitlinks e deleções rastreadas são descartados, e erros de I/O viram `Err`. O resultado é ordenado e deduplicado. Não altera o índice; caminhos não-UTF-8 são preservados.

## Importações
- `BashService`: executa os comandos `git ls-files` e devolve status e stdout.
- `Invocation`: descreve programa, argumentos, diretório de trabalho e ambiente.
- `Tree`: estrutura sendo construída, com `root` e `files`.
- `path_from_bytes`: converte bytes separados por NUL em `PathBuf`.
- `BTreeSet`: conjunto de exclusões para comparação eficiente.
- `fs`: `symlink_metadata` para classificar entradas sem seguir links.
- `io`: `sink()` descarta stderr e `ErrorKind::NotFound` distingue deleções.
- `Path`: representa o diretório raiz da listagem.
