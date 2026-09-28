## Resumo
Monta o endpoint de geração do Ollama Web a partir da URL configurada.

## Funcionamento
Prioriza a URL do adaptador, depois a variável de ambiente e, por fim, o valor do arquivo dotenv ou o padrão. Remove barras finais, retorna erro se a URL ficar vazia e acrescenta o caminho necessário, exceto quando já termina em `/api/generate`.

## Importações
- `super`: Tipos, URL padrão e validação de valor não vazio.
- `crate::harness`: Trait do adaptador e tipo de erro.
- `std::path::Path`: Caminho usado para consultar o arquivo dotenv.
