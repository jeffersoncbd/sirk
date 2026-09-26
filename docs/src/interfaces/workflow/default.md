## Resumo
Fornece a implementação padrão de `StepInput`, retornando uma entrada de texto vazia.

## Funcionamento
A trait `Default` é implementada para `StepInput`; `default()` constrói a variante `Text` com `String::new()`, ou seja, uma string vazia como valor inicial. Não há validações, erros (`Result`/`Option`) nem efeitos colaterais — é apenas a construção de um valor.

## Importações
- `super::StepInput`: Tipo local referenciado na implementação de `Default`.
