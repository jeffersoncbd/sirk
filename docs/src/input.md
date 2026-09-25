### Resumo

Este arquivo define a fronteira de interação com o usuário pelo terminal. Ele oferece uma trait abstrata para fazer perguntas e uma implementação concreta que lê respostas da entrada padrão.

### Funcionamento

`TerminalInput::ask`:

1. Exibe a pergunta recebida.
2. Mostra o prompt `> ` e força sua exibição com `stdout().flush()`.
3. Lê uma linha da entrada padrão.
4. Remove `\r` e `\n` do final da resposta.
5. Retorna erro se a entrada for fechada ou se o usuário digitar `/cancel`.
6. Ignora respostas vazias, solicitando uma nova resposta.
7. Retorna a resposta não vazia como `String`.

Erros de leitura e de atualização da saída são convertidos para `String` usando `map_err`.

### Componentes principais

- `UserInput`: trait pública que define o método `ask`, permitindo diferentes implementações de interação.
- `TerminalInput`: struct pública, sem campos, usada para interação direta com o terminal.
- `TerminalInput::ask`: implementação da leitura e validação das respostas do usuário.

### Dependências e integrações

- `std::io::{self, Write}`:
  - `stdin` é usado para ler as respostas.
  - `stdout` e `Write::flush` são usados para exibir imediatamente o prompt.
- A trait pode ser usada por outras partes do projeto para desacoplar a lógica da aplicação do mecanismo concreto de entrada.

### Observações

O arquivo não executa lógica de negócio nem persiste dados. A interação é síncrona e bloqueante. O conteúdo das respostas é preservado, exceto pelos caracteres de fim de linha removidos; espaços internos e espaços nas extremidades são mantidos na resposta aceita.
