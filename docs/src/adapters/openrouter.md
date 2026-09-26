## Resumo
Adapta requisições do harness em chamadas `curl` não-streaming à API de chat do OpenRouter.

## Funcionamento
`invocation` exige `model` (erro `MissingModel`) e rejeita `event_stream` (`UnsupportedOption`); monta um `POST` via curl com `--fail-with-body`, corpo JSON `stream: false` e o header de autorização injetado por variável de ambiente (`OPENROUTER_AUTHORIZATION`), evitando expor a chave na linha de comando. URL e chave vêm de `new`/`.env` do diretório de trabalho, com erro `InvalidConfiguration` se ausentes. `response` desserializa o JSON e devolve o `content` da primeira choice, ou `InvalidResponse` em falha de parse ou lista vazia. Efeitos colaterais: apenas leitura do `.env`; nenhuma chamada de rede (delegada ao processo filho).

## Importações
- `std::collections::BTreeMap`: mapa ordenado do ambiente passado ao processo filho
- `serde::Deserialize`: deriva structs de parsing da resposta JSON
- `serde_json::json`: monta o corpo da requisição e converte para string
- `crate::harness::{HarnessAdapter, HarnessError, Invocation, RunRequest}`: trait, erros e tipos de entrada/saída do harness
