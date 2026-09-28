## Resumo
Interpreta pedidos isolados de leitura ou consulta da árvore de arquivos.

## Funcionamento
Remove espaços externos, reconhece `TREE` ou `READ:` e retorna o tipo do pedido com seu caminho. Retorna `None` para outros textos ou caminhos com quebras de linha.

## Importações
- Nenhuma: usa apenas métodos e tipos padrão do Rust.
