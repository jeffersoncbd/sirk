## Resumo
Executa um agente em um fluxo silencioso de uma etapa e retorna seu resultado.

## Funcionamento
Monta um fluxo com a entrada de texto e configura a saída como `result`. Executa o fluxo no diretório informado; propaga erros da execução e retorna erro caso a saída `result` não seja produzida.

## Importações
- `runner::run_silent_with`: Executa o fluxo silenciosamente.
- `workflow::{Step, StepInput, Workflow}`: Define o fluxo e sua etapa.
- `std::path::Path`: Representa o diretório de execução.
