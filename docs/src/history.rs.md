## Resumo
Define o estado editável do histórico de conversas e sua estrutura de armazenamento.

## Funcionamento
Declara módulos auxiliares, reexporta `Block` e `Snapshot` e define `History`, que reúne o caminho, o snapshot, os blocos e um arquivo de lock.

## Importações
- `crate::interfaces`: Fornece `Block` e `Snapshot`.
- `std::fs::File`: Mantém o arquivo de lock.
- `std::path::PathBuf`: Armazena o caminho do histórico.
