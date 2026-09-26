## Resumo
Resolve a chave de API do Ollama Web priorizando valor explícito, depois variável de ambiente e por fim arquivo `.env`.

## Funcionamento
A função tenta primeiro a chave já configurada no adapter (`self.api_key`), retornando-a clonada. Se ausente, lê `OLLAMA_API_KEY` do ambiente; se a leitura falhar ou o valor for vazio, o resultado é `None`. Como último recurso, delega ao leitor de `.env` com o diretório informado. Erros de I/O na leitura do `.env` propagam como `HarnessError`; a ausência de configuração é um `Ok(None)`, não um erro.

## Importações
- `OllamaWebAdapter`: tipo receptor que expõe a chave já resolvida e a leitura de `.env`.
- `nonempty`: converte string vazia em `None`, descartando chaves em branco.
- `HarnessError`: tipo de erro propagado pela leitura de `.env`.
- `Path`: representa o diretório usado para localizar o arquivo `.env`.
