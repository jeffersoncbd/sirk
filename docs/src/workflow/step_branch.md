## Resumo
Seleciona o ramo (verdadeiro ou falso) de um `Step` e devolve seu rótulo junto com os passos associated.

## Funcionamento
A função recebe um booleano `selected` e faz uma escolha binária: quando `true`, retorna a tupla `("true", &self.is_true)`; quando `false`, retorna `("false", &self.is_false)`. Não há validações, erros ou efeitos colaterais — apenas a leitura de referências (`&str` e `&[Step]`) com tempo de vida vinculado a `&self`, sem alocação de memória.

## Importações
- `super::Step`: Tipo que与方法 implementa, fornecendo os campos `is_true` e `is_false`.
