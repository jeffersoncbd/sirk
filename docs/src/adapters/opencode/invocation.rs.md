## Resumo
Monta a invocação do OpenCode para executar uma solicitação do harness.

## Funcionamento
Começa com o comando `run` e define um título fixo, sem habilitar aprovação automática. Inclui formato JSON se houver fluxo de eventos e modelo se informado; depois separa os argumentos do prompt com `--`. Retorna programa, argumentos e diretório de trabalho da solicitação, sem variáveis de ambiente adicionais.

## Importações
- `super::OpenCodeAdapter`: Adaptador que fornece o executável.
- `HarnessAdapter`: Define a implementação da invocação.
- `HarnessError`: Tipo de erro do harness.
- `Invocation`: Estrutura da chamada ao processo.
- `RunRequest`: Contém opções e dados da solicitação.
