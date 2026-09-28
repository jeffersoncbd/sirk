## Resumo
`handle` atende verificações de saúde e solicitações HTTP para executar um agente.

## Funcionamento
Responde `200` a `GET /health`; para outras rotas, retorna `404`, e para métodos diferentes de `POST` em `/v1/agent/run`, retorna `405`. Nessa rota, desserializa o corpo como `AgentRequest`: falhas retornam `400`; a execução do agente retorna `200` com o resultado ou `500` com o erro.

## Importações
- `super::types`: Tipos da solicitação e da resposta HTTP.
- `serde_json`: Cria e serializa os corpos JSON.
