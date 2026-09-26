## Resumo
Interpreta um comando de texto `DELETE:<json>` e o converte em um `ExternalDeleteRequest` desserializado.

## Funcionamento
A função remove espaços do texto, exige o prefixo `DELETE:` (retornando `None` se ausente) e trima o restante como payload JSON. Em seguida, desserializa esse payload via `serde_json::from_str`; o resultado é envolvido em `Some`, então `None` indica apenas entrada inválida/ausente, enquanto `Some(Err(String))` indica JSON malformado com a mensagem de erro do serde prefixada por `invalid DELETE_TOOL request`.

## Importações
- `super::ExternalDeleteRequest`: Tipo de destino da desserialização do payload.
- `serde_json`: Converte a string JSON em `ExternalDeleteRequest` e fornece o erro.
