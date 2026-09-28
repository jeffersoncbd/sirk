## Resumo
Lê do arquivo `.env` de um diretório o valor não vazio de uma chave.

## Funcionamento
Se o arquivo não existir ou a chave não for encontrada, retorna `Ok(None)`. Erros de leitura ou análise são convertidos em `HarnessError::InvalidConfiguration`; ao encontrar a chave, retorna seu valor se não estiver vazio.

## Importações
- `OllamaWebAdapter`: fornece o identificador do adaptador.
- `nonempty`: converte valores vazios em `None`.
- `HarnessAdapter`: permite obter o identificador do adaptador.
- `HarnessError`: representa erros de configuração.
- `io`: identifica erros de arquivo inexistente.
- `Path`: representa o diretório consultado.
- `dotenvy`: lê e analisa o arquivo `.env`.
