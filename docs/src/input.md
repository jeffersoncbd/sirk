### Resumo

Este arquivo define a fronteira de interação com o usuário pelo terminal. Ele oferece uma trait abstrata para perguntas e confirmações, além de uma implementação concreta que lê respostas da entrada padrão.

### Funcionamento

`TerminalInput::ask`:

1. Exibe a pergunta recebida.
2. Mostra o prompt `> ` e força sua exibição com `stdout().flush()`.
3. Lê uma linha da entrada padrão.
4. Remove `\r` e `\n` do final da resposta.
5. Retorna erro se a entrada for fechada ou se o usuário digitar `/cancel`.
6. Ignora respostas vazias ou compostas apenas por espaços, solicitando uma nova resposta.
7. Retorna a resposta não vazia como `String`, preservando os espaços nas extremidades.

`TerminalInput::await_confirmation` abre `/dev/tty` para garantir interação com um terminal mesmo quando a entrada padrão está fechada, como em hooks do Git. Exibe o prompt, força sua saída com `flush()`, lê uma linha e retorna `Ok(())` para qualquer resposta, inclusive vazia. A resposta `/cancel` produz erro; falhas ao abrir, clonar ou ler o terminal também são convertidas para `String`.
- `UserInput`: trait pública que define `ask` e fornece `await_confirmation` com implementação padrão baseada em `ask`.
- `TerminalInput`: struct pública, sem campos, usada para interação direta com o terminal.
- `TerminalInput::ask`: implementação da leitura e validação de respostas textuais.
- `TerminalInput::await_confirmation`: implementação específica para aguardar uma confirmação sem validar o conteúdo da resposta.

### integrações

- `std::io::{self, Write}`:
  - `stdin` é usado para ler as respostas.
  - `stdout` e `Write::flush` são usados para exibir imediatamente os prompts.
- A trait `UserInput` pode ser implementada por outros mecanismos de interação e usada para desacoplar a lógica da aplicação da entrada concreta.
O arquivo não executa lógica de negócio nem persiste dados. A interação é síncrona e bloqueante. Em `ask`, o conteúdo da resposta é preservado, exceto pelos caracteres de fim de linha removidos; espaços internos e espaços nas extremidades são mantidos na resposta aceita.O arquivo não executa lógica de negócio nem persiste dados. A interação é síncrona e bloqueante. Em `ask`, o conteúdo da resposta é preservado, exceto pelos caracteres de fim de linha removidos; espaços internos e espaços nas extremidades são mantidos na resposta aceita.O arquivo não executa lógica de negócio nem persiste dados. A interação é síncrona e bloqueante. Em `ask`, o conteúdo da resposta é preservado, exceto pelos caracteres de fim de linha removidos; espaços internos e espaços nas extremidades são mantidos na resposta aceita.O arquivo não executa lógica de negócio nem persiste dados. A interação é síncrona e bloqueante. O conteúdo das respostas é preservado, exceto pelos caracteres de fim de linha removidos; espaços internos e espaços nas extremidades são mantidos na resposta aceita.
