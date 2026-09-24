### Resumo

Este arquivo implementa o armazenamento persistente e editável do histórico de execução da aplicação. Ele salva a configuração usada (`Snapshot`), os blocos de conversa e os rótulos das etapas em um transcript v2, permitindo reabrir e continuar execuções anteriores.

### Funcionamento

`History::create` cria o diretório `history`, gera um arquivo de log único baseado no horário e no ID do processo, adquire um arquivo de bloqueio e salva o histórico inicial.

`History::open` abre um transcript existente, canonicaliza seu caminho, adquire o bloqueio correspondente, lê o conteúdo e reconstrói o snapshot, as etapas e seus blocos.

O parser:

- valida o cabeçalho da versão v2;
- separa a configuração YAML do corpo do histórico;
- identifica etapas, marcadores e conteúdos;
- desfaz o escape de linhas que poderiam ser confundidas com marcadores;
- rejeita linhas inesperadas ou formatos não suportados.

`History::save` serializa o snapshot em YAML e grava os blocos novamente. A escrita ocorre em um arquivo temporário, seguido de `sync_all` e renomeação para o arquivo final, garantindo uma substituição atômica do transcript.

### Componentes principais

- `Snapshot`: configuração persistida da execução, contendo diretório, `Workflow` e agentes.
- `Block`: enum que representa os tipos de conteúdo do histórico:
  - `Ask`: solicitação ao agente;
  - `Input`: entrada do usuário;
  - `Output`: resposta do agente;
  - `Tree`: resultado da ferramenta TREE;
  - `Read`: resultado da ferramenta READ.
- `Block::text`: retorna o conteúdo textual do bloco.
- `Block::marker`: retorna o marcador usado no arquivo.
- `History`: estado completo de um transcript aberto, incluindo caminho, snapshot, etapas, rótulos e arquivo de lock.
- `History::create`: cria um novo histórico.
- `History::open`: carrega um histórico existente.
- `History::parse`: converte o texto persistido em estruturas Rust.
- `History::save`: serializa e persiste o histórico.
- `reserved`: identifica linhas que precisam ser escapadas para não serem interpretadas como estrutura do transcript.
- `Drop` para `History`: libera explicitamente o bloqueio ao destruir o histórico.
- Teste `round_trip_preserves_markers_and_whitespace`: verifica que marcadores, escapes e espaços em branco sobrevivem ao ciclo salvar–ler.

### Dependências e integrações

- `crate::agents::Agent`: representa as definições de agentes incluídas no snapshot.
- `crate::workflow::Workflow`: representa o workflow persistido.
- `serde` e `serde_yaml`: serializam e desserializam a configuração.
- `std::fs`, `File` e `OpenOptions`: criam diretórios, leem e escrevem arquivos.
- `SystemTime` e `UNIX_EPOCH`: geram nomes únicos para os históricos.
- O arquivo interage com o sistema de arquivos por meio de arquivos `.log`, `.log.lock` e `.log.tmp`.

### Observações

O histórico só aceita o formato v2 identificado por `TITLE`. O arquivo de lock impede que o mesmo transcript seja usado simultaneamente por diferentes processos. A configuração é validada com `#[serde(deny_unknown_fields)]`, portanto campos YAML desconhecidos são rejeitados.
