### Resumo

O arquivo implementa as operações públicas `READ` e de enumeração de conteúdo. `read` valida e lê conteúdo UTF-8 de arquivos dentro de um diretório de execução, respeitando regras de bloqueio em `.readignore`; `enumerate` adiciona coordenadas de linha sem perder o conteúdo original, e `enumerated_content` reconstitui esse conteúdo.

### Funcionamento

A função pública `read`:

1. Rejeita caminhos vazios.
2. Resolve canonicamente o diretório raiz e o caminho informado.
3. Garante que o arquivo esteja dentro do diretório de execução, impedindo travessias como `../`.
4. Verifica se o caminho corresponde a um arquivo regular.
5. Consulta `.readignore`, quando existente, e retorna `AccessDenied` para arquivos correspondentes às regras.
6. Lê o arquivo com `fs::read_to_string`, retornando erro caso o conteúdo não seja UTF-8 ou não possa ser acessado.

O `.readignore` aceita comentários iniciados por `#`, padrões para nomes de componentes, curingas `*` e `?`, além de `**` para múltiplos componentes de caminho. Padrões terminados em `/` representam diretórios.

`enumerate` produz o cabeçalho `Line | Content` e prefixa cada linha com seu número baseado em um, preservando quebras de linha, inclusive quando o conteúdo termina sem uma quebra. `enumerated_content` valida esse formato, exige numeração sequencial e remove os prefixos para recuperar exatamente o conteúdo original; formatos inválidos geram `Err`.

Os erros são representados por `Result<String, String>` e propagados com o operador `?`. Há também um `expect` interno para uma condição previamente validada: o caminho deve estar dentro da raiz.

### Componentes principais

- `read`: API pública que valida, autoriza e lê o arquivo.
- `enumerate`: API pública que enumera as linhas preservando o conteúdo original.
- `enumerated_content`: API pública que valida e desfaz a enumeração de linhas.
- `read_ignored`: carrega `.readignore` e verifica se o arquivo está bloqueado.
- `matches_ignore`: interpreta o padrão e decide como compará-lo com o caminho relativo.
- `matches_components`: compara componentes de diretórios, incluindo o curinga `**`.
- `matches_component`: compara partes individuais usando `*` e `?`.
- Módulo `tests`: verifica regras de `.readignore`, comentários, padrões de diretório e a preservação exata do conteúdo enumerado.
- `std::fs`, `std::io` e `std::path::Path`: fornecem operações de sistema de arquivos, identificação de erros e manipulação de caminhos.

### integrações

O arquivo expõe publicamente:

- `read(directory: &Path, path: &str) -> Result<String, String>`
- `enumerate(content: &str) -> String`
- `enumerated_content(numbered: &str) -> Result<String, String>`

As funções de interpretação do `.readignore` são privadas e usadas apenas internamente pelo módulo.