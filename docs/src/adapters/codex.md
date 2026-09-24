### Resumo

Este arquivo implementa o adaptador do harness Codex. Ele converte uma requisição genérica de execução (`RunRequest`) em um comando específico para o executável `codex`, preservando diretório de trabalho, modelo, prompt e modo de saída.

### Funcionamento

O `CodexAdapter` armazena o nome do executável a ser chamado, usando `"codex"` por padrão. Ao receber um `RunRequest`, constrói uma `Invocation` com:

- subcomando `exec`;
- verificação de repositório Git desativada;
- sandbox em modo `read-only`;
- política de aprovação configurada como `"never"`;
- opção `--json` quando solicitado um fluxo de eventos;
- opção `--model` quando um modelo foi informado;
- prompt delimitado por `--`.

A função `invocation` retorna a invocação construída dentro de um `Result`, embora a implementação atual não produza erros.

### Componentes principais

- `CodexAdapter`: struct pública que representa o adaptador do Codex.
- `Default for CodexAdapter`: define `"codex"` como executável padrão.
- `CodexAdapter::new`: permite configurar outro nome ou caminho de executável.
- `HarnessAdapter for CodexAdapter`:
  - `id`: identifica o adaptador como `"codex"`.
  - `invocation`: traduz `RunRequest` para `Invocation`.
- Módulo de testes:
  - verifica se as opções genéricas são convertidas nos argumentos esperados para `codex exec`.

### Dependências e integrações

O arquivo depende de tipos internos do módulo `crate::harness`:

- `HarnessAdapter`: trait que padroniza adaptadores de harness;
- `HarnessError`: tipo de erro da criação da invocação;
- `Invocation`: representação do programa, argumentos e diretório de execução;
- `RunRequest`: requisição genérica recebida pelo adaptador.

Também usa `PathBuf` exclusivamente no teste.

### Observações

O código não executa diretamente o processo; apenas monta uma descrição da invocação para outra camada realizar a execução. A configuração impõe execução somente para leitura e sem aprovações automáticas. O comportamento exato de `Invocation` e `RunRequest` depende de definições externas que não estão presentes no trecho.
