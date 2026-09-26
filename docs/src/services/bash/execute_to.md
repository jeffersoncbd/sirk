## Resumo
Executa um comando bash, escrevendo o stdout em tempo real no destino fornecido e devolvendo o `ProcessOutput`.

## Funcionamento
Delega a execução para `execute_bytes_to`, que transmite o stdout para o `Write` recebido pelo chamador e mantém o stderr anexado, com stdin fechado. Em seguida converte os bytes capturados em `String`, devolvendo `io::ErrorKind::InvalidData` caso a saída não seja UTF-8 válida; o status do processo é preservado no retorno.

## Importações
- `super::{BashService, ProcessOutput}`: Trait do serviço e struct de retorno com status e stdout.
- `crate::services::Invocation`: Descritor do comando a ser executado.
- `std::io::{self, Write}`: Sink genérico de escrita e erros de I/O (conversão UTF-8).
