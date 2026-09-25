### Resumo

Este arquivo implementa o adaptador do harness para o CLI do Codex. Ele converte uma requisição genérica (`RunRequest`) em uma `Invocation` específica para executar `codex exec`, mantendo a execução somente leitura e sem solicitar aprovações.

### Funcionamento

`CodexAdapter` armazena o nome ou caminho do executável do Codex. Por padrão, usa `"codex"`, mas pode receber outro executável por meio de `new`.

Ao criar uma invocação, o adaptador:

- inicia o comando com `codex exec`;
- ignora a exigência de estar dentro de um repositório Git;
- configura o sandbox como `read-only`;
- define `approval_policy="never"`;
- adiciona `--json` quando `event_stream` está habilitado;
- adiciona `--model` quando um modelo foi especificado;
- posiciona o prompt após `--`, evitando que seu conteúdo seja interpretado como opção;
- preserva o diretório de trabalho informado.

O método retorna os dados estruturados da execução sem iniciar o processo.

### Componentes principais

- `CodexAdapter`: struct pública que representa o adaptador do Codex.
- `Default for CodexAdapter`: cria um adaptador usando o executável padrão `codex`.
- `CodexAdapter::new`: permite configurar um executável personalizado.
- `HarnessAdapter for CodexAdapter`:
  - `id`: identifica o adaptador como `"codex"`.
  - `invocation`: transforma `RunRequest` em `Invocation`.
- Módulo de testes:
  - `translates_generic_options_to_codex_exec`: verifica se prompt, modelo, diretório de trabalho, modo JSON e opções de segurança são traduzidos corretamente.

### Dependências e integrações

O arquivo importa, do módulo interno `crate::harness`:

- `HarnessAdapter`: trait que define a interface dos adaptadores;
- `HarnessError`: tipo de erro da criação da invocação;
- `Invocation`: representação do programa e seus argumentos;
- `RunRequest`: requisição genérica de execução.

Também utiliza `std::path::PathBuf` nos testes para definir o diretório de trabalho.

### Observações

A função `invocation` retorna `Result`, mas a implementação atual sempre retorna `Ok`; não há validações ou falhas explícitas neste arquivo. A execução real do processo e o tratamento posterior da saída são responsabilidade de outras partes do projeto.
