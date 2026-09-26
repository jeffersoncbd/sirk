## Resumo
Define a estrutura `GitStatusTree` que armazena a raiz do repositório e a lista de caminhos alterados reportados pelo `git status`.

## Funcionamento
O módulo apenas declara um container de dados: `root` (diretório base) e `files` (caminhos relativo a `root`, ordenados e sem duplicatas, conforme o contrato do comentário). A derivação de `Debug`, `PartialEq` e `Eq` permite imprimir e comparar valores da estrutura em testes, sem qualquer lógica própria — a coleta e ordenação dos caminhos ficam a cargo dos submódulos (`git`, `list`, `path`, `tree_ignored`).

## Importações
- `std::path::PathBuf`: Representa a raiz e cada caminho de arquivo Relative ordenado.
