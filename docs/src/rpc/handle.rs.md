## Resumo
Processa uma requisição JSON-RPC para executar o agente e monta a resposta correspondente.

## Funcionamento
Valida a versão JSON-RPC e o identificador, verifica se o método é `agent.run` e converte os parâmetros; falhas retornam erros específicos. Em seguida, executa o agente no diretório informado e retorna o resultado ou o erro recebido.

## Importações
- `super::types`: Tipos da requisição, resposta, parâmetros e erro RPC.
- `serde_json::Value`: Representa o identificador JSON-RPC.
- `std::path::Path`: Tipo do caminho do diretório de execução.
