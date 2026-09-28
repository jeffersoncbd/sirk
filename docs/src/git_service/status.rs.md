## Resumo
Lista os caminhos alterados no Git dentro do diretório solicitado, excluindo arquivos ignorados.

## Funcionamento
Localiza o projeto e executa `git status` e `git ls-files` para obter alterações e caminhos ignorados. Valida os registros, trata renomes e cópias, converte caminhos UTF-8 e remove o prefixo do diretório solicitado. Mantém arquivos excluídos ou ainda visíveis no sistema de arquivos; retorna erro se os comandos falharem ou a saída for inválida. Ordena e deduplica os caminhos antes de retorná-los.

## Importações
- `super::project::project`: Localiza o projeto Git e seu prefixo.
- `super::run::run`: Executa comandos Git e retorna a saída.
- `std::collections::BTreeSet`: Armazena caminhos excluídos.
- `std::fs`: Verifica a existência e o tipo dos caminhos.
- `std::path::Path`: Representa o diretório de entrada.
