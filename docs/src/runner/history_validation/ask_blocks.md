## Resumo
Valida se um histórico de blocos é um par ASK/Input válido, indicando se ele deve ser reprocessado.

## Funcionamento
A função faz pattern matching sobre a fatia de blocos: histórico vazio ou um ASK com pergunta em branco retornam `Ok(false)` (nada a refazer); exatamente um ASK com pergunta preenchida seguido de um Input retorna `Ok(true)` (solicitação respondida, replay esperado); qualquer outra combinação (ASK sem Input, Input isolado, múltiplos pares) retorna `Err` pedindo que o resultado editado e tudo após sejam removidos. O `trim()` evita perguntas compostas só por espaços.

## Importações
- `crate::history::Block`: Enum dos blocos do histórico, usado no pattern matching.
