## Resumo
Executa uma invocação e transmite sua saída para a saída padrão.

## Funcionamento
Chama `execute_to` com a invocação recebida e a saída padrão bloqueada, retornando o resultado ou o erro de E/S.

## Importações
- `super`: Tipos `BashService` e `ProcessOutput`.
- `crate::services::Invocation`: Dados da invocação.
- `std::io`: Saída padrão e resultado de E/S.
