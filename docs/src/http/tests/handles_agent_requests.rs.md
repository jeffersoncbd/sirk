## Resumo
Este arquivo contém apenas um teste que verifica a execução de um agente via HTTP.

## Funcionamento
O teste cria um agente temporário com um adaptador executável, envia uma requisição `POST` para `/v1/agent/run` e confirma o status e o resultado da resposta.

## Importações
- `handle`: processa a requisição HTTP no teste.
- `serde_json`: cria e interpreta os dados JSON.
- `std::fs`: cria e remove arquivos e diretórios temporários.
- `PermissionsExt`: torna o adaptador executável.
- `SystemTime`, `UNIX_EPOCH`: geram um nome temporário único.
