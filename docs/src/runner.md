### Resumo

Este arquivo implementa o executor de workflows da aplicação. Ele coordena etapas com agentes, ferramentas internas e loops, mantém o histórico persistido da execução e permite interromper, retomar ou reconstruir uma execução a partir desse histórico.

### Funcionamento

A execução começa por `run` ou `run_with`:

1. O workflow é validado.
2. O diretório de execução é canonicalizado.
3. Todos os agentes referenciados, inclusive dentro de loops, são carregados de `.agents`.
4. É criado um `Snapshot` com diretório, workflow e configurações dos agentes.
5. Um objeto `History` é criado ou aberto.
6. `Engine::run_steps` percorre as etapas sequencialmente.

Cada etapa recebe um índice hierárquico, como `Step 1`, `Step 1.2.1`, e seus blocos de conversa ou resultados são salvos antes e depois das operações relevantes. Isso permite retomar uma execução sem repetir etapas já concluídas.

Etapas podem:

- Invocar um agente através de um adapter e um `RunRequest`.
- Executar ferramentas como `TREE`, `READ` e `WRITE`.
- Executar um `custom-tool`.
- Executar loops (`LOOP`) sobre arrays de strings.
- Produzir valores nomeados para uso posterior via templates como `{{ outputs.name }}` ou `{{ loop.item }}`.

Durante uma conversa com um agente, o executor:

- Envia as instruções do agente, ferramentas disponíveis e histórico acumulado.
- Interpreta respostas `TREE` e `READ` como solicitações de ferramenta.
- Interpreta respostas `ASK: ...` como perguntas ao usuário.
- Salva respostas e resultados no histórico.
- Considera a conversa concluída quando recebe uma resposta comum que não solicita ferramenta nem nova pergunta.

### Componentes principais

- `execute`: executa uma `Invocation` usando `BashService`, transmite a saída e rejeita processos que terminam com erro.

- `run`: inicia uma execução interativa usando `TerminalInput`.

- `resume`: abre um arquivo de histórico e continua a execução salva.

- `run_with`: executa sem entrada interativa, usando uma implementação que sempre retorna erro quando o workflow exige resposta do usuário. É útil para testes.

- `run_interactive_with`: valida o workflow, carrega os agentes, cria o snapshot e inicia o histórico.

- `validate_snapshot`: verifica novamente o workflow, o diretório de execução, a existência dos agentes, os adapters, a configuração `ask` e a proibição de streams JSON em conversas retomáveis.

- `all_steps`: percorre recursivamente as etapas, incluindo corpos de loops, para localizar agentes e validar referências.

- `validate_blocks`: valida se os blocos armazenados no histórico correspondem à estrutura atual do workflow e impede que existam registros posteriores a uma etapa pendente ou editada.

- `validate_step_blocks`: verifica a ordem válida dos blocos `Ask`, `Input`, `Output`, `Tree` e `Read`, além de identificar se uma conversa está completa.

- `continue_with`: valida o histórico e cria um `Engine` para continuar a execução, retornando os outputs globais.

- `Engine`: estrutura interna que contém o histórico, o executor de processos, a interface de entrada do usuário e o cursor da etapa atual.

- `Engine::run_steps`: processa etapas sequencialmente, cria escopos locais para loops, registra outputs e restaura corretamente o contexto entre iterações.

- `Engine::run_step`: executa uma etapa individual. Diferencia ferramentas, custom tools e conversas com agentes; salva entradas, resultados e solicitações de interação no histórico.

- `question`: identifica respostas de agentes no formato `ASK: pergunta`.

- Módulo de testes: cobre retomada após falhas, edição de respostas, solicitações de `TREE` e `READ`, escrita de arquivos, custom tools, loops aninhados, isolamento de escopos e validação antecipada de agentes.

### Dependências e integrações

O arquivo integra:

- `crate::workflow`: fornece `Workflow`, `Step`, renderização de templates, loops e destinos de output.
- `crate::agents::Agent`: carrega e fornece as configurações dos agentes.
- `crate::adapters`: resolve o adapter responsável por montar a invocação do harness.
- `crate::harness::RunRequest`: representa a solicitação enviada ao adapter.
- `crate::history`: persiste snapshots, blocos de conversa, resultados de ferramentas e labels das etapas.
- `crate::input`: abstrai entrada do usuário por `UserInput` ou terminal.
- `crate::services::{BashService, Invocation}`: executa processos externos.
- `crate::tools`: reconhece e executa ferramentas, além de implementar `READ`, `WRITE` e custom tools.
- `std::collections::BTreeMap`: armazena agentes, outputs e variáveis locais ordenadas.
- `std::path::Path`: representa diretórios e arquivos de execução.

O arquivo não chama diretamente um modelo. Ele delega essa responsabilidade ao adapter configurado e ao processo externo representado por `Invocation`.

### Observações

- Os resultados são persistidos atomicamente através de `History::save`, permitindo retomar depois de falhas.
- Chamadas que falham permanecem pendentes; resultados incompletos não são tratados como concluídos.
- Respostas vazias de agentes geram erro, enquanto resultados vazios de `READ` são aceitos.
- `LOOP` é tratado pelo workflow e não é disponibilizado como ferramenta para agentes.
- Outputs dentro de loops permanecem locais à iteração, enquanto outputs globais continuam legíveis.
- A confirmação de detalhes como o formato exato do histórico e a validação de paths depende das implementações dos módulos importados.
