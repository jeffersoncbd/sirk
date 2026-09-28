## Resumo
Executa uma invocação e retorna sua saída padrão se ela terminar com sucesso.

## Funcionamento
Usa `BashService` para executar a invocação, descartando a saída direcionada ao coletor. Converte erros de execução em texto; se o processo terminar com falha, retorna uma mensagem com o programa e o status. Caso contrário, retorna `stdout`.

## Importações
- `crate::services::{BashService, Invocation}`: execução e dados da invocação.
- `std::io`: fornece o coletor que descarta a saída.
