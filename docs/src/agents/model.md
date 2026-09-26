## Resumo
Normaliza o deserializador de um campo `model` de struct para `Option<String>` em minúsculas e sem espaços.

## Funcionamento
Aceita qualquer `serde::Deserializer` e delega a desserialização a `Option::<String>`, ou seja, campos ausentes ou `null` viram `None` sem erro. Valores presentes são convertidos com `trim()` (remove espaços nas pontas) e `to_lowercase()`, garantindo que o nome do modelo seja padronizado. Erros de formato do JSON (ex.: tipo incorreto para `String`) propagam via `D::Error`.

## Importações
- `serde::Deserialize`: trait que habilita a desserialização de `Option<String>`.
