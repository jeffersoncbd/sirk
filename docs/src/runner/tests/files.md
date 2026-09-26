## Resumo

`src/runner/tests/files.rs` contém testes de integração do executor de workflows relacionados às operações de arquivos. O arquivo verifica edição, escrita, exclusão, recuperação após falhas, conflitos de versão, confinamento de caminhos e permissões.

## Funcionamento

Os testes criam projetos temporários, arquivos e workflows YAML, executando-os por meio de helpers como `run_with`, `run_interactive_with` e `continue_with`.

A cobertura principal inclui:

- Rejeição de edições quando o arquivo mudou após o `READ`.
- Recuperação de operações `EDIT` pendentes antes ou depois do commit, sem duplicar conteúdo.
- Preservação do arquivo quando há conflito ou resultado forjado.
- Bloqueio de caminhos externos, diretórios especiais, arquivos binários e symlinks.
- Preservação das permissões Unix durante uma edição.
- Criação e recuperação de arquivos inexistentes.
- Exigência de confirmação para `DELETE`, exceto quando `force: true` é autorizado.
- Verificação das permissões `DELETE_TOOL` e `DELETE_WITHOUT_CONFIRM` para agentes.
- Tratamento de operações `WRITE` ignoradas com `skip: true`, inclusive durante retomada do histórico.

Os testes usam `unwrap` quando esperam sucesso e `unwrap_err` ou `is_err` quando validam falhas. As mensagens de erro são verificadas parcialmente, especialmente para conflitos de versão, caminhos inválidos e resultados inválidos de edição.

## Componentes principais

- `edit_rejects_stale_read_version_and_preserves_file`: valida controle de versão entre `READ` e `EDIT`.
- `prepared_edits_recover_before_and_after_commit_without_duplication`: testa recuperação de edições preparadas e idempotência.
- `edit_confines_paths_and_preserves_permissions`: verifica limites de caminho, rejeição de symlinks e preservação de permissões.
- `edit_creates_missing_files_and_recovers_creation`: cobre criação de novos arquivos e recuperação após publicação incompleta.
- `missing_edit_targets_remain_confined_and_line_edits_require_files`: garante que operações por linha exigem arquivos existentes e que novos arquivos não escapem do projeto.
- `skipped_write_completes_and_resume_does_not_recreate_it`: verifica que uma escrita ignorada não altera nem recria o arquivo.
- `delete_requires_confirmation_unless_forced`: testa confirmação interativa e exclusão forçada.
- `agent_delete_requires_permission_and_reports_its_result`: valida a permissão básica para `DELETE` e o retorno ao agente.
- `agent_delete_force_requires_the_separate_permission`: valida a permissão adicional necessária para exclusão sem confirmação.
- `Pending`, `Request` e `Operation`: estruturas importadas do módulo de edição para simular estados pendentes e aplicar operações diretamente.
- `History`, `Snapshot` e `Block`: representam o histórico persistido do workflow e seus passos.
- `Project`, `Workflow`, `run_with`, `run_interactive_with`, `continue_with` e `answers`: helpers importados por `super::*` para criar projetos temporários, executar workflows e simular respostas.

## integrações

O arquivo não define APIs públicas nem módulos de produção; ele contém apenas funções de teste privadas marcadas com `#[test]`.

Ele integra:

- O executor de workflows e seu histórico persistido.
- As ferramentas `READ`, `EDIT`, `WRITE` e `DELETE`.
- O sistema de permissões de agentes baseado em arquivos `.agents/*.md`.
- O sistema de entrada interativa e callbacks que simulam respostas de modelos.
- O filesystem local, incluindo comportamento específico de Unix para symlinks e permissões.
