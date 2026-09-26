## Resumo
Renderiza um template usando um mapa de saídas, delegando para `render_scoped` sem escopo definido.

## Funcionamento
A função é um wrapper fino: valida e repassa `template` e `outputs` para `super::render_scoped` com o escopo `None`, propagando o `Result<String, String>` (sucesso com o texto renderizado ou erro em `String`) sem tratamento adicional. Não possui efeitos colaterais próprios.

## Importações
- `std::collections::BTreeMap`: Mapa ordenado de variáveis disponíveis para a renderização.
