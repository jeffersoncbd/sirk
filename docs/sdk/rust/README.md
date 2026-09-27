## Resumo
Apresenta o SDK Rust para iniciar o S.I.R.K. e executar agentes pelo protocolo JSON-RPC.

## Funcionamento
`Sirk::start` inicia `sirk rpc` no diretório do projeto; `agent` envia a solicitação `agent.run` e retorna a resposta. As mensagens usam uma linha JSON cada, são processadas sequencialmente e interações que exigem entrada do usuário retornam erro RPC.

## Importações
- `sirk_sdk`: fornece o cliente `Sirk`.
