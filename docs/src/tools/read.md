### Resumo

O arquivo implementa a operação `READ`, responsável por ler o conteúdo UTF-8 de um arquivo localizado dentro de um diretório de execução, bloqueando caminhos externos e arquivos definidos em `.readignore`.

### Funcionamento

A função pública `read`:

1. Rejeita caminhos vazios.
2. Resolve canonicamente o diretório raiz e o caminho informado.
3. Garante que o arquivo esteja dentro do diretório de execução, impedindo travessias como `../`.
4. Verifica se o caminho corresponde a um arquivo regular.
5. Consulta `.readignore`, quando existente, e retorna `AccessDenied` para arquivos correspondentes às regras.
6. Lê o arquivo com `fs::read_to_string`, retornando erro caso o conteúdo não seja UTF-8 ou não possa ser acessado.

O `.readignore` aceita comentários iniciados por `#`, padrões para nomes de componentes, curingas `*` e `?`, além de `**` para múltiplos componentes de caminho. Padrões terminados em `/` representam diretórios.

Os erros são propagados com `Result<String, String>` e o operador `?`. Há também um `expect` interno para uma condição que já foi validada: o caminho deve estar dentro da raiz.

### Componentes principais

- `read`: API pública que valida, autoriza e lê o arquivo.
- `read_ignored`: carrega `.readignore` e verifica se o arquivo está bloqueado.
- `matches_ignore`: interpreta o padrão e decide como compará-lo com o caminho relativo.
- `matches_components`: compara componentes de diretórios, incluindo o curinga `**`.
- `matches_component`: compara partes individuais usando `*` e `?`.
- Módulo `tests`: contém testes para arquivos ignorados por nome, extensões, diretórios e comentários.
- `std::fs`, `std::io` e `std::path::Path`: fornecem operações de sistema de arquivos, identificação de erros e manipulação de caminhos.

### integrações

O arquivo expõe publicamente apenas:

- `read(directory: &Path, path: &str) -> Result<String, String>`

As funções de interpretação do `.readignore` são privadas e usadas apenas internamente pelo módulo.
