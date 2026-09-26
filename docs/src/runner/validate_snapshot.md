## Resumo
Valida se um snapshot de execução persistido ainda está íntegro e pode ser retomado.

## Funcionamento
A função recebe um `Snapshot` e primeiro valida a estrutura do workflow via `workflow.validate()`. Em seguida confirma que o diretório de execução ainda existe, garantindo que o estado em disco não foi removido. Depois, itera sobre todos os passos do workflow (`all_steps`); para cada passo que tenha um agente associado, procura a configuração do agente pelo id (erro se não encontrado), confirma que o adapter referenciado existe entre os adapters registrados e valida que o agente não usa streaming de eventos JSON (incompatível com conversas retomáveis) nem um prompt `ask` vazio ou apenas com espaços. Retorna `Ok(())` em caso de sucesso, ou `Err(String)` com mensagem descritiva para a primeira falha encontrada.

## Importações
- `crate::adapters`: Resolução de adapters para validar que o adapter do agente existe.
- `crate::history::Snapshot`: Tipo do snapshot de execução, fornecendo workflow, diretório e agentes.
- `super::all_steps::all_steps`: Itera sobre os passos do workflow incluindo variações aninhadas.
