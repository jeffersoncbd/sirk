### Resumo

Este arquivo implementa o componente `Tree`, responsável por listar arquivos de uma árvore de trabalho Git. A listagem inclui arquivos rastreados e arquivos não rastreados que não sejam ignorados, respeitando regras do Git e regras adicionais definidas em `.treeignore`.

### Funcionamento

`Tree::list` recebe um diretório e:

1. Resolve o caminho para uma forma canônica e verifica se ele é um diretório.
2. Executa `git ls-files` para obter arquivos rastreados e arquivos não ignorados, usando saída delimitada por NUL (`-z`).
3. Executa uma segunda consulta ao Git para identificar arquivos excluídos por `.treeignore`.
4. Remove da listagem os caminhos encontrados nessa segunda consulta.
5. Converte os caminhos em `PathBuf`, preservando caminhos não UTF-8 em sistemas Unix.
6. Usa `symlink_metadata` para incluir arquivos regulares e links simbólicos, sem seguir os links.
7. Ignora arquivos removidos do working tree, diretórios e gitlinks.
8. Ordena e remove duplicatas antes de retornar o resultado.

O método retorna `Result<Tree, String>`. Erros incluem diretório inválido, falha ao executar o Git, execução fora de uma árvore Git, falha na avaliação de `.treeignore` e problemas ao inspecionar arquivos.

### Componentes principais

- `Tree`: struct pública com:
  - `root: PathBuf`: diretório raiz canonicalizado.
  - `files: Vec<PathBuf>`: caminhos relativos, ordenados e únicos.

- `Tree::list`: função pública que realiza a descoberta dos arquivos usando Git.

- `path_from_bytes`: funções condicionadas à plataforma que convertem os caminhos retornados pelo Git:
  - Em Unix, aceita bytes arbitrários por meio de `OsStrExt`.
  - Em outras plataformas, exige UTF-8 válido.

- `Project`: estrutura privada usada apenas nos testes para criar repositórios Git temporários, escrever arquivos e limpar os diretórios ao final.

- Testes: verificam regras de `.gitignore`, exclusões padrão do Git, padrões aninhados, negações, `.treeignore`, arquivos rastreados ignorados, arquivos removidos, links simbólicos, caminhos não UTF-8 e diretórios inválidos.

### Dependências e integrações

- `crate::services::{BashService, Invocation}`: executa o processo externo `git` com programa, argumentos e diretório de trabalho separados.
- `std::fs` e `std::io`: acessam metadados dos arquivos e descartam a saída de erro dos processos.
- `BTreeSet`: armazena os caminhos excluídos por `.treeignore`.
- Git: fornece a semântica de arquivos rastreados, ignorados, subdiretórios e padrões de exclusão.
- O resultado é consumido pelo restante do projeto através da struct pública `Tree`.

### Observações

A implementação não altera o índice Git. Arquivos já rastreados continuam aparecendo mesmo quando `.gitignore` os ignora, mas arquivos removidos fisicamente não são retornados. Links simbólicos são listados como entradas e não são percorridos. A interpretação completa dos padrões depende do comportamento do Git e dos arquivos `.gitignore`/`.treeignore` existentes no diretório.
