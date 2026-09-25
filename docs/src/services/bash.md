### Resumo

O arquivo implementa o serviço compartilhado para executar processos por meio do Bash. Ele transforma uma `Invocation` em um comando shell com argumentos protegidos contra interpretação indevida, executa o processo e captura/transmite sua saída.

### Funcionamento

`BashService` armazena o executável do shell, usando `"bash"` por padrão. A execução ocorre com `bash -lc`, no diretório de trabalho e com as variáveis de ambiente definidos pela `Invocation`.

O processo:

- recebe os argumentos renderizados com aspas simples POSIX;
- tem `stdin` fechado (`Stdio::null()`);
- transmite `stderr` diretamente para o processo pai;
- transmite `stdout` para um destino fornecido e também o captura;
- preserva a saída como bytes antes de convertê-la para `String`;
- encerra e aguarda o processo caso o destino de saída falhe.

A função `shell_quote` evita expansão de comandos, substituição de comandos e divisão de argumentos. Há uma exceção controlada para valores no formato `$VAR`, desde que a variável exista no ambiente da invocação e use apenas letras maiúsculas, dígitos ou `_`; nesse caso, ela é expandida pelo shell sem inserir o conteúdo diretamente no comando renderizado.

Erros de criação, leitura, escrita ou espera do processo são propagados por `io::Result`. Saída que não seja UTF-8 resulta em `io::ErrorKind::InvalidData`. Um código de saída diferente de zero não é convertido em erro de I/O: permanece disponível em `ExitStatus`.

### Componentes principais

- `BashService`: serviço público que mantém o caminho do executável Bash.
- `BashService::new`: cria o serviço com um executável customizado.
- `BashService::default`: usa o executável `"bash"`.
- `execute_streaming`: executa o processo e transmite a saída para o stdout do processo atual.
- `execute_to`: transmite e captura stdout como `String`.
- `execute_bytes_to`: variante interna que preserva stdout como `Vec<u8>`, inclusive dados binários e caminhos delimitados por NUL.
- `render`: monta a linha passada para `bash -lc`, iniciada por `exec --`.
- `ProcessOutput`: tipo público contendo `ExitStatus` e stdout textual.
- `BinaryProcessOutput`: tipo restrito ao crate, contendo status e stdout binário.
- `shell_quote`: função privada responsável pelo escaping seguro de cada argumento.
- Módulo de testes: verifica preservação de argumentos, diretório de trabalho, status de saída, falhas do destino de saída e tratamento de variáveis de ambiente.

### integrações

- Expõe `BashService` e `ProcessOutput` publicamente.
- Depende do tipo interno `Invocation`, importado de `super`, para fornecer programa, argumentos, diretório de trabalho e ambiente.
- Utiliza tipos da biblioteca padrão para processos (`Command`, `Stdio`, `ExitStatus`) e I/O (`Read`, `Write`, `io`).
- `execute_bytes_to` e `BinaryProcessOutput` são acessíveis apenas dentro do crate.
