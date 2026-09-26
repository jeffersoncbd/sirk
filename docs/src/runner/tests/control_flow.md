## Resumo
Suíte de testes do runner que valida os passos de controle de fluxo `LOOP` e `IF`, o passo `WRITE` e a retomada/validação do histórico.

## Funcionamento
Os testes montam workflows YAML com `Project::new()` e os executam via `run_with`/`run_interactive_with`, injetando um closure como harness de agente para contar chamadas e simular saídas ou interrupções. Verificam que `LOOP` itera itens com escopo local (`loop.*`), valida que a entrada do loop é um array de strings, que loops aninhados e vazios restauram o escopo externo, e que nenhum arquivo de histórico é criado quando a validação falha. Para `IF`, confirmam que apenas o ramo selecionado executa, que documentos existentes são reaproveitados em vez de regerados, e que agentes do ramo não selecionado ainda assim são validados. Também testam `WRITE` gravando arquivo sem chamar o harness e sem repetir após retomada, e que `continue_with` rejeita histórico editado, truncado ou com registros posteriores inconsistentes.

## Importações
- `super::*`:reexporta o módulo `runner` (Project, Workflow, History, Block, run_with).
