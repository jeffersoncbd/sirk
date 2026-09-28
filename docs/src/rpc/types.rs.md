## Resumo
Define os tipos usados para receber e representar mensagens RPC e erros.

## Funcionamento
O arquivo não contém uma função principal; declara estruturas para requisições, parâmetros de agente, respostas e erros RPC. Parâmetros ausentes recebem um valor padrão, campos desconhecidos em `AgentParams` são rejeitados e campos opcionais ausentes em `Response` não são serializados.

## Importações
- `serde`: Desserializa requisições e serializa respostas.
- `serde_json::Value`: Representa valores JSON arbitrários.
