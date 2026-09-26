## Resumo
Retorna o marcador textual (seta) que representa o tipo de cada bloco do histórico.

## Funcionamento
O método faz um `match` sobre as variantes do enum `Block` e devolve um `&str` estático correspondente, com direção da seta indicando entrada (`==>`) ou saída (`<==`) de dados. Não há validações, erros (`Result`/`Option`) nem efeitos colaterais; a variante não contemplada resultaria em erro de compilação, garantindo exaustividade.

## Importações
- `super::Block`: Enum base cujas variantes (Ask, Input, Output, etc.) são mapeadas para marcadores.
