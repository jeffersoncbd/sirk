## Resumo
Compara componentes de um padrão com os de um caminho, permitindo `**` e correspondência por prefixo.

## Funcionamento
Retorna `true` quando padrão e caminho terminam juntos; se o padrão termina antes, usa `prefix` como resultado. `**` pode corresponder a zero ou mais componentes. Nos demais casos, compara cada par de componentes recursivamente; combinações incompatíveis retornam `false`.

## Importações
- `super::component`: Compara um componente do padrão com o caminho.
