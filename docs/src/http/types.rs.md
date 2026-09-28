## Resumo
Define os tipos para solicitações de agente e diretório e para respostas HTTP.

## Funcionamento
`AgentRequest` reúne diretório, agente e entrada; `DirectoryRequest` contém o diretório. Ambos rejeitam campos desconhecidos durante a desserialização. `HttpResponse` armazena o código de status e o corpo.

## Importações
- `serde`: desserializa solicitações e rejeita campos desconhecidos.
- `std::path::PathBuf`: representa diretórios nas solicitações.
