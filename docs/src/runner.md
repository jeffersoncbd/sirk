### Resumo

O arquivo `src/runner.rs` implementa o motor de execução e retomada de workflows sequenciais. Ele valida workflows, coordena agentes e ferramentas, processa entradas interativas e persiste cada etapa em um histórico para permitir recuperação após interrupções.

### Funcionamento

A execução valida o `Workflow`, canonicaliza o diretório e carrega as configurações de todos os agentes encontrados recursivamente nos passos, incluindo loops e os dois ramos de condições. Essas configurações são armazenadas em um `Snapshot` junto com o workflow e o diretório, permitindo que retomadas futuras usem a configuração persistida.

O `Engine` percorre os passos em ordem e mantém um cursor alinhado aos registros do histórico. Entradas, respostas, resultados de ferramentas, confirmações e alterações são salvos progressivamente. Antes de continuar, o histórico é validado contra a estrutura do workflow, seus rótulos, condições, ramificações e resultados; registros posteriores a um passo pendente ou incompatíveis com o workflow são rejeitados.

Os passos implementados incluem:

- Agentes, que recebem instruções, entradas renderizadas, respostas anteriores e resultados de `TREE` e `READ`. Podem solicitar interação com `ASK:`, edição externa com `EDIT:` e exclusão externa com `DELETE:`, conforme as permissões da configuração do agente.
- Ferramentas internas `TREE`, `READ`, `WRITE`, `DELETE`, ferramentas customizadas e `AWAIT`, com persistência dos argumentos e resultados para evitar repetição após uma retomada.
- `LOOP`, que percorre arrays de strings e cria escopos locais contendo `loop.item` e outputs da iteração.
- `IF`, que persiste a condição avaliada e executa somente o ramo selecionado.
- `EDIT`, que prepara, valida, aplica e recupera alterações de arquivos, incluindo conflitos de versão, criação de arquivos ausentes e recuperação após falhas antes ou depois da publicação.

Pedidos externos de edição e exclusão são interpretados como JSON. Edições validam coordenadas, versão e conteúdo antes do commit; exclusões exigem confirmação, exceto quando autorizadas por `force` e pela permissão específica. Solicitações já concluídas são detectadas para impedir duplicação.

Erros são representados como `Result<_, String>` e propagados com `?`. Há validações explícitas para diretórios, agentes, adaptadores, permissões, respostas vazias, pedidos `ASK:`, `EDIT:` e `DELETE:` inválidos, arquivos, processos externos e histórico inconsistente. A execução de processos passa por `BashService` e `Invocation`, e a saída só é confirmada quando o processo termina com sucesso.

### Componentes principais

- `execute`: executa uma `Invocation` por meio de `BashService`, captura a saída e rejeita processos com status de erro.
- `run`, `resume`, `run_with` e `run_interactive_with`: iniciam execuções interativas, sem entrada, novas ou retomadas a partir de um histórico.
- `validate_snapshot`: valida o workflow, o diretório, as configurações dos agentes, os adaptadores e as restrições de conversas retomáveis.
- `all_steps`: percorre recursivamente passos comuns, loops e ramos condicionais para localizar agentes.
- `question`: reconhece respostas de agentes no formato `ASK:`.
- `external_edit_request` e `external_delete_request`: analisam pedidos externos e convertem seus payloads JSON em requisições tipadas.
- `prepare_external_edit`, `external_edit_pending` e `completed_external_edit`: preparam edições, recuperam edições pendentes e detectam pedidos já aplicados.
- `external_delete_pending` e `completed_external_delete`: validam exclusões pendentes e impedem sua repetição.
- `validate_blocks`, `validate_edit_blocks`, `validate_step_blocks` e `validate_agent_blocks`: verificam se os registros persistidos representam uma execução consistente e retomável.
- `continue_with`: valida o histórico e cria o `Engine` para continuar a execução.
- `Engine::run_steps`: controla a sequência dos passos, IDs hierárquicos, loops, condições, escopos locais e propagação de outputs.
- `Engine::run_step`: executa edições, confirmações, ferramentas e conversas com agentes, salvando cada transição no histórico.
- `ExternalDeleteRequest`: representa pedidos externos de exclusão, incluindo o caminho e a opção `force`.
- Módulo `tests`: cobre retomadas, perguntas, ferramentas, loops, condições, edições, exclusões, conflitos, arquivos, histórico e recuperação após falhas.

### integrações

O arquivo expõe as funções públicas `run`, `resume`, `run_with`, `run_interactive_with` e `continue_with`. Elas retornam `BTreeMap<String, String>` com outputs globais ou um erro textual; `run` e `run_interactive_with` usam entrada de terminal, enquanto `run_with` rejeita operações que exigem entrada interativa.

Ele integra os módulos internos de:

- `adapters`, para resolver adaptadores e construir invocações de agentes;
- `agents`, para carregar configurações persistidas;
- `harness`, para representar requisições aos agentes;
- `history`, para snapshots, blocos, rótulos e persistência;
- `input`, para entrada interativa e confirmações;
- `services`, para execução de processos externos;
- `workflow`, para passos, loops, condições, ramos e templates;
- `tools`, para leitura, escrita, edição, exclusão, ferramentas customizadas e operações `TREE`/`READ`.

As estruturas importadas e os adaptadores são definidos em outros módulos, que não fazem parte do conteúdo analisado.