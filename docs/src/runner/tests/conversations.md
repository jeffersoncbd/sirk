## Resumo
Arquivo de testes que valida o protocolo de conversas do runner: invocação de agentes, perguntas ao usuário, ferramentas `READ`/`TREE` e retomada de execuções interrompidas.

## Funcionamento
Não há função de produção aqui — só testes. Eles criam um `Project` temporário, escrevem definições de agentes em `.agents/*.md`, parseiam workflows YAML e injetam um callback falso como "harness" para inspecionar o prompt enviado e devolver respostas simuladas. Cobrem: prefixo de chamada do adaptador (`call_prefix`), recusa de `TREE` sem permissão, perguntas (`ASK`) respondidas pelo usuário, execuções retomadas via `History` (inclusive após falha, remoção do `.git` ou dos arquivos-fonte), rejeição de estado inconsistente, e restauração de saídas já concluídas sem reexecutar o modelo.

## Importações
- `super::*`: harness do runner (`Project`, `Workflow`, `History`, `Block`, `run_with`, `run_interactive_with`, `continue_with`, `answers`, `fs`, `serde_yaml`).
