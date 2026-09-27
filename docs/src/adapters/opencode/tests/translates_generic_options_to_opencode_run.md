## Resumo
O teste verifica se opções genéricas de execução são convertidas corretamente em uma invocação do OpenCode.

## Funcionamento
Cria uma solicitação com prompt, diretório de trabalho, modelo e fluxo de eventos habilitado; em seguida, compara a invocação produzida com o comando `opencode run` esperado, incluindo argumentos, diretório e ambiente vazio.

## Importações
- `super::*`: Acessa os tipos e métodos do módulo pai.
