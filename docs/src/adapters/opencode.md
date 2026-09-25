### Resumo

Este arquivo implementa o adaptador do provedor OpenCode. Sua responsabilidade é converter uma requisição genérica do harness em uma invocação adequada da CLI `opencode`.

### Funcionamento

`OpenCodeAdapter` armazena o nome do executável a ser chamado, usando `"opencode"` por padrão. Ao receber um `RunRequest`, monta o comando:

- inicia com `opencode run`;
- define o título fixo `new-harness`;
- adiciona `--format json` quando `event_stream` está habilitado;
- adiciona `--model` quando um modelo foi informado;
- usa `--` antes do prompt, separando opções do argumento textual;
- preserva o diretório de trabalho da requisição.

O adaptador deliberadamente não inclui a opção `--auto`, pois ela habilitaria aprovações automáticas de permissões.

### Componentes principais

- `OpenCodeAdapter`: struct pública que representa o adaptador OpenCode.
- `Default::default`: cria o adaptador usando o executável `opencode`.
- `OpenCodeAdapter::new`: permite configurar outro nome ou caminho de executável.
- `HarnessAdapter for OpenCodeAdapter`:
  - `id`: retorna o identificador `"opencode"`.
  - `invocation`: gera uma `Invocation` com programa, argumentos e diretório de trabalho.
- Testes internos:
  - verificam a tradução das opções genéricas para os argumentos da CLI;
  - garantem que `--auto` não seja incluído.

### Dependências e integrações

O arquivo utiliza os tipos internos de `crate::harness`:

- `HarnessAdapter`: trait que padroniza adaptadores de provedores;
- `HarnessError`: tipo de erro da construção da invocação;
- `Invocation`: representação do processo a ser executado;
- `RunRequest`: requisição genérica contendo prompt, modelo, diretório e formato de saída.

Também usa `std::path::PathBuf` apenas nos testes.

### Observações

A função `invocation` retorna `Result`, mas nesta implementação sempre produz `Ok`, pois não há validações ou operações externas durante a montagem dos argumentos. A execução real do processo ocorre em outra parte do projeto.
