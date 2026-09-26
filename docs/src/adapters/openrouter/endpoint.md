## Resumo
Resolve a URL completa do endpoint de chat da OpenRouter a partir de configuração, variável de ambiente ou arquivo `.env`.

## Funcionamento
A resolução segue a precedência: campo `base_url` do adaptador, depois a variável de ambiente `OPENROUTER_URL` e, por fim, o valor `OPENROUTER_URL` no `.env` do diretório informado, com fallback para `DEFAULT_URL`. O resultado é normalizado removendo barras finais; se ficar vazio, retorna `HarnessError::InvalidConfiguration`. O sufixo `/chat/completions` é preservado, `/api/v1` é apenas complementado e qualquer outro caminho recebe `/api/v1/chat/completions` — sempre como `Ok(String)`.

## Importações
- `DEFAULT_URL`: URL padrão usada quando nenhuma outra fonte define o endpoint.
- `OpenRouterAdapter`: Struct que implementa o método e fornece `base_url`, `dotenv_value` e `id`.
- `nonempty`: Filtro que descarta strings de ambiente vazias.
- `HarnessAdapter`: Trait que fornece a leitura do valor no `.env`.
- `HarnessError`: Tipo de erro retornado em caso de configuração inválida.
- `std::path::Path`: Representa o diretório usado para localizar o arquivo `.env`.
