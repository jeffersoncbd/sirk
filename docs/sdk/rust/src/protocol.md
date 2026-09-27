## Resumo
Define os tipos usados para representar mensagens JSON-RPC de solicitação e resposta.

## Funcionamento
`Request` e `AgentParams` descrevem dados serializáveis de uma solicitação; `Response` e `RpcError` descrevem dados desserializáveis de uma resposta. O resultado e o erro são opcionais.

## Importações
- `serde`: Fornece derivações de serialização e desserialização.
