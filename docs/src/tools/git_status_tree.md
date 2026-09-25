### Resumo

O arquivo implementa a descoberta de arquivos existentes que aparecem como alterados no `git status` dentro de uma árvore Git. O resultado contém a raiz canonicalizada e uma lista ordenada, única e relativa de caminhos de arquivos elegíveis.

### Funcionamento

`GitStatusTree::list`:

1. Resolve o diretório informado e valida se ele é uma pasta.
2. Confirma que o diretório pertence a uma working tree Git usando `git rev-parse`.
3. Executa `git status --porcelain=v1 -z --untracked-files=all`.
4. Interpreta a saída binária delimitada por NUL, incluindo renomeações e cópias.
5. Aplica regras adicionais de exclusão definidas em arquivos `.treeignore`.
6. Remove:
   - arquivos deletados;
   - arquivos ignorados;
   - diretórios;
   - conteúdos de submódulos;
   - caminhos excluídos por `.treeignore`.
7. Verifica no sistema de arquivos se cada caminho ainda é um arquivo ou symlink.
8. Ordena e remove duplicatas antes de retornar o resultado.

Erros são propagados como `Result<Self, String>`, com mensagens específicas para diretórios inválidos, falhas na execução do Git, saída malformada e problemas ao inspecionar arquivos. A execução externa é feita por `BashService` e `Invocation`, com captura binária da saída padrão.

### Componentes principais

- `GitStatusTree`: struct pública com:
  - `root`: caminho canonicalizado da raiz analisada;
  - `files`: caminhos relativos encontrados.
- `GitStatusTree::list`: função pública que realiza toda a coleta e filtragem dos caminhos.
- `GitOutput`: struct privada que agrupa o `ExitStatus` e a saída binária do Git.
- `tree_ignored`: função privada que executa `git ls-files` para obter os caminhos abrangidos por `.treeignore`.
- `git`: função privada que encapsula a execução de comandos Git por meio de `BashService`.
- `path_from_bytes`: converte caminhos retornados pelo Git em `PathBuf`. Em Unix preserva bytes arbitrários; em outras plataformas exige UTF-8 válido.
- Módulo `tests`: cria repositórios Git temporários e verifica filtragem, caminhos relativos, renomeações, arquivos deletados, regras `.treeignore` e erros de entrada.

O arquivo usa `BTreeSet` para representar exclusões ordenadas, `Path`/`PathBuf` para caminhos, `fs::symlink_metadata` para inspeção sem seguir symlinks e `ExitStatus` para validar comandos externos.

### integrações

A API pública expõe `GitStatusTree` e seu método `list`. O arquivo depende internamente de `crate::services::{BashService, Invocation}` para executar o Git e do próprio Git instalado no ambiente. Também interage com o sistema de arquivos para canonicalizar diretórios, ler metadados e interpretar arquivos `.treeignore`.

A implementação depende de convenções específicas da saída `git status --porcelain=v1 -z` e de comandos Git disponíveis; o restante da lógica da aplicação não está presente no arquivo.
