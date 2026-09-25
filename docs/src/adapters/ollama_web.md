### Resumo

O arquivo implementa o adaptador `ollama-web`, responsável por transformar uma solicitação do harness em uma chamada HTTP não streaming à API web do Ollama, executada por meio do `curl`. Também interpreta a resposta JSON e extrai o texto gerado.

### Funcionamento

O adaptador:

- Usa `curl` por padrão, mas permite configurar outro executável.
- Determina a URL da API nesta ordem:
  1. URL fornecida ao construtor;
  2. variável de ambiente `OLLAMA_WEB_URL`;
  3. valor correspondente no arquivo `.env` do diretório de execução;
  4. URL padrão `https://ollama.com/api`.
- Normaliza a URL para terminar em `/api/generate`.
- Obtém a chave em `OLLAMA_API_KEY`, também priorizando configuração direta, variável de ambiente e `.env`.
- Exige que exista um modelo em `RunRequest`.
- Rejeita requisições com `event_stream = true`, pois o adaptador trabalha apenas sem streaming.
- Monta uma requisição `POST` com conteúdo JSON contendo `model`, `prompt` e `"stream": false`.
- Mantém a chave de API fora dos argumentos visíveis, colocando o cabeçalho de autorização no ambiente da execução.
- Converte a saída JSON em texto, extraindo o campo `response`.
- Retorna `HarnessError` para modelo ausente, streaming não suportado, configuração inválida, URL vazia ou resposta JSON inválida.

Os testes verificam a montagem da invocação, o tratamento da chave de API, a extração da resposta e a leitura de valores a partir de `.env`.

### Componentes principais

- `OllamaWebAdapter`: struct pública que armazena o executável, a URL base opcional e a chave de API opcional.
- `Default`: configura `curl` como executável padrão e deixa URL e chave sem configuração explícita.
- `new`: construtor público que recebe executável, URL base e chave opcional.
- `endpoint`: função privada que resolve e normaliza o endpoint `/api/generate`.
- `api_key`: função privada que localiza a chave de API nas configurações disponíveis.
- `dotenv_value`: função privada que lê uma variável específica do arquivo `.env`.
- `nonempty`: função privada que transforma strings vazias ou apenas com espaços em `None`.
- `HarnessAdapter for OllamaWebAdapter`:
  - `id`: identifica o adaptador como `"ollama-web"`;
  - `invocation`: cria uma `Invocation` com programa, argumentos, diretório de trabalho e ambiente;
  - `response`: desserializa o JSON e retorna o campo `response`.
- `GenerateResponse`: struct privada usada apenas para desserializar a resposta da API.
- Módulo `tests`: contém testes unitários para requisições, respostas inválidas e configuração via `.env`.

### integrações

O arquivo integra-se com:

- `crate::harness::{HarnessAdapter, HarnessError, Invocation, RunRequest}`, que fornece o contrato do adaptador, os erros, a representação da execução e os dados da requisição.
- `serde::Deserialize` e `serde_json`, usados para desserializar respostas e construir o corpo JSON.
- `dotenvy`, usado para ler configurações no arquivo `.env`.
- `curl`, tratado como processo externo para realizar a chamada HTTP.
- Variáveis de ambiente `OLLAMA_WEB_URL` e `OLLAMA_API_KEY`.

A struct `OllamaWebAdapter` e seu construtor `new` são públicos. A implementação pública de `HarnessAdapter` expõe a identificação, a criação da invocação e o processamento da resposta.
