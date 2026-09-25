### Resumo

Este arquivo implementa `GitStatusTree`, responsável por listar arquivos existentes que aparecem no status de um repositório Git a partir de um diretório informado. Ele retorna caminhos relativos, ordenados e sem duplicatas, excluindo arquivos apagados, ignorados, diretórios e conteúdos de submódulos.

### Funcionamento

`GitStatusTree::list`:

1. Converte o diretório recebido para um caminho canônico e verifica se ele é um diretório.
2. Executa `git rev-parse --show-prefix` para confirmar que o diretório está dentro de uma árvore de trabalho Git e obter o prefixo relativo ao repositório.
3. Executa `git status --porcelain=v1 -z --untracked-files=all` para obter arquivos modificados, adicionados, renomeados, copiados, não mesclados e não rastreados.
4. Avalia regras adicionais de exclusão definidas em `.treeignore`.
5. Processa a saída delimitada por NUL, preservando corretamente nomes de arquivos que possam conter espaços ou caracteres especiais.
6. Remove caminhos fora do diretório solicitado.
7. Verifica no sistema de arquivos se cada caminho ainda existe e se é um arquivo regular ou um link simbólico.
8. Ordena e elimina duplicatas antes de retornar o resultado.

Arquivos apagados são ignorados porque não existem no sistema de arquivos. Para renomeações e cópias, o registro adicional emitido pelo Git é consumido, mas apenas o caminho principal é considerado.

### Componentes principais

- `GitOutput`: estrutura privada que reúne o `ExitStatus` e a saída binária (`stdout`) de um comando Git.
- `GitStatusTree`: estrutura pública com:
  - `root`: diretório canônico usado como raiz da consulta;
  - `files`: lista pública de `PathBuf` relativos à raiz.
- `GitStatusTree::list`: API pública principal para obter os caminhos em status.
- `tree_ignored`: executa `git ls-files` para identificar caminhos excluídos por `.treeignore`.
- `git`: encapsula a execução de comandos Git usando `BashService` e `Invocation`, mantendo a saída em formato binário.
- `path_from_bytes`: converte caminhos retornados pelo Git em `PathBuf`. Em Unix, preserva bytes arbitrários usando `OsStrExt`; em outras plataformas, exige UTF-8.
- Módulo `tests`: cria repositórios Git temporários e testa:
  - arquivos em diferentes estados do Git;
  - aplicação de `.treeignore`;
  - consultas a partir de subdiretórios;
  - rejeição de diretórios inexistentes ou que não são repositórios.

### Dependências e integrações

- `crate::services::{BashService, Invocation}`: usados para executar o processo externo `git` sem interpolar comandos em shell.
- `std::fs` e `std::io`: usados para inspecionar arquivos e descartar a saída padrão do processo.
- `std::path`: fornece representação e manipulação de caminhos.
- `std::collections::BTreeSet`: armazena os caminhos excluídos de forma ordenada.
- `std::process::ExitStatus`: representa o resultado de execução do Git.

O arquivo depende de um ambiente Git acessível e de um diretório que pertença a uma árvore de trabalho Git.

### Observações

- Os erros são propagados como `Result<_, String>`, com mensagens específicas para falhas de resolução de caminho, execução do Git, status inválido, `.treeignore` e inspeção do sistema de arquivos.
- A saída do Git é processada como bytes para manter compatibilidade com caminhos não UTF-8 em Unix.
- O arquivo usa `symlink_metadata`, portanto links simbólicos existentes são incluídos, mas diretórios e links para diretórios não são.
- A implementação considera apenas o conteúdo do arquivo; o comportamento exato de `BashService`, `Invocation` e a integração de `GitStatusTree` com o restante da aplicação não está demonstrado.
