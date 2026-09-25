### Resumo

Este arquivo define o modelo `Agent` e o mecanismo para carregar e interpretar agentes descritos em arquivos Markdown com front matter YAML. Ele valida o identificador do agente, lê o arquivo, extrai seus metadados e armazena as instruções Markdown.

### Funcionamento

O arquivo espera definições no formato:

```text
---
adapter: codex
model: algum-modelo
json: true
ask: pergunta opcional
---
Instruções do agente em Markdown.
```

O fluxo principal é:

1. `Agent::load` valida o identificador recebido.
2. Monta o caminho `<directory>/<id>.md`.
3. Lê o arquivo como UTF-8.
4. Passa o conteúdo para `Agent::parse`.
5. `parse` verifica o delimitador inicial e final `---`.
6. Converte o front matter YAML em `Metadata`.
7. Valida se `adapter` e as instruções Markdown não estão vazios.
8. Cria e retorna um `Agent`.

O campo `model` é normalizado: espaços nas extremidades são removidos e o valor é convertido para minúsculas, tanto ao ler arquivos Markdown quanto ao desserializar snapshots salvos.

### Componentes principais

- `Agent`: struct pública que representa um agente carregado. Contém:
  - `id`: identificador do agente.
  - `adapter`: adaptador responsável por executá-lo.
  - `instructions`: conteúdo Markdown.
  - `model`: modelo opcional normalizado.
  - `json`: indica se há saída em JSON.
  - `ask`: pergunta opcional associada ao agente.

- `Metadata`: struct privada usada somente para desserializar o front matter YAML. Usa `deny_unknown_fields`, rejeitando propriedades não reconhecidas.

- `deserialize_model`: função auxiliar que desserializa e normaliza o campo `model`.

- `valid_id`: função pública que aceita apenas identificadores não vazios contendo letras ASCII, dígitos, `_` ou `-`.

- `Agent::load`: função pública que valida o ID, lê o arquivo correspondente e transforma erros de leitura ou parsing em mensagens contextualizadas.

- `Agent::parse`: função interna ao crate (`pub(crate)`) que interpreta diretamente o conteúdo de um arquivo, separando metadados YAML das instruções Markdown.

- Módulo `tests`: verifica suporte a finais de linha CRLF, normalização de modelos, rejeição de metadados desconhecidos, conteúdo incompleto e IDs potencialmente usados para atravessar diretórios.

### Dependências e integrações

- `std::fs` e `std::path::Path`: leitura dos arquivos e construção segura dos caminhos.
- `serde`: serialização e desserialização de `Agent` e `Metadata`.
- `serde_yaml`: interpretação do front matter YAML.
- O arquivo integra-se com o restante do projeto por meio do campo `adapter`, cujo comportamento não é definido aqui.

### Observações

- O tratamento de erros usa `Result`, mensagens próprias e o operador `?`; não há `panic!` no código de produção.
- IDs inválidos são rejeitados antes da construção do caminho, evitando nomes com separadores de diretório ou extensões arbitrárias.
- O parser preserva a estrutura Markdown, removendo apenas espaços e quebras de linha nas extremidades.
- O arquivo não mostra como os agentes são executados; ele apenas os representa e carrega suas definições.
