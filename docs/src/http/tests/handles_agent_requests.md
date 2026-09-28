## Resumo
Testa se a rota HTTP executa um agente e retorna seu resultado.

## Funcionamento
Cria um agente temporário executável, envia uma requisição `POST` para `/v1/agent/run` e confirma que a resposta tem status 200 e contém a saída esperada. Ao final, remove os arquivos temporários.

## Importações
- `super::super::handle`: Executa a requisição HTTP simulada.
- `serde_json`: Cria e interpreta o corpo JSON.
- `std::fs`: Cria e remove os arquivos temporários.
- `PermissionsExt`: Define o agente como executável.
- `SystemTime`, `UNIX_EPOCH`: Gera um nome temporário único.
