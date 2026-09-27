## Resumo
Valida uma requisição RPC e encaminha `agent.run`, convertendo resultados e erros em uma resposta.

## Funcionamento
Usa o ID da requisição ou `null` e verifica a versão RPC e o formato do ID. Rejeita métodos desconhecidos e parâmetros inválidos com erros específicos; se forem válidos, chama `run_agent` com o diretório, o agente e a entrada, retornando o resultado ou um erro de execução.

## Importações
- `run_agent`: executa o agente solicitado.
- `AgentParams`, `Request`, `Response`, `RpcError`: tipos da requisição e resposta RPC.
- `serde_json::Value`: representa o ID e os parâmetros JSON.
- `std::path::Path`: representa o diretório de execução.
