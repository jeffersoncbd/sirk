## Resumo
Avança a execução de um passo de workflow persistindo cada troca em blocos de histórico e convertendo respostas do agente em chamadas de ferramentas.

## Funcionamento
O `Engine::run_step` carrega referências mutáveis de histórico, executor de invocação e fonte de entrada do usuário. Conforme o `Step`, ele delega a behaviours-tools (EDIT, AWAIT, ASK, WRITE, DELETE, ferramentas padrão e customizadas) ou monta um prompt com as instruções do agente,加上 toolkits opcionais (TREE, READ, EDIT, DELETE) e a transcrição da conversa, converte a resposta em blocos, executa ferramentas solicitadas pelo modelo em ciclos sucessivos e persiste cada alteração com `history.save()`. Falhas de serialização, de ferramentas e de permissões ausentes (ex.: EDIT/DELETE sem flag no agente) retornam `Err(String)`; respostas vazias do modelo abortam o passo mantendo o input pendente.

## Importações
- `adapters`: resolve o adaptador do agente e monta a requisição.
- `harness::RunRequest`: dados da chamada ao modelo (prompt, diretório, modelo).
- `history::{Block, History}`: blocos de conversa e persistência do passo.
- `input::UserInput`: sourcing de respostas e confirmações do usuário.
- `services::Invocation`: descrição da chamada ao modelo com prefixo.
- `workflow::Step`: definição do passo (tool, agente, flags, input).
- `continue_with` / `run_steps`: módulos irmãos reexportados por este arquivo.
- `super::external::*`: detecção/preparação de pedidos externos de EDIT/DELETE.
- `super::question::question`: extrai perguntas `ASK` da resposta do agente.
- `std::collections::BTreeMap`: mapa ordenado de saídas e variáveis locais.
- `crate::tools`: escrita, leitura, delete, edição e execução de ferramentas.
