## Resumo
Deserializador customizado que converte a string `"allow"` em `bool` para a tool de delete.

## Funcionamento
Lê o valor recebido via `String::deserialize`; se for exatamente `"allow"`, retorna `Ok(true)`, caso contrário devolve erro de validação informando que `DELETE_TOOL` e `DELETE_WITHOUT_CONFIRM` devem ser `allow`. Falha apenas na desserialização do tipo base.

## Importações
- `serde::Deserialize`: Lê o campo como `String` antes da conversão para booleano.
- `serde::Deserializer`: Parametro generico que permite aceitar qualquer formato de desserializacao.
- `serde::de::Error::custom`: Gera o erro de configuracao invalida com a mensagem exigida.
