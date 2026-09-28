## Resumo
Gera um agente no diretório informado e retorna o caminho salvo.

## Funcionamento
Deleg​a a geração a `create_with`, executando a invocação pelo `BashService`. Converte falhas de execução em erro e rejeita saídas com status malsucedido; se tudo der certo, retorna a saída padrão.

## Importações
- `UserInput`: fornece a entrada usada na geração.
- `BashService`: executa a invocação e captura a saída.
- `Path`, `PathBuf`: representam o diretório e o caminho retornado.
