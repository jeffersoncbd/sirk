## Resumo
Executa um comando Git no diretório indicado e retorna sua saída padrão em bytes.

## Funcionamento
Monta uma invocação de `git` com os argumentos fornecidos e ambiente padrão. Converte falhas de execução em `Err` com contexto; se o processo terminar com sucesso, retorna `stdout`, caso contrário informa o status de saída.

## Importações
- `BashService`: executa o processo e captura sua saída.
- `Invocation`: define programa, argumentos, diretório e ambiente.
- `std::io`: fornece o destino descartável para outra saída.
- `std::path::Path`: representa o diretório de trabalho.
