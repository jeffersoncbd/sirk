## Resumo
Extrai o nome de um alvo `{{ loop.NOME }}` de uma string de saída, devolvendo-o apenas se válido.

## Funcionamento
Aplica `trim` e remove o prefixo `{{` e o sufixo `}}` de forma opcional (qualquer ausência encerra o fluxo via `?`), trimando novamente, removendo o prefixo `loop.` e, por fim, usando `filter` com `valid` para devolver `Some(nome)` somente quando o nome cumpre a regra de validade; caso contrário retorna `None`. Não há efeitos colaterais; o dado de retorno é uma fatia (`&str`) emprestada da entrada.

## Importações
- `super::output_name::valid`: Valida o nome extraído antes de retorná-lo.
