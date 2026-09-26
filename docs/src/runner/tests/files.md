## Resumo
Suíte de testes do runner que valida a ferramenta EDIT/WRITE/DELETE: versionamento, contenção de caminhos, retomada após falha e permissões.

## Funcionamento
Cada `#[test]` monta um `Workflow` em YAML e o executa sobre um `Project` temporário, simulando o modelo com `run_with`/`run_interactive_with`. Verifica conflitos de versão (arquivo alterado entre READ e EDIT), o ciclo prepare/commit da operação pendente (retomada idempotente sem duplicar escrita, rejeição de histórico adulterado ou Conflict), criação de arquivos inexistentes e seu recovery, escape do diretório via caminho absoluto, binário, symlink ou link em `.agents`, preservação de permissões POSIX, `skip: true` no WRITE e as permissões `DELETE_TOOL`/`DELETE_WITHOUT_CONFIRM` dos agentes. Falhas são affirmadas por mensagem (`"version conflict"`, `"conflict"`, `"invalid EDIT result"`); efeitos colaterais são restaurados/preservados no disco.

## Importações
- `super::*`: traz `Workflow`, `Project`, `History`, `Block`, `Snapshot` e os helpers `run_with`, `continue_with`, `answers`.
- `std::fs`: cria, lê e altera arquivos para forçar conflitos de versão.
- `serde_yaml`: converte os workflows YAML em `Workflow`.
- `serde_json`: serializa entradas e pendências nos blocos do histórico.
- `crate::tools::edit`: `Operation`, `Request`, `Pending` e `version` para exercitar a edição diretamente.
- `crate::tools::request`: confirma que a invocação textual `EDIT: file` não é aceita.
- `std::os::unix::fs::{PermissionsExt, symlink}`: valida symlinks e preservação de modo `0o751`.
