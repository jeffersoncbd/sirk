### Resumo

`src/adapters/codex.rs` implementa o adaptador responsável por transformar uma requisição genérica do harness em uma invocação do comando `codex exec`.

### Funcionamento

O `CodexAdapter` armazena o nome do executável e, por padrão, usa `codex`. Ao receber um `RunRequest`, monta uma lista de argumentos que:

- executa o subcomando `exec`;
- ignora a exigência de estar em um repositório Git;
- força o sandbox em modo `read-only`;
- desativa aprovações com `approval_policy="never"`;
- adiciona `--json` quando solicitado um fluxo de eventos;
- inclui o modelo especificado, se houver;
- encerra as opções com `--` e acrescenta o prompt.

O resultado é um `Invocation` contendo programa, argumentos, diretório de trabalho e ambiente. O método retorna `Result`, embora a implementação atual sempre produza `Ok`.

O módulo também contém um teste que verifica a tradução de uma requisição completa para os argumentos esperados do Codex.

### Componentes principais

- `CodexAdapter`: struct pública que guarda o caminho ou nome do executável.
- `Default for CodexAdapter`: cria o adaptador usando o executável `codex`.
- `CodexAdapter::new`: permite configurar outro executável.
- `HarnessAdapter for CodexAdapter`:
  - `id`: retorna o identificador estático `"codex"`.
  - `invocation`: constrói a invocação do processo Codex.
- `#[cfg(test)] mod tests`: valida a montagem dos argumentos.

O arquivo importa `HarnessAdapter`, `HarnessError`, `Invocation` e `RunRequest` do módulo interno `crate::harness`.

### integrações

A struct `CodexAdapter` e seu construtor `new` são públicos. A implementação pública de `HarnessAdapter` expõe a identificação do adaptador e a conversão de `RunRequest` em `Invocation`.

O arquivo depende dos tipos definidos em `crate::harness` e do executável externo configurado, mas o conteúdo fornecido não mostra como o processo é posteriormente executado.
