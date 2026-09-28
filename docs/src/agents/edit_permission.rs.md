## Resumo
Converte o valor textual `allow` em `true`.

## Funcionamento
Desserializa uma string e retorna `Ok(true)` se ela for `allow`; caso contrário, retorna um erro indicando o valor esperado.

## Importações
- `serde::Deserialize`: Desserializa o valor como string.
