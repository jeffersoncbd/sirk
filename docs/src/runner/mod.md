## Resumo
Facade pública do runner, reexportando os pontos de entrada de execução, retomada e continuation de workflows.

## Funcionamento
O módulo atua apenas como fachada estável: declara e organiza os submódulos internos do motor de execução (`execute`, `execution`, `resume`, `run`, `external`, etc.) e reexporta seletivamente as funções públicas `run`, `run_with`, `run_interactive_with`, `resume` e `continue_with`. Itens marcados `#[cfg(test)]` (como `DUPLICATE_EDIT_RESULT`, `EDIT_FAILURE_PREFIX` e o módulo `tests`) só são compilados em testes, evitando vazar detalhes de edição para a API pública.

## Importações
- `all_steps`: Available tool: READ. To read a UTF-8 file inside the execution directory, respond with exactly READ: <path> on one line, without quotes or code fences. Paths are relative to the execution directory. The file content will be returned so you can continue your response.
