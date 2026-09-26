## Resumo
Renderiza uma coordenada de edição (literal ou template) e valida que o resultado seja um inteiro positivo.

## Funcionamento
Para `EditCoordinate::Number`, o valor é repassado direto a `positive_coordinate`. Para `Template`, o texto é expandido via `render_scoped` com `outputs` e `locals` (escopo opcional). O resultado é aparado com `trim` e convertido para `usize`; falhas de interpolação ou de parse viram `coordinate_error(name)`. O valor final sempre passa por `positive_coordinate`, que rejeita não-positivos. Retorna a coordenada como `usize` ou `Err(String)`. Sem efeitos colaterais.

## Importações
- `EditCoordinate`: Enum do próprio módulo, alvo da implementação.
- `coordinate_error`: Constrói o erro padronizado para coordenada inválida.
- `positive_coordinate`: Valida e devolve a coordenada como inteiro positivo.
- `render_scoped`: Expande o template com outputs e locals.
- `render_scoped` (via `super`): Reexportado do módulo pai.
- `BTreeMap`: Maps ordenados de saídas e variáveis locais.
- `std::collections::BTreeMap`: Origem do tipo de mapa utilizado.
