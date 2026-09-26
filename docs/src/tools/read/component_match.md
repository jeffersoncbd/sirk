## Resumo
Verifica se um padrão de curingas casa com um valor, recursivamente.

## Funcionamento
Percorre `pattern` e `value` em par: `[]` casa só com `[]`; `*` casa com o restante ou consome um caractere e repete; `?` consome exatamente um; demais exigem igualdade. Qualquer outra combinação (incluindo sobras em um dos lados) retorna `false`.

## Importações
- Nenhuma: a função é pura e não depende de módulos externos.
