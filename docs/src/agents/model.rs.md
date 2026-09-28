## Resumo
Desserializa um modelo opcional, removendo espaços e convertendo-o para minúsculas.

## Funcionamento
Converte a entrada em `Option<String>` e propaga erros de desserialização; se houver uma string, normaliza-a com `trim` e `to_lowercase`.

## Importações
- `serde::Deserialize`: Habilita a desserialização de strings.
