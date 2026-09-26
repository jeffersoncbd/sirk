## Resumo
Coleta caminhos de arquivos ignorados pelo `.treeignore` via `git ls-files`, retornando-os como conjunto ordenado.

## Funcionamento
Invoca `git ls-files` com `--cached --others --ignored` e o padrão de exclusão `.treeignore`, delimitando a saída por NUL (`-z`) para suportar caminhos com espaços. Se o comando falhar, converte o status em `Err` com mensagem contextualizada. A stdout é dividida nos separadores nulos, entradas vazias são descartadas e cada caminho é convertido em `Vec<u8>` antes de ser inserido no `BTreeSet`, que elimina duplicados e ordena o resultado.

## Importações
- `std::collections::BTreeSet`: conjunto ordenado que deduplica os caminhos ignorados.
- `std::path::Path`: parâmetro `root` que define o diretório de trabalho do Git.
- `super::git::git`: executa o comando Git e devolve `Result` com status e stdout.
