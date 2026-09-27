## Resumo
Verifica se uma solicitação `agent.run` retorna o resultado produzido pelo agente.

## Funcionamento
Cria um projeto temporário com um agente e um adaptador simulado que responde a uma leitura; envia a solicitação e confirma que o resultado é `documented` e que não há erro. Ao final, remove o diretório temporário.

## Importações
- `handle`: Processa a solicitação RPC.
- `Request`: Estrutura da solicitação RPC.
- `serde_json`: Cria valores JSON para a solicitação.
- `std::fs`: Cria e remove arquivos e diretórios temporários.
- `PermissionsExt`: Define permissão executável no adaptador.
- `SystemTime`, `UNIX_EPOCH`: Geram um nome temporário único.
