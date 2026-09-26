## Resumo
Executa um comando via `BashService` com streaming e devolve a saída padrão apenas em caso de sucesso.

## Funcionamento
A função cria um `BashService` padrão e dispara `execute_streaming` com a `Invocation` recebida, convertendo qualquer erro para `String` com `map_err` e propagando-o via `?`. Em seguida verifica `result.status.success()`: se o processo terminou com falha, retorna um `Err` descrevendo o programa, o status de saída e o fato de a saída parcial não ter sido consolidada. Se houve sucesso, retorna `Ok` com `result.stdout`. Não há estado interno: a função é sem efeito colateral além da execução do processo.

## Importações
- `BashService`: executa o comando com leitura incremental da saída.
- `Invocation`: carrega programa e argumentos usados na execução.
