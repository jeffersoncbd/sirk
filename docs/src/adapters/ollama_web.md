## Resumo
O adaptador `OllamaWebAdapter` traduz pedidos em chamadas HTTP não-streaming à API do Ollama, montando um comando `curl` e extraindo o texto gerado.

## Funcionamento
`invocation` exige que a requisição tenha modelo (senão `MissingModel`) e recusa `event_stream` (`UnsupportedOption`). Monta os argumentos do `curl` com POST, header de JSON e o corpo `{"model", "prompt", "stream": false}`, resolving URL base e chave de API (via `.env` do diretório de trabalho ou configuração). A chave nunca vai nos argumentos: é injetada na variável `OLLAMA_WEB_AUTHORIZATION` e referenciada por expansão de shell, evitando vazamento em logs/ps. `response` desserializa o JSON retornado e extrai o campo `response`, convertendo falhas de parse em `InvalidResponse`.

## Importações
- `std::collections::BTreeMap`: mapa ordenado do ambiente passado à invocação
- `serde::Deserialize`: derive para ler o campo `response` do JSON
- `serde_json::json`: monta o corpo JSON da requisição sem struct auxiliar
- `crate::harness::{HarnessAdapter, HarnessError, Invocation, RunRequest}`: trait e tipos de domínio do harness
