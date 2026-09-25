### Resumo

Este arquivo implementa o motor de execução de workflows da aplicação. Ele inicia workflows, carrega agentes, executa etapas de agentes ou ferramentas, mantém o histórico persistido e permite retomar execuções interrompidas com validação de consistência.

### Funcionamento

A execução começa por `run`, `run_with` ou `run_interactive_with`:

1. O workflow é validado.
2. O diretório de trabalho é canonicalizado.
3. As configurações dos agentes referenciados são carregadas de `.agents`.
4. É criado um `Snapshot` contendo diretório, workflow e agentes.
5. O histórico é criado e a execução é delegada a `continue_with`.

`continue_with` valida novamente o snapshot e os blocos já registrados no histórico. Em seguida, cria um `Engine`, que percorre as etapas e salva cada alteração no histórico antes ou depois de operações relevantes. Isso permite continuar a partir da última etapa pendente sem repetir etapas concluídas.

O mecanismo suporta:

- etapas de agentes, executadas por meio de um adapter e de uma `Invocation`;
- ferramentas internas, como `TREE`, `READ`, `WRITE`, `EDIT` e ferramentas customizadas;
- estruturas condicionais `IF`;
- iterações `LOOP`, com variáveis locais como `loop.item`;
- perguntas ao usuário através de `ASK:`;
- solicitações de `TREE` e `READ` feitas pelo agente;
- persistência e recuperação após falhas;
- edição de arquivos com preparação, validação de versão e commit recuperável.

Cada resposta, entrada do usuário, resultado de ferramenta e resultado de agente é registrado como um `Block` no histórico. O histórico também armazena rótulos como `Step 1.2 — nome`, permitindo detectar quando registros foram removidos ou alterados de forma incompatível.

### Componentes principais

- `execute`: executa uma `Invocation` usando `BashService`. Retorna a saída padrão apenas quando o processo termina com sucesso.

- `run`: inicia uma execução usando `TerminalInput` para interação com o usuário.

- `resume`: abre um arquivo de histórico e continua sua execução.

- `run_with`: versão não interativa, usando uma implementação `NoInput` que falha caso seja necessária entrada do usuário. É especialmente útil para testes.

- `run_interactive_with`: valida o workflow, carrega agentes, cria o snapshot e inicia o histórico.

- `validate_snapshot`: verifica se o workflow, diretório, agentes e adapters continuam válidos. Também rejeita agentes configurados para JSON ou com perguntas vazias, pois esse modo não é compatível com conversas retomáveis.

- `question`: identifica respostas de agentes no formato `ASK: pergunta`.

- `all_steps`: percorre recursivamente etapas normais, de loops e de branches condicionais para localizar todos os agentes.

- `validate_blocks`: verifica se o histórico corresponde à estrutura atual do workflow e se não existem registros posteriores a uma etapa editada.

- `validate_edit_blocks`: valida históricos de operações `EDIT`, incluindo o `Pending` serializado e o diff produzido.

- `validate_step_blocks`: valida a sequência de entradas, saídas, perguntas e resultados de `TREE`/`READ` em uma conversa.

- `continue_with`: ponto principal de retomada. Valida o estado persistido, executa todas as etapas pendentes e retorna os outputs finais em um `BTreeMap`.

- `Engine`: estrutura interna que mantém:
  - o `History` mutável;
  - a função de execução externa;
  - a fonte de entrada do usuário;
  - o cursor da etapa atual.

- `Engine::run_steps`: percorre etapas, gerencia IDs hierárquicos, escopos de loop, branches de `IF` e outputs globais ou locais.

- `Engine::run_step`: executa uma etapa individual:
  - aplica operações `EDIT`;
  - executa ferramentas internas ou customizadas;
  - monta prompts para agentes;
  - processa perguntas e solicitações `TREE`/`READ`;
  - salva cada novo bloco no histórico.

- Módulo de testes: contém testes abrangentes para retomada, perguntas, loops, condicionais, leitura e escrita de arquivos, edição recuperável, ferramentas customizadas, conflitos de versão e validação de históricos.

### Dependências e integrações

O arquivo depende de módulos internos:

- `adapters`: resolve adapters e constrói invocações para agentes.
- `agents::Agent`: carrega configurações dos agentes.
- `harness::RunRequest`: representa uma solicitação ao harness.
- `history`: fornece `History`, `Snapshot` e os tipos de bloco persistidos.
- `input`: fornece `TerminalInput` e o trait `UserInput`.
- `services`: executa processos externos por meio de `BashService` e `Invocation`.
- `workflow`: fornece `Workflow`, `Step`, condicionais, loops e resolução de variáveis.
- `tools`: executa `TREE`, `READ`, `WRITE`, `EDIT` e ferramentas customizadas.

Também usa `BTreeMap`, `Path` e `serde_json` para armazenar outputs, manipular caminhos e serializar operações pendentes de edição.

A comunicação com agentes é feita por processos externos. O arquivo constrói prompts contendo as instruções do agente e toda a conversa persistida, executa o adapter correspondente e registra a resposta recebida.

### Observações

- O arquivo não implementa diretamente o armazenamento do histórico, o parsing do workflow ou o comportamento individual das ferramentas; essas responsabilidades pertencem aos módulos importados.
- Falhas de execução, respostas vazias, histórico inconsistente, agentes ausentes e conflitos de edição são propagados como `Result::Err` com mensagens textuais.
- As operações são persistidas incrementalmente, permitindo retomar após interrupções sem repetir etapas já concluídas.
- Em loops, outputs intermediários são locais a cada iteração; somente outputs fora desse escopo são retornados no mapa final.
- O arquivo contém código de testes que cria diretórios temporários, arquivos, repositórios Git e configurações de agentes para simular os diferentes fluxos.
