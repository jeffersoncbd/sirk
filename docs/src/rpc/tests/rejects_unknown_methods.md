## Resumo
Este arquivo testa se métodos RPC desconhecidos são rejeitados com o erro “Method not found”.

## Funcionamento
Não há função principal: o teste envia `tools.read` ao handler e verifica que a resposta contém o código `-32601` e a mensagem esperada.

## Importações
- `handle`: Processa a requisição RPC.
- `Request`: Representa a requisição RPC.
- `serde_json::json`: Cria valores JSON.
