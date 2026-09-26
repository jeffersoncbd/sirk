## Resumo
Retorna o texto de ajuda/uso do CLI `new-harness` como uma string estática.

## Funcionamento
A função é um getter sem argumentos nem efeitos colaterais: devolve sempre a mesma string `&'static str` com o uso do programa (criação de agente, execução e retomada de fluxos via `flows/<flow-name>.yml`). Não há validação, `Option` ou `Result`; a constante fica embutida em tempo de compilação, e o chamador (`pub(super)`) deve imprimir ou propagar esse conteúdo.

## Importações
- Nenhuma: a função é autocontida, sem `use` nem dependências externas.
