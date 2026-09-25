### Resumo

O arquivo `src/runner.rs` implementa o motor de execução de workflows sequenciais. Ele coordena agentes, ferramentas, entradas do usuário, histórico persistente e retomada de execuções interrompidas.

### Funcionamento

A execução começa validando o `Workflow`, canonicalizando o diretório e carregando as configurações dos agentes definidos nos passos. Em seguida, cria um `Snapshot` com o workflow, diretório e agentes, valida esse estado e inicializa um `History`.

O `Engine` percorre os passos em ordem e mantém um cursor alinhado ao histórico. Cada passo é registrado antes ou depois de sua execução, permitindo retomar a execução sem repetir etapas concluídas.

O arquivo trata diferentes tipos de passos:

- Agentes: constroem prompts com instruções, entradas, respostas anteriores e resultados de `TREE`/`READ`. Podem fazer perguntas ao usuário através de respostas `ASK:`.
- Ferramentas: executam operações como `TREE`, `READ`, `WRITE`, ferramentas customizadas e `EDIT`.
- `LOOP`: percorre arrays de strings, criando escopos locais com `loop.item` e preservando resultados por iteração.
- `IF`: avalia uma condição e executa apenas o ramo correspondente.
- `EDIT`: prepara, valida e aplica alterações de arquivos de forma recuperável, registrando o diff no histórico.

O histórico é validado antes da retomada. Registros inconsistentes, passos removidos ou resultados editados com etapas posteriores são rejeitados. Resultados e entradas são salvos progressivamente; falhas deixam o ponto pendente para uma execução posterior.

Erros são propagados por `Result<String, String>` e pelo operador `?`. Também há validações explícitas para agentes inexistentes, adaptadores desconhecidos, respostas vazias, arquivos inválidos e histórico incompatível. A execução externa é feita por `BashService` através de `Invocation`, verificando o status do processo.

### Componentes principais

- `execute`: executa uma `Invocation` usando `BashService`, captura a saída e rejeita processos com status de erro.
- `run`: inicia uma execução interativa usando entrada do terminal.
- `resume`: abre um histórico existente e continua sua execução.
- `run_with`: executa sem interação, rejeitando perguntas que exigiriam entrada do usuário.
- `run_interactive_with`: valida o workflow, carrega agentes e cria o snapshot inicial.
- `validate_snapshot`: verifica diretório, agentes, adaptadores, configurações `ask` e compatibilidade com histórico.
- `all_steps`: percorre recursivamente passos normais, loops e ramificações condicionais.
- `validate_blocks`, `validate_edit_blocks` e `validate_step_blocks`: verificam se os registros persistidos correspondem à estrutura esperada do workflow.
- `continue_with`: valida o histórico e inicia o `Engine`.
- `Engine::run_steps`: percorre passos, controla IDs hierárquicos, escopos locais, loops, condições e propagação de outputs.
- `Engine::run_step`: executa passos de edição, ferramentas e agentes, atualizando o histórico.
- `question`: reconhece respostas de agentes no formato `ASK:`.
- Módulo `tests`: contém testes de retomada, perguntas, ferramentas, loops, condições, edições, arquivos e recuperação após falhas.

### integrações

O arquivo expõe as funções públicas `run`, `resume`, `run_with`, `run_interactive_with` e `continue_with`, que retornam `BTreeMap<String, String>` com os outputs finais ou um erro textual.

Ele integra os módulos internos de:

- `adapters`, para resolver adaptadores e criar invocações de agentes;
- `agents`, para carregar configurações;
- `harness`, para representar requisições aos agentes;
- `history`, para snapshots, blocos e persistência;
- `input`, para entrada interativa;
- `services`, para execução de processos externos;
- `workflow`, para passos, loops, condições e templates;
- `tools`, para leitura, escrita, edição, execução de ferramentas e solicitações `TREE`/`READ`.

O comportamento detalhado dessas estruturas depende dos módulos importados, que não estão incluídos no conteúdo analisado.
