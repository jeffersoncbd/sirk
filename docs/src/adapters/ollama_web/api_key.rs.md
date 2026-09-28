## Resumo
Obtém a chave da API do Ollama, buscando-a nas fontes disponíveis.

## Funcionamento
Prioriza a chave já configurada no adaptador; em seguida, consulta `OLLAMA_API_KEY` no ambiente e, por fim, no arquivo dotenv do diretório. Valores vazios são tratados como ausentes. Erros da leitura do dotenv são propagados.

## Importações
- `super`: Adaptador e validador de valores não vazios.
- `crate::harness::HarnessError`: Tipo de erro retornado.
- `std::path::Path`: Representa o diretório consultado.
