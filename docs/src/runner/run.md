## Resumo
Executa um `Workflow` de forma interativa e devolve as variáveis de saída coletadas.

## Funcionamento
`run` é um ponto de entrada fino: delega a execução para `run_interactive_with`, passando o workflow, o diretório de trabalho, a função `execute` (passada como callback) e uma instância de `TerminalInput` como fonte de entrada. Todo o fluxo interativo — leitura de comandos, chamadas a `execute` e coleta de resultados — acontece ali; o único tratamento de erro é o `Result<BTreeMap<String, String>, String>` propagado sem transformação, mapeando falhas internas para mensagens de string.

## Importações
- `crate::input::TerminalInput`: Fornece a leitura de entrada do usuário durante a execução interativa.
- `crate::workflow::Workflow`: Define o fluxo de trabalho a ser executado.
- `std::collections::BTreeMap`: Tipa o mapa ordenado de variáveis de saída retornado.
- `std::path::Path`: Representa o diretório de trabalho usado na execução.
- `super::execute::execute`: Callback que executa um passo do workflow e gera as saídas.
- `super::run_interactive::run_interactive_with`: Orquestra o loop interativo que combina workflow, input e execução.
