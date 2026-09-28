## Resumo
Verifica se um caminho corresponde ao padrão de ignorados informado.

## Funcionamento
Remove barras iniciais do padrão e separa padrão e caminho em componentes. Para padrões de um único componente, verifica se algum componente do caminho corresponde; para padrões com vários componentes, delega a comparação e indica se o padrão termina com `/`.

## Importações
- `super::component`: compara um padrão com um componente do caminho.
- `super::components`: compara sequências de componentes.
