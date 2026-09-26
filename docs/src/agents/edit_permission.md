## Resumo
Deserializa o campo `EDIT_TOOL` exigindo que o único valor aceito seja `allow`, retornando `true` nesse caso.

## Funcionamento
A função é um desserializador customizado para `bool`. Ela primeiro converte o valor recebido em `String` via `String::deserialize`, propagando o erro do framework caso o tipo não seja uma string. Em seguida compara o texto com `"allow"`: se iguais, retorna `Ok(true)`; caso contrário, retorna `Err` com uma mensagem de validação personalizada informando que o valor deve ser `allow`. O efeito colateral é apenas a leitura do valor do documento durante a desserialização; nenhuma escrita em disco ou rede ocorre.

## Importações
- `serde::Deserialize`: Trait que permite desserializar a `String` a partir do documento de entrada.
