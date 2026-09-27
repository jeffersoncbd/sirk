## Resumo
Valida e inicia a execução interativa de um workflow configurado.

## Funcionamento
Valida o workflow, canonicaliza o diretório e carrega cada agente referenciado uma única vez. Cria e valida um snapshot e então continua a execução interativa, retornando os resultados ou um erro.

## Importações
- `Agent`: carrega agentes do diretório `.agents`.
- `History`, `Snapshot`: cria o histórico e registra o estado inicial.
- `UserInput`: fornece entrada à execução interativa.
- `Invocation`: tipa a função que executa comandos.
- `Workflow`: representa e valida o fluxo de trabalho.
- `BTreeMap`: armazena agentes e resultados por chave.
- `Path`: representa o diretório do workflow.
- `all_steps`: percorre as etapas do workflow.
- `continue_configured_with`: continua a execução interativa.
- `validate_snapshot`: valida o snapshot inicial.
