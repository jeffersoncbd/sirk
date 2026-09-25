### Resumo

O arquivo implementa o adaptador `OpenRouterAdapter`, responsável por transformar uma solicitação do harness em uma chamada HTTP não streaming à API de chat da OpenRouter usando o executável `curl`, além de interpretar a resposta JSON recebida.

### Funcionamento

O adaptador:

- Obtém o modelo da requisição e rejeita chamadas sem modelo.
- Não permite `event_stream`, pois trabalha apenas com respostas completas.
- Resolve a URL da API nesta ordem:
  1. URL configurada diretamente no adaptador;
  2. variável de ambiente `OPENROUTER_URL`;
  3. variável `OPENROUTER_URL` no arquivo `.env` do diretório de trabalho;
  4. URL padrão da OpenRouter.
- Obtém a chave da API de forma semelhante, usando `OPENROUTER_API_KEY`.
- Monta uma requisição `POST` com:
  - cabeçalho `Content-Type: application/json`;
  - autenticação Bearer;
  - modelo solicitado;
  - prompt como mensagem de usuário;
  - `"stream": false`.
- Mantém a chave fora dos argumentos do processo, armazenando o cabeçalho em uma variável de ambiente usada pelo `curl`.
- Usa `--fail-with-body` para preservar o corpo JSON de erros HTTP.
- Desserializa a resposta e retorna o conteúdo da mensagem da primeira escolha. Respostas JSON inválidas ou sem escolhas resultam em `HarnessError::InvalidResponse`.

A leitura e análise do `.env` convertem erros de arquivo ou parsing em `HarnessError::InvalidConfiguration`. Valores vazios são ignorados.

### Componentes principais

- `OpenRouterAdapter`: struct pública que armazena o executável, a URL opcional e a chave da API.
- `Default`: configura `curl` como executável padrão e deixa URL e chave para resolução posterior.
- `new`: constrói um adaptador com configurações explícitas, descartando valores vazios.
- `endpoint`: determina e normaliza o endpoint final `/chat/completions`.
- `api_key`: resolve a chave de autenticação.
- `dotenv_value`: lê uma variável específica do `.env`.
- `nonempty`: converte strings vazias ou compostas apenas por espaços em `None`.
- Implementação de `HarnessAdapter`:
  - `id`: identifica o adaptador como `"openrouter"`.
  - `invocation`: valida a requisição e cria a invocação do `curl`.
  - `response`: interpreta o JSON de resposta e extrai o conteúdo textual.
- Tipos locais `ChatCompletion`, `Choice` e `Message`: representam apenas a estrutura JSON necessária da resposta.
- Testes: verificam montagem da requisição, proteção da chave nos argumentos, leitura do `.env`, extração da resposta e validações de configuração.

### integrações

O arquivo expõe publicamente `OpenRouterAdapter` e seus métodos `new` e, por meio da implementação pública de `HarnessAdapter`, fornece operações para:

- identificar o adaptador;
- converter `RunRequest` em `Invocation`;
- converter a saída textual em uma resposta do harness.

Ele depende de `serde` e `serde_json` para desserialização e criação de JSON, `dotenvy` para leitura de `.env`, do executável externo `curl` e dos tipos internos `HarnessAdapter`, `HarnessError`, `Invocation` e `RunRequest`.
