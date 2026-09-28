## Resumo
O arquivo contém apenas um teste que verifica a edição de um arquivo por um agente via HTTP.

## Funcionamento
O teste cria um diretório temporário, configura um adaptador simulado e chama `handle` com uma requisição para executar o agente. Em seguida, verifica a resposta e o conteúdo editado, removendo o diretório ao final.

## Importações
- `handle`: Executa a requisição HTTP simulada.
- `std::fs`: Cria, lê, altera e remove arquivos de teste.
- `PermissionsExt`: Define o adaptador como executável.
- `SystemTime` e `UNIX_EPOCH`: Geram um nome único para o diretório temporário.
