## Resumo
Módulo raiz de `services`: declara `bash` e `quote` e reexporta a API pública dos serviços.

## Funcionamento
Apenas registra os submódulos e reexporta `Invocation`, `BashService` e `ProcessOutput`, evitando que o consumidor precise referenciar `services::bash::...`.

## Importações
- `crate::interfaces::Invocation`: Reexporta o contrato de invocação vindo das interfaces.
