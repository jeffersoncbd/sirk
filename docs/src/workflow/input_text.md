## Resumo
Fornece um acessor que extrai o texto de um `StepInput` quando a entrada é do tipo texto.

## Funcionamento
O método `text` faz um `match` sobre as variantes de `StepInput`: `Text` retorna `Some` com a referência ao conteúdo, enquanto `Array` e `Bool` retornam `None`, sinalizando ao chamador que a entrada não é textual. Não há validações adicionais nem efeitos colaterais; apenas borrowing da variante correspondente.

## Importações
- `super::StepInput`: enum de entrada do workflow implementado, cujas variantes são inspecionadas.
