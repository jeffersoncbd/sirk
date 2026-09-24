### Resumo

Este arquivo define o modelo de configuração de um agente Rust e fornece mecanismos para carregar e validar sua definição a partir de arquivos Markdown com front matter YAML.

### Funcionamento

O arquivo espera definições no formato:

1. Um bloco YAML iniciado e encerrado por `---`.
2. Um corpo Markdown contendo as instruções do agente.

`Agent::load` valida o identificador, monta o caminho `<id>.md`, lê o arquivo e delega o parsing para `Agent::parse`.

Durante o parsing, o código:

- Valida a presença correta do front matter.
- Desserializa os metadados YAML.
- Rejeita campos desconhecidos.
- Exige que `adapter` não esteja vazio.
- Exige que as instruções Markdown contenham algum texto.
- Normaliza o nome do modelo removendo espaços e convertendo-o para minúsculas.

Erros são propagados como `Result<T, String>`, com mensagens contextualizadas sobre o arquivo ou a etapa inválida.

### Componentes principais

- `Agent`: struct pública que representa um agente carregado, contendo:
  - `id`: identificador do agente.
  - `adapter`: adaptador de execução.
  - `instructions`: instruções Markdown.
  - `model`: modelo opcional normalizado.
  - `json`: configuração booleana.
  - `ask`: configuração opcional.

- `Metadata`: struct privada usada exclusivamente para desserializar o front matter YAML. O atributo `deny_unknown_fields` impede configurações não reconhecidas.

- `deserialize_model`: função auxiliar que desserializa e normaliza nomes de modelos.

- `valid_id`: função pública que aceita apenas identificadores não vazios formados por caracteres ASCII alfanuméricos, `_` ou `-`.

- `Agent::load`: função pública que lê um arquivo de agente do diretório informado.

- `Agent::parse`: função `pub(crate)` que interpreta o conteúdo textual de uma definição de agente.

- Módulo `tests`: contém testes para parsing com CRLF, normalização de modelos e rejeição de definições ou caminhos inválidos.

### Dependências e integrações

- `std::fs` e `std::path::Path` são usados para ler arquivos e construir caminhos.
- `serde` fornece `Serialize`, `Deserialize` e a desserialização customizada.
- `serde_yaml` interpreta o front matter YAML.
- O arquivo depende do restante do projeto para decidir como `adapter`, `model`, `json`, `ask` e `instructions` serão utilizados posteriormente.

### Observações

A única operação externa realizada neste arquivo é a leitura do arquivo de configuração; ele não executa o agente nem grava dados. O código apresentado não mostra as regras aplicadas pelos adaptadores ou pelo mecanismo de execução.
