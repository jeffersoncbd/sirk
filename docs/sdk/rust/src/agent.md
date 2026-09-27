## Resumo
Envia uma solicitação `agent.run` ao CLI e retorna o resultado como texto.

## Funcionamento
Gera um identificador de requisição, verifica se ele não excede o limite e se a entrada do CLI está aberta. Envia a solicitação JSON-RPC pela entrada, lê uma linha da saída e valida a versão do protocolo e o identificador. Retorna erros de transporte, protocolo ou remotos; se a resposta não tiver resultado nem erro, retorna erro de protocolo.

## Importações
- `crate`: Tipos de erro, cliente e estruturas do protocolo.
- `std::io`: Leitura da resposta e escrita da solicitação.
- `serde_json`: Serialização e desserialização JSON.
