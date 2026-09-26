## Resumo
Executa um `Invocation` no shell redirecionando a saída padrão para o stream de saída do processo atual.

## Funcionamento
Bloqueia `io::stdout()` e delega a execução para `execute_to`, que recebe o invocação e um writer como destino da saída, retornando `io::Result<ProcessOutput>` com o resultado ou o erro propagado.

## Importações
- `super::{BashService, ProcessOutput}`: Trait do serviço e estrutura de resultado retornada.
- `crate::services::Invocation`: Definição do comando e parâmetros a executar.
- `std::io`: Fornece `io::Result` e o lock do stream de saída padrão.
