## Resumo
Expõe o texto associado a qualquer variante de `Block` que o contenha.

## Funcionamento
O método `text()` faz um `match` sobre as variantes de `Block` que carregam `&str` (`Ask`, `Input`, `Output`, `Tree`, `Read`, `Edit`, `Delete`) e devolve o conteúdo armazenado em cada uma, sem clone nem alocação. Como o enum não tem variantes sem texto, o `match` é exaustivo e sempre retorna um `&str` com o mesmo tempo de vida de `&self`, dispensando `Option` ou `Result`.

## Importações
- `super::Block`: Enum alvo, cujas variantes de texto são desestruturadas no `match`.
