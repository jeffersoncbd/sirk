## Resumo
Expõe serviços e tipos relacionados à execução de comandos.

## Funcionamento
Declara os módulos `bash` e `quote` e reexporta `Invocation`, `BashService` e `ProcessOutput` para uso por outros módulos.

## Importações
- `bash`: Serviço de execução e saída de processos.
- `quote`: Módulo de tratamento de argumentos.
- `crate::interfaces::Invocation`: Tipo reexportado para construir chamadas.
