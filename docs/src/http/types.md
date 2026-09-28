## Resumo
Define os tipos usados para representar uma solicitação e uma resposta HTTP.

## Funcionamento
`AgentRequest` contém o diretório, o agente e a entrada, rejeitando campos desconhecidos na desserialização. `HttpResponse` armazena o código de status e o corpo da resposta.

## Importações
- `serde`: desserializa a solicitação.
- `std::path::PathBuf`: representa o caminho do diretório.
