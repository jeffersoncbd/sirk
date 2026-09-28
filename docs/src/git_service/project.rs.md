## Resumo
Resolve um diretório Git e retorna seus caminhos raiz e prefixo relativos.

## Funcionamento
Canonicaliza o caminho recebido, confirma que ele aponta para um diretório e consulta o Git para obter a raiz do projeto e o prefixo. Erros de resolução, comandos Git ou conversão UTF-8 são retornados como `String`; as quebras de linha finais são removidas antes de montar `Project`.

## Importações
- `super::run::run`: Executa comandos Git no diretório informado.
- `std::path::{Path, PathBuf}`: Representa e manipula caminhos.
