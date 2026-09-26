## Resumo
Converte um comando textual `EDIT:` em um `Request` de edição validado.

## Funcionamento
Aplica `trim`, exige o prefixo `EDIT:` (ausente → `None`), desserializa o JSON restante em `Request` e rejeita payloads com `version` preenchido, retornando a mensagem de erro do parse formatada dentro de `Some`.

## Importações
- `serde_json`: desserializa o payload JSON em `Request`.
- `crate::tools::edit::Request`: tipo de destino validado.
