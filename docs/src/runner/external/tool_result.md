## Resumo
Converte o resultado textual de uma ferramenta em um `Block` de histórico, classificando-o como leitura ou árvore.

## Funcionamento
A função recebe o nome da ferramenta e o resultado retornado por ela. Se a ferramenta for `"TREE"`, o conteúdo é encapsulado em `Block::Tree`; em qualquer outro caso (por exemplo, ferramentas de leitura), vira `Block::Read`. Não há validações, erros (`Result`/`Option`) nem efeitos colaterais — é apenas um mapeamento puro entre o nome da ferramenta e a variante do bloco. Assume-se que `tool` e `result` já estejam disponíveis e que o `Block` resultante seja aceito pelo histórico.

## Importações
- `crate::history::Block`: Tipo de bloco do histórico usado para classificar o resultado.
