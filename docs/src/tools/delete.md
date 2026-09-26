## Resumo
Remove um arquivo comum dentro do diretório de execução, recusando qualquer caminho que escape dele.

## Funcionamento
Valida que `path` não seja vazio, canonicaliza `directory` e resolve o alvo (absoluto ou relativo a `root`). Exige que todos os componentes do caminho relativo sejam `Component::Normal`, bloqueando `..`, `.` e travessia de diretório. Percorre os componentes intermediários com `symlink_metadata` para garantir que sejam diretórios reais, e usa `symlink_metadata` no alvo para exigir arquivo regular (bloqueando symlinks, diretórios e pipes). Canonicaliza o alvo e reconfirma que está sob `root` antes de `remove_file`; ao final, sincroniza o diretório pai com `File::open`/`sync_all` para persistir a remoção. Retorna `Err(String)` com mensagens prefixed por `DELETE` em cada falha de I/O ou de validação.

## Importações
- `std::fs::{self, File}`: consulta metadados sem seguir links, remove o arquivo e sincroniza o diretório pai.
- `std::path::{Component, Path}`: representa o caminho solicitado e valida que cada segmento é normal (sem `..`/`.`).
