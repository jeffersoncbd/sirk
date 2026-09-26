## Resumo
Casa uma lista de componentes de caminho com um padrão glob, aceitando correspondência parcial quando `prefix` é verdadeiro.

## Funcionamento
A função é recursiva e resolve três casos: padrão e caminho vazios retornam `true`; padrão vazio com caminho restante retorna o valor de `prefix`, indicando que o padrão anterior casou com o início do caminho; `"**"` casa com zero ou mais componentes, avançando no padrão ou consumindo um segmento do caminho. Demais casos exigem que `matches_component` valide cada par segmento-a-segmento; qualquer incompatibilidade de tamanho retorna `false`.

## Importações
- `super::component`: delega a comparação de um segmento único do padrão.
