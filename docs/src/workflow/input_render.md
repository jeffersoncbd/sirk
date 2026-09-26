## Resumo
Renderiza um `StepInput` (bool, texto ou array) em `String` usando os escopos `outputs` e `locals`.

## Funcionamento
O método faz `match` sobre a variante do enum: `Bool` retorna o valor convertido com `to_string`; `Text` delega a interpolação a `render_scoped`; `Array` renderiza cada item individualmente, coleta os resultados em `Vec<String>` e serializa o conjunto como JSON. Como `collect::<Result<Vec<_>, _>>()` percorre todos os itens, a renderização falha no primeiro erro, sendo o retorno `Result<String, String>` com a mensagem do erro como string.

## Importações
- `super::{StepInput, render_scoped}`: Enum de entrada do passo e rotina de interpolação de templates.
- `std::collections::BTreeMap`: Mapa ordenado que armazena os escopos de variáveis (`outputs` e `locals`).
- `serde_json`: Serialização do vetor de itens renderizados para JSON.
