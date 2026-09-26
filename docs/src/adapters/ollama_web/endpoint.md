## Resumo
Resolve a URL completa do endpoint de geração do Ollama Web a partir de configuração, variável de ambiente ou arquivo `.env`, com fallback padrão.

## Funcionamento
A函数 prioriza a URL definida no adapter; na ausência, tenta a variável de ambiente `OLLAMA_WEB_URL` (descartando valores vazios) e, por fim, o `.env` do diretório informado, caindo em `DEFAULT_URL`. O valor é normalizado removendo barras finais e rejeitado com `HarnessError::InvalidResponse` se ficar vazio. Depois, normaliza o sufixo: `/api/generate` é mantido, `/api` recebe `/generate` e qualquer outro caminho recebe `/api/generate`. Retorna sempre `Ok(String)`, sem efeitos colaterais além da leitura de configuração.

## Importações
- `super::DEFAULT_URL`: URL padrão usada quando nenhuma configuração define o endpoint
- `super::OllamaWebAdapter`: Adapter receptor, fornece `base_url`, `dotenv_value` e `id`
- `super::nonempty::nonempty`: Filtro que descarta strings de ambiente vazias
- `crate::harness::HarnessAdapter`: Trait que expõe `dotenv_value` e `id` ao adapter
- `crate::harness::HarnessError`: Tipo de erro retornado quando a URL resulta vazia
- `std::path::Path`: Tipo do diretório usado para localizar o arquivo `.env`
