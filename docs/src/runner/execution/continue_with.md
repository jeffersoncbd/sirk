## Resumo
Retoma a execução de um workflow a partir do snapshot salvo, retornando os outputs dos passos executados.

## Funcionamento
Valida o snapshot do histórico e a integridade dos blocos antes de executar qualquer passo. Clona os passos do workflow, monta um `Engine` com cursor zerado e executa a sequência completa de forma iterativa, acumulando os resultados em um `BTreeMap` nomeado por passo. Propaga como `Err(String)` qualquer falha de validação ou de invocação (`execute`), interrompendo o fluxo imediatamente; em caso de sucesso, devolve o mapa de outputs.

## Importações
- `history_validation::validate_blocks`: Verifica a consistência dos blocos do histórico.
- `validate_snapshot::validate_snapshot`: Confere a validade do snapshot antes da retomada.
- `super::Engine`: Máquina de estados que executa os passos do workflow.
- `crate::history::History`: Fornece o snapshot e o estado mutável do histórico.
- `crate::input::UserInput`: Abstrato de entrada do usuário durante a execução.
- `crate::services::Invocation`: Representa a chamada a um passo, base para `execute`.
- `std::collections::BTreeMap`: Coleta e ordena os outputs por chave de passo.
