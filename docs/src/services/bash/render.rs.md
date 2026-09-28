## Resumo
Gera uma linha de comando para executar uma `Invocation`.

## Funcionamento
Adiciona `exec --`, cita o programa e cada argumento conforme o ambiente da invocação e une tudo com espaços. Retorna a linha como `String`.

## Importações
- `BashService`: Tipo que recebe o método.
- `Invocation`: Fornece programa, argumentos e ambiente.
- `shell_quote`: Cita o programa e os argumentos.
