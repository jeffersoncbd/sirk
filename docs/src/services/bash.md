### Resumo

Este arquivo implementa o serviço compartilhado `BashService`, responsável por executar comandos externos através do Bash, preservando corretamente argumentos, diretório de trabalho, saída padrão e status de encerramento.

### Funcionamento

`BashService` recebe uma `Invocation`, que contém o programa, seus argumentos e o diretório de execução. O comando é convertido para uma string usando aspas simples compatíveis com POSIX e executado como:

```text
bash -lc "exec -- ..."
```

Cada argumento é escapado individualmente por `shell_quote`, impedindo que espaços, aspas, quebras de linha ou construções como `$(...)` sejam interpretados como código shell.

A saída padrão do processo é:

- enviada em blocos de 8 KiB para um destino fornecido pelo chamador;
- armazenada simultaneamente em memória;
- convertida para `String` em `execute_to`, exigindo UTF-8 válido;
- mantida como bytes em `execute_bytes_to`, permitindo lidar com saídas binárias, como caminhos Git separados por NUL.

A entrada padrão do processo é fechada (`Stdio::null()`), enquanto o erro padrão permanece ligado ao processo pai. Se o destino da saída falhar, o processo filho é encerrado e aguardado antes de retornar o erro.

### Componentes principais

- `BashService`: serviço público que armazena o executável do shell a ser usado.
- `BashService::new`: cria o serviço com um executável configurável.
- `BashService::default`: usa `bash` como executável padrão.
- `execute_streaming`: executa o comando e envia a saída diretamente para a saída padrão do processo atual.
- `execute_to`: transmite e captura a saída, retornando-a como `String`.
- `execute_bytes_to`: variante interna que preserva a saída como `Vec<u8>`.
- `render`: monta a linha de comando com `exec --` e argumentos devidamente escapados.
- `shell_quote`: função privada que envolve valores em aspas simples e escapa aspas internas.
- `ProcessOutput`: tipo público contendo `ExitStatus` e stdout textual.
- `BinaryProcessOutput`: tipo interno contendo `ExitStatus` e stdout binário.
- Módulo de testes: verifica preservação de argumentos, diretório de trabalho, status de saída, transmissão de bytes, falhas do destino e renderização segura.

### Dependências e integrações

- `std::process`: cria e controla o processo filho por meio de `Command`, `Stdio` e `ExitStatus`.
- `std::io`: lê stdout, escreve no destino, faz flush e trata erros de I/O.
- `super::Invocation`: tipo definido no módulo pai, usado para descrever o programa, argumentos e diretório de execução.
- O Bash é usado como camada de execução, mas o programa real e seus argumentos continuam sendo definidos pela `Invocation`.

### Observações

A conversão para `String` falha com `io::ErrorKind::InvalidData` quando a saída não é UTF-8 válida. Por isso, `execute_bytes_to` existe para consumidores que precisam preservar bytes arbitrários.

O método usa `expect` apenas para uma condição interna esperada: o stdout deve estar disponível porque foi configurado como `Stdio::piped()`. Não há uso de `unsafe`, concorrência explícita ou `async/await`.
