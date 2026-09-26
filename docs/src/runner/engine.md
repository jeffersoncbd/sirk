## Resumo

`src/runner/engine.rs` é o módulo de composição interna do mecanismo de execução de workflows. Ele organiza submódulos relacionados à inicialização, execução, processos externos e validação de histórico, além de reexportar as funções públicas usadas para controlar a execução.

## Funcionamento

O arquivo declara quatro submódulos por meio de caminhos explícitos:

- `bootstrap.rs`: inicialização e retomada de execuções.
- `execution.rs`: operações de execução e continuação.
- `external.rs`: integração com operações externas.
- `history_validation.rs`: validação do histórico de execução.

As funções relevantes são reexportadas para que possam ser acessadas por meio deste módulo, sem expor diretamente a organização interna desses arquivos.

Durante testes, também são reexportadas duas constantes internas de `external.rs`. O módulo `tests.rs` é compilado apenas em configurações de teste.

O arquivo não implementa lógica de execução diretamente, não realiza chamadas externas e não possui tratamento próprio de `Result` ou `Option`; ele apenas estrutura e expõe componentes implementados nos submódulos.

## Componentes principais

- `mod bootstrap`: módulo privado responsável pela composição inicial e retomada de workflows.
- `mod execution`: módulo privado que contém a lógica de continuação da execução.
- `mod external`: módulo privado relacionado a resultados ou operações externas.
- `mod history_validation`: módulo privado de validação de histórico.
- `pub use bootstrap::{resume, run, run_interactive_with, run_with}`: expõe funções para iniciar, retomar e executar workflows, incluindo modos interativo e configurável.
- `pub use execution::continue_with`: expõe a continuação de uma execução existente.
- `DUPLICATE_EDIT_RESULT` e `EDIT_FAILURE_PREFIX`: constantes disponibilizadas somente durante testes.
- `mod tests`: módulo de testes compilado apenas com `#[cfg(test)]`.

## integrações

O arquivo expõe publicamente as funções `resume`, `run`, `run_interactive_with`, `run_with` e `continue_with`. Os detalhes do comportamento dessas funções dependem dos submódulos `bootstrap.rs` e `execution.rs`, que não estão incluídos no conteúdo analisado.
