## Resumo
Verifica se uma solicitação `agent.run` retorna o resultado produzido pelo adaptador.

## Funcionamento
Cria um projeto temporário com um adaptador simulado e uma definição de agente. Envia a solicitação `agent.run` para `handle` e confirma que a resposta contém `"documented"` e nenhum erro; ao final, remove o diretório temporário.

## Importações
- `super::super`: acesso a `handle` e `Request`.
- `serde_json`: criação dos parâmetros e do identificador JSON.
- `std::fs`: criação e remoção dos arquivos temporários.
- `std::os::unix::fs::PermissionsExt`: permissão executável do adaptador.
- `std::time`: nome temporário único baseado no horário.
