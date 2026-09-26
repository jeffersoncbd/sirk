## Resumo
Lê o conteúdo de um arquivo de texto (UTF-8) dentro de um diretório de execução, bloqueando caminhos ignorados ou fora da raiz.

## Funcionamento
Valida que o caminho não seja vazio, canonicaliza o diretório base e o caminho-alvo (resolvendo `..` e symlinks) e garante que o resultado permaneça dentro da raiz. Em seguida recusa diretórios ou qualquer entrada listada no `.readignore` (via `super::ignored::read_ignored`, devolvendo `AccessDenied`) e por fim retorna o conteúdo do arquivo. Todos os erros de I/O são convertidos em `String` descritivas prefixedadas por `READ`.

## Importações
- `std::fs`: Lê o conteúdo do arquivo em `String`.
- `std::path::Path`: Representa o diretório base e o caminho canônico.
