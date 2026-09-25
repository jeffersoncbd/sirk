### Resumo

Este arquivo implementa o adaptador do provedor OpenCode. Ele converte uma requisição genérica do harness em uma invocação específica da CLI `opencode run`.

### Funcionamento

`OpenCodeAdapter` armazena o nome do executável a ser chamado, usando `"opencode"` por padrão. Ao receber um `RunRequest`, o método `invocation` monta os argumentos da linha de comando:

- inicia com `run`;
- define o título fixo `new-harness`;
- adiciona `--format json` quando `event_stream` está habilitado;
- adiciona `--model` quando um modelo foi especificado;
- usa `--` para separar opções da CLI do prompt;
- adiciona o prompt fornecido;
- preserva o diretório de trabalho da requisição.

O código deliberadamente não inclui a opção `--auto`, pois ela habilitaria aprovações automáticas de permissões não negadas explicitamente.

### Componentes principais

- `OpenCodeAdapter`: struct pública que representa o adaptador OpenCode.
- `Default for OpenCodeAdapter`: cria o adaptador usando o executável padrão `opencode`.
- `OpenCodeAdapter::new`: permite configurar outro nome ou caminho de executável.
- `HarnessAdapter for OpenCodeAdapter`:
  - `id`: retorna o identificador `"opencode"`.
  - `invocation`: traduz `RunRequest` em `Invocation` ou retorna `HarnessError`.
- Testes internos:
  - verificam a montagem dos argumentos com modelo e saída JSON;
  - garantem que `--auto` não seja incluído.

### Dependências e integrações

O arquivo usa tipos internos de `crate::harness`:

- `HarnessAdapter`: trait que padroniza adaptadores de provedores;
- `HarnessError`: tipo de erro da abstração;
- `Invocation`: representa programa, argumentos e diretório de execução;
- `RunRequest`: contém prompt, modelo, diretório de trabalho e configuração do fluxo de eventos.

A execução efetiva do processo não ocorre neste arquivo; ele apenas produz a descrição da invocação para outro componente do harness executar.

### Observações

O tratamento de erro é representado pelo tipo `Result`, embora a implementação atual sempre retorne `Ok`. O prompt é passado como argumento separado, sem interpolação em shell.
