## Resumo
Representa o inventário recursivo de arquivos de um repositório, seguindo as regras de ignore e índice do próprio Git.

## Funcionamento
`Tree` guarda o diretório raiz e os arquivos já ordenados, únicos e relativos à raiz; a enumeração e a relativização ficam nos submódulos `list` e `path`.

## Importações
- `std::path::PathBuf`: Armazena a raiz e cada caminho relativo listado.
- `list`: Submódulo interno que enumera os arquivos elegíveis.
- `path`: Submódulo interno que resolve e relativiza os caminhos.
