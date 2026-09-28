## Resumo
Lê do arquivo `.env` de um diretório o valor não vazio de uma chave.

## Funcionamento
Se `.env` não existir, retorna `Ok(None)`; outros erros de leitura ou análise viram `HarnessError::InvalidConfiguration`. Ao encontrar a chave, retorna seu valor se não estiver vazio; se não encontrar, retorna `Ok(None)`.

## Importações
- `super`: Acesso ao adaptador e à função que filtra valores vazios.
- `crate::harness`: Tipos usados para identificar erros de configuração.
- `std`: Verificação de erro de arquivo ausente e caminho do diretório.
- `dotenvy`: Leitura e análise das variáveis do arquivo `.env`.
