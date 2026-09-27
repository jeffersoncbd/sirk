## Resumo
Executa um workflow sem entrada do usuário e retorna os resultados por etapa.

## Funcionamento
Encaminha o workflow, o diretório e a função de execução para `run_interactive_configured_with`, usando `NoInput` e desativando a interação; propaga o mapa de resultados ou o erro.

## Importações
- `Invocation`: Tipo recebido pela função de execução.
- `Workflow`: Define o workflow a executar.
- `BTreeMap`: Armazena os resultados por etapa.
- `Path`: Representa o diretório de execução.
- `run_interactive_configured_with`: Executa o workflow configurado.
- `NoInput`: Indica que não há entrada do usuário.
