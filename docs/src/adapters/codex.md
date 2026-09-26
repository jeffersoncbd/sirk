## Resumo
Traduz um `RunRequest` genérico em uma `Invocation` de linha de comando para o harness `codex exec`.

## Funcionamento
Monta a lista de argumentos começando por `exec`, sempre adicionando `--skip-git-repo-check` (permitindo execução fora de repositório Git), `--sandbox read-only` e `-c approval_policy="never"` para garantir execução não interativa e somente leitura. Se `event_stream` for verdadeiro, inclui `--json`; se houver `model`, repassa via `--model`. Finaliza com `--` e o prompt como argumento posicional. O `Invocation` resultante usa o executável configurado, o diretório de trabalho da requisição e ambiente vazio. Sempre retorna `Ok`, pois as flags fixas dispensam validação.

## Importações
- `default`: módulo com implementação padrão de `CodexAdapter` (binário `codex`).
- `new`: módulo com construtor alternativo do adaptador.
- `crate::harness::HarnessAdapter`: trait que define `id` e `invocation` para adaptadores.
- `crate::harness::HarnessError`: tipo de erro do trait; não é construído neste arquivo.
- `crate::harness::Invocation`: struct de saída com programa, argumentos, diretório e ambiente.
- `crate::harness::RunRequest`: dados de entrada (prompt, modelo, diretório, flag de stream).
