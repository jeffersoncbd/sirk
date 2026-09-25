### Resumo

Este arquivo implementa o armazenamento e a recuperação de históricos de execução da aplicação. Ele mantém um `Snapshot` da configuração, registra as etapas da execução em blocos de conversa e salva tudo em um transcript persistente no formato v2.

### Funcionamento

`History::create` cria o diretório `history`, gera um nome único para o arquivo, adquire um lock exclusivo e salva o histórico inicial.

`History::open` abre um transcript existente, canonicaliza o caminho, adquire o lock, lê o conteúdo e reconstrói o snapshot, as etapas e seus rótulos.

O formato do arquivo contém:

- Um cabeçalho fixo identificando o formato v2.
- Metadados serializados em YAML.
- Etapas separadas por uma linha delimitadora.
- Blocos identificados por marcadores como `==> INPUT`, `==> ASK` e `<== OUTPUT`.

Durante a gravação, linhas que poderiam ser confundidas com marcadores ou cabeçalhos são prefixadas com `\`. Durante a leitura, esse escape é removido. A gravação usa um arquivo temporário, `sync_all` e renomeação para substituir o arquivo final.

### Componentes principais

- `Snapshot`: estado serializável da execução, contendo o diretório de trabalho, o `Workflow` e os `Agent`s configurados.
- `Block`: enum que representa tipos de conteúdo do histórico:
  - `Ask`: solicitação enviada ao agente.
  - `Input`: entrada fornecida pelo usuário ou sistema.
  - `Output`: saída produzida.
  - `Tree`: resultado de uma consulta de árvore de arquivos.
  - `Read`: conteúdo de um arquivo lido.
- `Block::text`: retorna o conteúdo textual do bloco.
- `Block::marker`: retorna o marcador usado no formato persistido.
- `History`: estado completo do histórico aberto, incluindo:
  - caminho do transcript;
  - snapshot;
  - etapas e blocos;
  - rótulos das etapas;
  - arquivo usado para manter o lock.
- `History::create`: cria um novo histórico.
- `History::open`: carrega um histórico existente.
- `History::parse`: interpreta o formato textual v2.
- `History::save`: serializa e grava o histórico.
- `History::lock`: obtém um lock exclusivo associado ao transcript.
- `reserved`: identifica linhas que precisam ser escapadas.
- `Drop for History`: libera explicitamente o lock quando o histórico é destruído.
- `ParsedHistory`: alias para o resultado interno do parser.

### Dependências e integrações

O arquivo usa:

- `crate::agents::Agent` e `crate::workflow::Workflow`, que representam a configuração de agentes e do fluxo da aplicação.
- `serde` e `serde_yaml` indiretamente via `Serialize`, `Deserialize` e desserialização YAML.
- APIs de `std` para arquivos, diretórios, locks, caminhos, timestamps e escrita persistente.
- `std::process::id()` para incluir o identificador do processo no nome do arquivo.

### Observações

O parser aceita apenas históricos iniciados pelo cabeçalho v2 e rejeita formatos anteriores ou estruturas inválidas com `Result<String>`.

Erros de leitura, escrita, serialização, parsing ou aquisição do lock são convertidos em mensagens textuais. O arquivo de lock impede que o mesmo histórico seja aberto simultaneamente por outra execução.

O código de teste verifica a preservação de marcadores, escapes e espaços em branco durante o ciclo de gravação e leitura.
