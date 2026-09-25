### Resumo

O arquivo `src/tools/tree.rs` implementa a enumeração recursiva de arquivos de uma árvore de trabalho Git. Ele combina arquivos rastreados e arquivos não rastreados que não estejam ignorados, aplica regras adicionais de `.treeignore` e retorna caminhos relativos, ordenados e únicos.

### Funcionamento

A função pública `Tree::list`:

1. Converte o diretório informado para um caminho canônico e verifica se ele é um diretório.
2. Executa `git ls-files` por meio de `BashService` e `Invocation`, usando saída delimitada por NUL (`-z`) para preservar nomes incomuns, incluindo espaços e quebras de linha.
3. Inclui arquivos rastreados e arquivos não rastreados não ignorados pelas regras padrão do Git.
4. Executa uma segunda consulta ao Git para identificar entradas excluídas por `.treeignore`.
5. Remove essas entradas sem permitir que uma negação em `.treeignore` reintroduza arquivos bloqueados por `.gitignore`.
6. Verifica cada caminho com `symlink_metadata`, incluindo arquivos regulares e links simbólicos, mas sem seguir os links.
7. Ignora diretórios, gitlinks, arquivos rastreados que foram removidos e entradas inexistentes.
8. Ordena e elimina duplicatas antes de retornar o resultado.

Erros de resolução do diretório, execução do Git, avaliação de `.treeignore`, conversão de caminhos ou inspeção do sistema de arquivos são convertidos em `String` descritivas. A função não altera o índice Git.

Os testes criam repositórios temporários e verificam padrões aninhados, negações, exclusões padrão, `.treeignore`, arquivos rastreados ignorados, arquivos removidos, caminhos não UTF-8, links simbólicos e diretórios inválidos.

### Componentes principais

- `Tree`: struct pública que contém:
  - `root: PathBuf`: raiz canônica usada na enumeração.
  - `files: Vec<PathBuf>`: arquivos encontrados, relativos à raiz, ordenados e sem duplicatas.

- `Tree::list`: método público que realiza toda a enumeração e retorna `Result<Tree, String>`.

- `path_from_bytes`: função interna específica da plataforma que converte os caminhos retornados pelo Git:
  - Em Unix, preserva bytes não UTF-8 usando `OsStrExt`.
  - Em outras plataformas, exige UTF-8 válido e retorna erro caso contrário.

- `BashService` e `Invocation`: abstrações internas usadas para executar o Git sem montar comandos shell por interpolação de strings.

- `BTreeSet`: armazena as entradas excluídas por `.treeignore`, permitindo comparação eficiente e ordenada.

- Módulo de testes `#[cfg(test)]`: fornece o fixture `Project` para criar repositórios Git temporários e validar o comportamento de `Tree::list`.

### integrações

A struct pública `Tree` e o método público `Tree::list` são a interface exposta por este arquivo. O módulo depende de `crate::services::{BashService, Invocation}` para executar comandos Git e de APIs da biblioteca padrão para manipular caminhos, arquivos, links simbólicos e erros de entrada/saída.
