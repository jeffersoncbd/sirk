## Resumo
Verifica se uma chamada a um método desconhecido recebe um erro JSON-RPC de método não encontrado.

## Funcionamento
Cria uma requisição para `tools.read`, chama `handle` no diretório atual e confirma que a resposta contém o código `-32601` e a mensagem `"Method not found"`.

## Importações
- `super::super::{handle::handle, types::Request}`: Cria e processa a requisição.
- `serde_json::json`: Define os valores JSON da requisição.
