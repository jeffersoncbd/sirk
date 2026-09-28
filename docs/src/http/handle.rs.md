## Resumo
`handle` encaminha verificações de saúde e solicitações HTTP para execução de agentes e operações Git.

## Funcionamento
Responde `200` a `GET /health`; rejeita rotas desconhecidas com `404` e métodos inválidos com `405`. Nas rotas POST, desserializa o corpo conforme a operação: erros de formato retornam `400`, erros dos serviços retornam `500` e resultados bem-sucedidos retornam `200`.

## Importações
- `super::types`: Tipos das solicitações e da resposta HTTP.
- `serde_json`: Cria corpos de resposta em JSON.
