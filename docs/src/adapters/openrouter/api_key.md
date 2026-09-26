## Resumo
Resolve a chave de API do OpenRouter, usando campo interno, variável de ambiente ou arquivo `.env`.

## Funcionamento
Tenta primeiro a chave já configurada no adaptador; se ausente, lê `OPENROUTER_API_KEY` do ambiente descartando valores vazios. Se ambas falharem, consulta o arquivo `.env` no diretório informado (propagando `HarnessError` da leitura) e, não findingando valor, retorna `HarnessError::InvalidConfiguration` com o id do adaptador. Valores em branco são sempre descartados via `nonempty`.

## Importações
- `super::OpenRouterAdapter`: Adaptador que expõe a configuração e o `id()` usados na resolução.
- `super::nonempty::nonempty`: Filtro que descarta strings vazias ou só com espaços.
- `crate::harness::HarnessAdapter`: Trait que fornece `dotenv_value` e `id` para ler configuração.
- `crate::harness::HarnessError`: Tipo de erro retornado em falha de leitura ou ausência da chave.
- `std::path::Path`: Define o diretório onde o `.env` é procurado.
