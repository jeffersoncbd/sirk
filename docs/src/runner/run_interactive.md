## Resumo
Prepara e executa um workflow interativo, devolvendo um mapa de resultados por passo.

## Funcionamento
Valida o workflow, canonicaliza o diretório de trabalho e carrega cada agente citado nos passos (`.agents/<id>`), evitando duplicatas via `BTreeMap`. Em seguida monta um `Snapshot` (diretório, clone do workflow e agentes), valida esse snapshot e o entrega a `continue_with`, que conduz o histórico com o executor fornecido e o leitor de entrada do usuário. Erros de IO ou de validação são propagados como `String`.

## Importações
- `crate::agents::Agent`: Carrega a definição de cada agente referenciado pelos passos
- `crate::history::History`: Cria e conduz o histórico da execução interativa
- `crate::history::Snapshot`: Consolida diretório, workflow e agentes validados
- `crate::input::UserInput`: Abstrata a leitura de respostas do usuário durante o fluxo
- `crate::services::Invocation`: Representa cada invocação de passo enviada ao executor
- `crate::workflow::Workflow`: Define os passos e regras do fluxo a executar
- `std::collections::BTreeMap`: Deduplica agentes e armazena os resultados por passo
- `std::path::Path`: Representa o diretório base usado para montar o snapshot
- `super::all_steps::all_steps`: Achata os passos do workflow, incluindo os aninhados
- `super::execution::continue_with`: Executa os passos interativos sobre o histórico
- `super::validate_snapshot::validate_snapshot`: Garante Consistência do snapshot antes da execução
