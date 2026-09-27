## Resumo
Executa uma invocação e retorna sua saída padrão somente se ela terminar com sucesso.

## Funcionamento
Descarta a saída direcionada ao destino de escrita, converte falhas de execução em texto e retorna um erro se o processo terminar com status malsucedido. Nesse caso, a saída parcial não é retornada.

## Importações
- `crate::services`: fornece o serviço de execução e a invocação.
- `std::io`: fornece o destino que descarta a saída.
