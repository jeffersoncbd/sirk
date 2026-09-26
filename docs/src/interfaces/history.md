## Resumo
Define os tipos de dados de histórico: o `Snapshot` serializável de um estado de trabalho e o enum `Block` que identifica cada tipo de bloco registrado.

## Funcionamento
`Snapshot` agrega o diretório de trabalho, o `Workflow` e a lista de `Agent`s, com `deny_unknown_fields` para rejeitar chaves extras ao desserializar. `Block` é um enum simples (sem dados estruturados, apenas rótulo `String`) que enumera as operações possíveis — `Ask`, `Input`, `Output`, `Tree`, `Read`, `Edit` e `Delete` — servindo como tag para blocos de interação, derivados de `Debug`/`Clone`/`PartialEq`/`Eq` para comparação em testes. Os submódulos `marker` e `text` apenas estendem o módulo.

## Importações
- `crate::agents::Agent`: Tipo dos agentesLOB gravados no snapshot.
- `crate::workflow::Workflow`: Workflow associado ao estado serializado.
- `serde::Serialize`: Habilita a serialização do `Snapshot`.
- `serde::Deserialize`: Habilita a leitura do `Snapshot` de disco.
- `std::path::PathBuf`: Armazena o diretório de trabalho do snapshot.
