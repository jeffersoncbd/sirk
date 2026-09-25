### Resumo

Este arquivo implementa `BashService`, um serviço compartilhado para executar processos por meio do Bash. Ele transforma um `Invocation` em um comando com argumentos protegidos contra interpretação indevida pelo shell, executa o processo e transmite/captura sua saída padrão.

### Funcionamento

O serviço inicia o executável configurado — por padrão, `bash` — usando `bash -lc`. O comando é montado com:

- `exec --`, para substituir o processo do Bash pelo programa chamado;
- o programa e cada argumento individualmente protegidos por aspas simples;
- o diretório de trabalho definido em `Invocation`.

Durante a execução:

1. A entrada padrão do processo é fechada com `Stdio::null()`.
2. A saída padrão é capturada em um pipe.
3. A saída de erro permanece ligada ao processo-pai.
4. A saída padrão é lida em blocos de 8.192 bytes.
5. Cada bloco é escrito no destino fornecido e armazenado em memória.
6. O processo filho é aguardado, preservando seu `ExitStatus`.

A variante `execute_to` converte a saída capturada de bytes para `String`, retornando erro caso ela não seja UTF-8 válida. Já `execute_bytes_to` preserva dados binários, inclusive caminhos separados por NUL produzidos pelo Git.

Se o destino da saída falhar, o processo filho é encerrado e aguardado antes que o erro seja retornado.

### Componentes principais

- `BashService`: serviço público que armazena o executável do shell.
  - `new`: cria o serviço com um executável personalizado.
  - `default`: usa `"bash"`.
  - `execute_streaming`: executa e transmite a saída diretamente para `stdout`.
  - `execute_to`: transmite a saída para um destino e retorna-a como `String`.
  - `execute_bytes_to`: versão interna que preserva a saída como `Vec<u8>`.
  - `render`: converte um `Invocation` em uma linha de comando shell protegida.

- `ProcessOutput`: estrutura pública contendo:
  - `status`: o `ExitStatus` do processo;
  - `stdout`: saída padrão convertida para `String`.

- `BinaryProcessOutput`: estrutura privada do crate usada para transportar saída binária sem conversão UTF-8.

- `shell_quote`: função privada que envolve valores em aspas simples e escapa aspas simples internas usando a forma POSIX `'\"'\"'`.

- Testes:
  - verificam preservação de argumentos especiais;
  - confirmam captura e transmissão simultâneas da saída;
  - validam diretório de trabalho e código de saída;
  - verificam propagação de erro de um destino quebrado;
  - conferem a renderização correta dos argumentos para o shell.

### Dependências e integrações

O arquivo usa:

- `std::io` para leitura, escrita, captura de saída e erros de I/O;
- `std::process` para criar e controlar processos filhos;
- `super::Invocation`, tipo interno que fornece programa, argumentos e diretório de trabalho.

Ele integra-se às funcionalidades do projeto que precisam executar comandos externos por meio de Bash, mantendo a construção dos argumentos centralizada no serviço.

### Observações

O arquivo não executa diretamente os comandos durante sua definição; eles são executados apenas quando os métodos correspondentes são chamados. A proteção contra expansão do shell depende da função `shell_quote`, que trata cada programa e argumento como uma única palavra shell. O código também usa `expect` para assumir que o stdout foi configurado como pipe; essa condição é garantida pela própria configuração do `Command`.
