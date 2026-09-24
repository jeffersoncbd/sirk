### Resumo

Este arquivo implementa a funcionalidade `TREE`, responsável por listar arquivos de um diretório dentro de um repositório Git. A listagem respeita os arquivos rastreados, as regras padrão de exclusão do Git e regras adicionais definidas em `.treeignore`.

### Funcionamento

A função pública `Tree::list`:

1. Resolve o diretório para um caminho absoluto e verifica se ele é um diretório.
2. Executa `git ls-files` para obter arquivos rastreados e arquivos não ignorados.
3. Executa uma segunda consulta ao Git para identificar arquivos excluídos por `.treeignore`.
4. Remove da lista os caminhos afetados por `.treeignore`.
5. Verifica no sistema de arquivos se cada entrada ainda é um arquivo regular ou um link simbólico.
6. Converte os caminhos, ordena-os, elimina duplicatas e retorna um `Tree`.

A comunicação com o Git ocorre através de `BashService` e `Invocation`. A saída usa o separador NUL (`-z`), preservando corretamente nomes com espaços, quebras de linha e, em sistemas Unix, bytes que não formam UTF-8 válido.

Links simbólicos são listados, mas não seguidos. Diretórios, submódulos, arquivos removidos do disco e metadados do Git não entram no resultado.

### Componentes principais

- `Tree`: struct pública que armazena:
  - `root`: diretório canônico usado como raiz da busca.
  - `files`: caminhos relativos encontrados, ordenados e únicos.

- `Tree::list`: função pública que realiza toda a descoberta e filtragem dos arquivos. Retorna `Result<Self, String>` para representar falhas de resolução de caminho, execução do Git, filtragem ou inspeção do sistema de arquivos.

- `path_from_bytes`: converte os caminhos retornados pelo Git em `PathBuf`.
  - Em Unix, preserva os bytes originais usando `OsStrExt`.
  - Em outras plataformas, exige que o caminho seja UTF-8 válido.

- Módulo de testes:
  - Cria repositórios Git temporários.
  - Verifica regras de `.gitignore`, `.git/info/exclude`, exclusões globais e `.treeignore`.
  - Testa arquivos rastreados, arquivos removidos, links simbólicos e caminhos não UTF-8.
  - Confirma que a operação não altera o índice Git.

### Dependências e integrações

- `crate::services::{BashService, Invocation}`: executa comandos externos, especialmente o Git.
- `std::fs` e `std::io`: acessam metadados dos arquivos e capturam a saída binária dos comandos.
- `BTreeSet`: armazena os caminhos excluídos por `.treeignore` para filtragem eficiente e ordenada.
- Git: fornece a semântica de arquivos rastreados e regras padrão de ignore.

O arquivo depende de que o diretório esteja dentro de uma árvore de trabalho Git acessível e de que o módulo `services` forneça a execução correta de processos externos.

### Observações

Erros são propagados como mensagens `String`; não há `panic!` na implementação principal. Os testes usam `unwrap`, apropriado apenas para falhas inesperadas durante a preparação das fixtures.

A função não modifica o índice Git, mas executa comandos Git e consulta o estado atual do sistema de arquivos.
