## Resumo
Converte uma string JSON em um vetor de strings, garantindo que a entrada seja um array de textos.

## Funcionamento
A função delega a desserialização para `serde_json::from_str` e encapsula qualquer falha de parse no `Err` como uma mensagem descritiva prefixada com "LOOP requires a JSON array of strings", incluindo o erro original. Em caso de sucesso, retorna o `Vec<String>` desserializado; entradas que não sejam arrays de strings (ou JSON inválido) resultam em `Err(String)`. Não há efeitos colaterais.

## Importações
- `serde_json`: Desserializa a string de entrada em `Vec<String>`, reportando erros de formato.
