## Resumo
Define os tipos de dados usados para representar registros do histórico.

## Funcionamento
`Snapshot` reúne o diretório e o agente, permitindo serialização e rejeitando campos desconhecidos. `Block` enumera os tipos de conteúdo registrados, cada um associado a uma string.

## Importações
- `crate::agents::Agent`: Tipo de agente armazenado no snapshot.
- `serde`: Deriva serialização e desserialização para `Snapshot`.
- `std::path::PathBuf`: Representa o diretório do snapshot.
