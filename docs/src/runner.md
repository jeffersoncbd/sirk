### Resumo

O arquivo `src/runner.rs` implementa o motor de execução e retomada de workflows sequenciais. Ele valida workflows e agentes, coordena conversas, ferramentas, entradas do usuário e edições de arquivos, mantendo um histórico persistente para recuperação após interrupções.

### Funcionamento

A execução valida o `Workflow`, canonicaliza o diretório e carrega todas as configurações de agentes presentes nos passos, inclusive em loops e ramificações condicionais. Em seguida, cria um `Snapshot` com o workflow, o diretório e os agentes — preservando essas configurações para retomadas futuras — e inicializa um `History` persistente.

O `Engine` percorre os passos em ordem, mantendo um cursor alinhado aos registros do histórico. Cada entrada, resposta, resultado de ferramenta e alteração é salvo progressivamente. Assim, uma falha deixa apenas o passo pendente para retomada, sem repetir etapas concluídas.

Os passos suportados incluem:

- Agentes, que recebem instruções, entradas renderizadas, respostas anteriores e resultados de `TREE`, `READ` ou `EDIT`. Podem solicitar entrada com `ASK:` e, quando autorizados pela configuração `edit_tool`, alterações externas com `EDIT:`.
- Ferramentas internas, como `TREE`, `READ`, `WRITE`, ferramentas customizadas e `AWAIT`.
- `LOOP`, que percorre arrays de strings e cria escopos locais com `loop.item` e outros outputs da iteração.
- `IF`, que avalia uma condição persistida e executa somente o ramo selecionado.
- `EDIT`, que prepara, valida, aplica e recupera alterações de arquivos, incluindo conflitos de versão e criação de arquivos ausentes.

Antes da retomada, o histórico é validado contra a estrutura atual do workflow. Registros inconsistentes, passos removidos, condições ou resultados alterados e registros posteriores a um passo pendente são rejeitados. Os resultados de agentes e ferramentas são propagados por outputs globais ou locais de loops.

Erros são representados como `Result<_, String>` e propagados com `?`, com validações explícitas para diretórios, agentes, adaptadores, permissões de edição, respostas vazias, pedidos `ASK:` ou `EDIT:` inválidos, arquivos e histórico incompatível. A execução de processos externos passa por `BashService` e `Invocation`, verificando o status do processo antes de confirmar sua saída.

### Componentes principais

- `execute`: executa uma `Invocation` com `BashService`, captura a saída e rejeita processos que terminam com erro.
- `run`, `resume`, `run_with` e `run_interactive_with`: iniciam execuções interativas, sem entrada, novas ou retomadas a partir de histórico.
- `validate_snapshot`: verifica o workflow, o diretório, agentes, adaptadores, permissões de edição e configurações de perguntas.
- `all_steps`: percorre recursivamente passos comuns, loops e os dois ramos de condições para localizar agentes.
- `external_edit_request`, `prepare_external_edit` e `completed_external_edit`: interpretam pedidos externos `EDIT:`, preparam alterações e detectam pedidos já aplicados.
- `continue_with`: valida o histórico e cria o `Engine` para continuar a execução.- `external_edit_request`, `prepare_external_edit` e `completed_external_edit`: interpretam pedidos externos `EDIT:`, preparam alterações e detectam pedidos já aplicados.- `continue_with`: valida o histórico e cria o `Engine` para continuar a execução.
- `Engine::run_steps`: controla a sequência, IDs hierárquicos, loops, condições, escopos locais e propagação de outputs.
- `Engine::run_step`: executa edições, confirmações, ferramentas e conversas com agentes, salvando cada transição no histórico.
- `question`: reconhece respostas de agentes no formato `ASK:`.
- Módulo `tests`: cobre retomada, perguntas, ferramentas, loops, condições, edições, conflitos, arquivos, histórico e recuperação após falhas.

### Integrações

O arquivo expõe as funções públicas `run`, `resume`, `run_with`, `run_interactive_with` e `continue_with`, que retornam `BTreeMap<String, String>` com outputs globais ou um erro textual. `run` e `run_interactive_with` usam entrada de terminal; `run_with` rejeita pedidos que exigem entrada; `resume` e `continue_with` retomam um `History` existente.

Ele integra os módulos internos de:

- `adapters`, para resolver adaptadores e construir invocações de agentes;
- `agents`, para carregar configurações persistidas;
- `harness`, para representar requisições aos agentes;
- `history`, para snapshots, blocos, labels e persistência;
- `input`, para entrada interativa e confirmações;
- `services`, para execução de processos externos;
- `workflow`, para passos, loops, condições, branches e templates;
- `tools`, para leitura, escrita, edição, ferramentas customizadas e operações `TREE`/`READ`.

Os detalhes das estruturas importadas e dos adaptadores dependem dos respectivos módulos, que não fazem parte do conteúdo analisado.