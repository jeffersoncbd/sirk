## Resumo
Valida se uma `String` tem conteúdo não em branco, devolvendo-a como `Option`.

## Funcionamento
Aplica `trim()` para ignorar espaços; se restar texto, devolve o valor original (sem `trim`) via `then_some`, senão `None`. Sem `Result`, sem efeitos colaterais.

## Importações
- Nenhuma: usa só a biblioteca padrão (`str::trim`, `bool::then_some`).
