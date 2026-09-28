## Resumo
O teste verifica se uma requisição HTTP adiciona ao stage do Git um arquivo no diretório informado.

## Funcionamento
Cria um repositório temporário, grava `generated.md` e envia uma requisição `POST` para `/v1/git/add`. Confirma que a resposta tem status HTTP 200 e campo `status` igual a `ok`, verifica que o arquivo foi staged e remove o diretório temporário. Erros de preparação ou execução causam falha no teste.

## Importações
- `handle`: processa a requisição HTTP de teste.
- `BashService`, `Invocation`: executam comandos Git no diretório do teste.
- `std::fs`: cria, grava e remove arquivos e diretórios.
- `std::io`: fornece o destino descartável para a saída do comando.
- `SystemTime`, `UNIX_EPOCH`: tornam único o nome do diretório temporário.
