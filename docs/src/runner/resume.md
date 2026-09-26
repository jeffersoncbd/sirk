## Resumo
Retoma uma execução pausada, reexecutando os comandos restantes do histórico.

## Funcionamento
Abre o histórico persistido no caminho indicado, invoca `continue_with` sobre ele passando `execute` como executor e `TerminalInput` como fonte de entrada, e devolve um `BTreeMap` de pares chave/valor. Qualquer falha ao abrir o histórico ou durante a continuação é propagada como `Err(String)`, interrompendo a retomada sem estado parcial exposto ao chamador.

## Importações
- `crate::history::History`: Abre e carrega o histórico de comandos Previously salvo em disco.
- `crate::input::TerminalInput`: Fornece a leitura da entrada do usuário durante a retomada.
- `super::execute::execute`: Callback que executa cada comando retomado do histórico.
- `super::execution::continue_with`: Orquestra a iteração dos comandos e o tratamento de erros.
- `std::collections::BTreeMap`: Agrupa ordenadamente o resultado retornado pela execução.
- `std::path::Path`: Representa o caminho do arquivo de histórico, sem melakukan I/O.
