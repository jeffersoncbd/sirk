## Resumo
Executa uma invocação e retorna sua saída padrão apenas se o processo terminar com sucesso.

## Funcionamento
Executa `invocation` descartando a saída adicional; erros de execução são convertidos em texto. Se o processo falhar, retorna uma mensagem com o programa e o status, sem confirmar a saída. Caso tenha sucesso, retorna `stdout`.

## Importações
- `crate::services`: fornece o executor e a invocação do processo.
- `std::io`: fornece o destino que descarta a saída adicional.
