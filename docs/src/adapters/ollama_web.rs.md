## Resumo
Converte solicitações em chamadas não streaming à API web do Ollama e extrai a resposta recebida.

## Funcionamento
`invocation` exige um modelo, rejeita solicitações com fluxo de eventos e monta uma chamada POST com prompt e modelo; inclui autenticação quando há chave disponível e obtém o endpoint configurado. `response` lê o campo `response` do JSON ou retorna erro se o conteúdo for inválido.

## Importações
- `api_key`, `default`, `dotenv`, `endpoint`, `new`, `nonempty`: Configuração e criação do adaptador.
- `std::collections::BTreeMap`: Armazena variáveis de ambiente da chamada.
- `serde::Deserialize`: Desserializa a resposta JSON.
- `serde_json::json`: Monta o corpo JSON da solicitação.
- `crate::harness`: Tipos e trait para chamadas do harness.
