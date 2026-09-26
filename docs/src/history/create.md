## Resumo
Cria uma nova instância de `History` vinculada ao `Snapshot` fornecido.

## Funcionamento
Garante a existência do diretório `history` dentro do diretório do snapshot, gera um caminho único (`run-{nanos}-{pid}.log`), adquire um bloqueio nesse arquivo, constrói o `History` com listas vazias e o persiste via `save()`, propagando qualquer erro como `String`.

## Importações
- `super::{History, Snapshot}`: Tipos necessários para criar e vincular o histórico.
- `std::fs`: Cria o diretório `history`.
- `std::time::{SystemTime, UNIX_EPOCH}`: Gera o timestamp único para o arquivo.
