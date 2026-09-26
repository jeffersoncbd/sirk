## Resumo
Executa uma lista de passos do workflow, tratando recursivamente loops e condicionais, e persistindo o resultado no histórico.

## Funcionamento
Percorre os passos atribuindo a cada um um id hierárquico (`prefix` + posição) e reserving uma entrada no histórico; na primeira visita renderiza e valida o argumento do passo (persistindo-o) para garantir reuso em reexecuções. Passos `LOOP` expandem os itens da expressão e executam recursamente o corpo, isolating outputs e injetando `item` como local; `IF` avalia a condição e executa apenas o ramo selecionado. Demais passos são executados por `run_step`, com conteúdo enumerado quando a ferramenta é `READ` com enumeração, e os valores de saída são gravados como locais do loop (via `loop_target`) ou no mapa global de outputs. Falhas de renderização, validação de expressão, persistência e execução propagam como `Err(String)`.

## Importações
- `super::Engine`: Engine genérica que hospeda o estado de execução
- `crate::history::Block`: Bloco persistido no histórico (entrada ou resultado)
- `crate::input::UserInput`: Trait do provedor de entrada do usuário
- `crate::services::Invocation`: Invocação de serviço passada ao executor externo
- `crate::workflow::Step`: Definição de um passo (tool, input, output, ramos)
- `crate::workflow::condition`: Valida e avalia a expressão da condicional
- `crate::workflow::loop_items`: Valida e expande a expressão de iteração
- `crate::workflow::loop_target`: Identifica nomes de saída ligados ao escopo do loop
- `std::collections::BTreeMap`: Mapa ordenado de saídas e variáveis locais
