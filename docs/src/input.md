### Resumo

Este arquivo define a fronteira síncrona de interação com o usuário pelo terminal. Ele abstrai perguntas e confirmações por meio da trait `UserInput` e fornece a implementação concreta `TerminalInput`.

### Funcionamento

`UserInput::ask` é a operação obrigatória para obter uma resposta textual. A trait também fornece `await_confirmation` com implementação padrão que chama `ask` e descarta a resposta.

`TerminalInput::ask` imprime a pergunta, exibe o prompt `> `, força sua saída com `stdout().flush()` e lê uma linha da entrada padrão. Os caracteres `\r` e `\n` finais são removidos, mas os demais espaços são preservados. Respostas vazias ou compostas apenas por espaços são rejeitadas e solicitadas novamente. A entrada fechada, erros de leitura ou escrita e a resposta `/cancel` resultam em `Err(String)`.

`TerminalInput::await_confirmation` abre `/dev/tty` para permitir interação mesmo quando a entrada padrão está fechada, como pode ocorrer em hooks do Git. Ele clona o descritor para leitura, escreve o prompt no terminal, força a saída e lê uma linha. Qualquer resposta diferente de `/cancel`, inclusive vazia, confirma a operação; falhas ao abrir, clonar, escrever ou ler o terminal resultam em erro.

### Componentes principais

- `UserInput`: trait pública que define `ask` e a implementação padrão de `await_confirmation`.
- `TerminalInput`: struct pública sem campos, usada para interação direta com o terminal.
- `TerminalInput::ask`: implementa perguntas interativas com validação de resposta não vazia.
- `TerminalInput::await_confirmation`: implementa confirmações usando diretamente `/dev/tty`, sem exigir conteúdo não vazio.

### integrações

- `std::io::{self, BufRead, BufReader, Write}` fornece leitura de linhas, buffers, acesso à entrada padrão e escrita com `flush`.
- `std::fs::OpenOptions` abre `/dev/tty` para confirmações independentes da entrada padrão.
- A trait `UserInput` é a interface pública para mecanismos alternativos de interação.

A interação é síncrona e bloqueante. O arquivo não persiste dados, executa lógica de negócio nem usa código `unsafe`; erros são propagados como `String` por meio de `Result`.