## Resumo
Constrói um `Request` de edição a partir de um `Step`, renderizando caminho, coordenadas e versão no escopo atual.

## Funcionamento
Valida cada coordenada opcional (`line`, `start`, `end`) via closure que escolhe entre `render_for_validation` e `render` conforme o flag `validating`, propagando erro com `transpose()`. O caminho e o conteúdo de entrada são renderizados com `outputs`/`locals`; `version` só é processada se presente. Retorna `Err("EDIT requires operation")` quando a operação é `None`, e propaga erros de renderização como `String`.

## Importações
- `super::{EditCoordinate, Step, render_scoped}`: Tipos do passo e renderização com escopo de outputs/locals.
- `std::collections::BTreeMap`: Mapa ordenado de variáveis para resolução de templates.
- `crate::tools::edit::Request`: Tipo de retorno da requisição de edição.
