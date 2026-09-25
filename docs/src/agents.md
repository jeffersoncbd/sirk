### Resumo

O arquivo `src/agents.rs` define o modelo de dados e o carregamento de agentes descritos em arquivos Markdown com front matter YAML. Ele valida identificadores, interpreta metadados, normaliza modelos e rejeita definições inválidas.

### Funcionamento

Um agente é lido a partir de um arquivo `<id>.md`. O identificador precisa conter apenas letras, dígitos, `_` ou `-`, evitando caminhos inválidos ou traversal.

O conteúdo deve começar e terminar com `---`, delimitando o front matter YAML. Esse bloco é desserializado com `serde_yaml`, aceitando apenas campos conhecidos. O campo `model` é opcional e tem espaços removidos e letras convertidas para minúsculas.

Após o front matter, o restante do arquivo é tratado como instruções Markdown. Essas instruções são normalizadas removendo espaços externos e não podem estar vazias. O campo `json: true` é explicitamente rejeitado.

Erros de leitura, parsing, metadados inválidos, identificadores incorretos e instruções ausentes são convertidos em mensagens `String` usando `Result` e o operador `?`. Não há uso de `unsafe`, concorrência ou comunicação externa além da leitura local de arquivos.

### Componentes principais

- `Agent`: struct pública serializável e desserializável que representa um agente, incluindo:
  - `id`: identificador;
  - `adapter`: adaptador associado;
  - `instructions`: instruções Markdown;
  - `model`: modelo opcional normalizado;
  - `json`: configuração de saída JSON;
  - `ask`: configuração opcional adicional.

- `Metadata`: struct privada usada apenas para interpretar o front matter YAML. Usa `deny_unknown_fields` para rejeitar propriedades não declaradas.

- `deserialize_model`: função privada de desserialização que transforma o modelo opcional para sua forma normalizada.

- `valid_id`: função pública que valida os identificadores permitidos.

- `Agent::load`: função pública que valida o identificador, lê o arquivo correspondente e delega o parsing para `Agent::parse`.

- `Agent::parse`: função `pub(crate)` que interpreta o conteúdo textual, separa o YAML das instruções Markdown, valida os campos e constrói um `Agent`.

- Módulo `tests`: contém testes unitários para CRLF, normalização de modelos, serialização de snapshots e rejeição de definições inválidas.

### integrações

O arquivo expõe publicamente:

- A struct `Agent`, com suporte a `Serialize` e `Deserialize`.
- A função `valid_id`.
- O método `Agent::load`.

Também disponibiliza `Agent::parse` dentro da própria crate por meio de `pub(crate)`. Internamente, depende de `std::fs` e `std::path::Path` para acessar arquivos e de `serde`/`serde_yaml` para serialização, desserialização e validação do front matter.
