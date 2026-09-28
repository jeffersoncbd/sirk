## Resumo
Adapta solicitações do harness para chamadas não-streaming à API de chat da OpenRouter e extrai o conteúdo da resposta.

## Funcionamento
`invocation` exige um modelo e rejeita fluxos de eventos; depois obtém a chave de API e monta uma chamada POST com prompt, modelo e autenticação no ambiente. Erros de configuração são propagados. `response` interpreta o JSON, retorna o conteúdo da primeira escolha ou um erro se a resposta for inválida ou não contiver escolhas.

## Importações
- `std::collections::BTreeMap`: Monta as variáveis de ambiente da chamada.
- `serde::Deserialize`: Interpreta a resposta JSON.
- `serde_json::json`: Monta o corpo JSON da solicitação.
- `crate::harness`: Fornece tipos e erros do adaptador.
