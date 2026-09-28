## Resumo
Define os tipos usados para representar uma solicitação de agente e uma resposta HTTP.

## Funcionamento
`AgentRequest` contém o diretório, o agente e a entrada, e a desserialização rejeita campos desconhecidos. `HttpResponse` armazena o código de status e o corpo da resposta.

## Importações
- `serde`: desserializa a solicitação e rejeita campos desconhecidos.
- `std::path::PathBuf`: representa o diretório da solicitação.
