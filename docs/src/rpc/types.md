## Resumo
Define os tipos usados para representar requisições, parâmetros, respostas e erros RPC.

## Funcionamento
`Request` desserializa mensagens JSON-RPC e usa parâmetros vazios quando `params` está ausente; `AgentParams` exige `agent` e `input` e rejeita campos desconhecidos. `Response` serializa o resultado ou erro, omitindo a opção ausente, e `RpcError` contém código e mensagem.

## Importações
- `serde`: Derivação de serialização e desserialização.
- `serde_json::Value`: Representação de valores JSON.
