### Resumo

O arquivo `src/agents.rs` define o modelo de dados e o carregamento de agentes descritos em arquivos Markdown com front matter YAML. Ele valida identificadores, interpreta metadados, normaliza modelos, configura prefixos de chamada, controla permissões de ferramentas e rejeita definições inválidas.

### Funcionamento

Um agente é lido a partir de um arquivo `<id>.md`. O identificador precisa conter apenas letras, dígitos, `_` ou `-`, evitando caminhos inválidos ou traversal.

O conteúdo deve começar com `---`, delimitando o front matter YAML, que também precisa ter um marcador de fechamento `---`. Esse bloco é desserializado com `serde_yaml`, aceitando apenas campos conhecidos.

Os campos `EDIT_TOOL`, `DELETE_TOOL`, `DELETE_WITHOUT_CONFIRM` e `TREE_TOOL` são opcionais e só aceitam o valor textual `allow`. `DELETE_WITHOUT_CONFIRM` exige que `DELETE_TOOL` também esteja habilitado. A ausência de `TREE_TOOL` mantém o valor `true` para compatibilidade com snapshots antigos. O campo `json: true` é explicitamente rejeitado.

O campo `model` é normalizado removendo espaços externos e convertendo o texto para minúsculas. `call_prefix` aceita uma string única ou uma lista de argumentos, mas rejeita argumentos vazios.

Após o front matter, o restante do arquivo é tratado como instruções Markdown. Essas instruções são normalizadas removendo espaços externos e não podem estar vazias.

Erros de leitura, parsing, metadados inválidos, identificadores incorretos, permissões incompatíveis e instruções ausentes são convertidos em mensagens `String` usando `Result` e o operador `?`. Não há uso de `unsafe`, concorrência ou comunicação externa além da leitura local de arquivos.

### Componentes principais

- `Agent`: struct pública serializável e desserializável que representa um agente, incluindo:
  - `id`: identificador;
  - `adapter`: adaptador associado;
  - `instructions`: instruções Markdown;
  - `model`: modelo opcional normalizado;
  - `call_prefix`: argumentos opcionais usados como prefixo de chamada;
  - `tree_tool`: indica se a ferramenta de árvore foi permitida;
  - `json`: configuração de saída JSON;
  - `ask`: configuração opcional adicional;
  - `edit_tool`: indica se a ferramenta externa de edição foi permitida;
  - `delete_tool`: indica se operações de exclusão foram permitidas;
  - `delete_without_confirm`: indica se exclusões sem confirmação foram permitidas.

- `Metadata`: struct privada usada apenas para interpretar o front matter YAML. Usa `deny_unknown_fields` para rejeitar propriedades não declaradas.

- `deserialize_model`: função privada de desserialização que transforma o modelo opcional para sua forma normalizada.

- `deserialize_call_prefix`: função privada que aceita um argumento único ou uma lista de argumentos e rejeita elementos vazios.

- `deserialize_edit_tool`: função privada que aceita somente o valor `allow` para `EDIT_TOOL` e o converte em `bool`.

- `deserialize_delete_permission`: função privada que aplica a mesma regra de valor `allow` aos campos de permissão de exclusão.

- `deserialize_tree_permission`: função privada que aceita somente o valor `allow` para `TREE_TOOL`.

- `legacy_tree_tool_default`: fornece o valor padrão `true` para snapshots de agentes criados antes da existência de `TREE_TOOL`.

- `valid_id`: função pública que valida os identificadores permitidos.

- `Agent::load`: função pública que valida o identificador, lê o arquivo correspondente e delega o parsing para `Agent::parse`.

- `Agent::parse`: função `pub(crate)` que interpreta o conteúdo textual, separa o YAML das instruções Markdown, valida os campos e constrói um `Agent`.

- Módulo `tests`: contém testes unitários para CRLF, normalização de modelos, prefixos de chamada, compatibilidade de `TREE_TOOL`, serialização de snapshots, habilitação das permissões de edição e exclusão e rejeição de definições inválidas.

### integrações
- A função `valid_id`.
- O método `Agent::load`.

Também disponibiliza `Agent::parse` dentro da própria crate por meio de `pub(crate)`. Internamente, depende de `std::fs` e `std::path::Path` para acessar arquivos e de `serde`/`serde_yaml` para serialização, desserialização e validação do front matter.Também disponibiliza `Agent::parse` dentro da própria crate por meio de `pub(crate)`. Internamente, depende de `std::fs` e `std::path::Path` para acessar arquivos e de `serde`/`serde_yaml` para serialização, desserialização e validação do front matter.