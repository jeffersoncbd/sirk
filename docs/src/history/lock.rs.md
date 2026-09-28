## Resumo
Adquire um bloqueio exclusivo para o arquivo de histórico.

## Funcionamento
Cria ou abre o arquivo `.log.lock` correspondente, sem truncá-lo, e solicita um bloqueio. Retorna o arquivo bloqueado ou uma mensagem de erro caso a abertura ou o bloqueio falhe.

## Importações
- `super::History`: Tipo ao qual o método é associado.
- `std::fs`: Abre ou cria o arquivo de bloqueio.
- `std::path::Path`: Recebe o caminho do histórico.
