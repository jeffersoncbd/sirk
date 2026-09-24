### Resumo

Este arquivo define uma abstração para entrada de texto do usuário e uma implementação baseada no terminal. Ele é usado para fazer perguntas, receber respostas e permitir cancelamento explícito.

### Funcionamento

A trait pública `UserInput` define o contrato de interação:

- `ask` recebe uma pergunta.
- Retorna `Ok(String)` quando há uma resposta válida.
- Retorna `Err(String)` quando a entrada é encerrada, cancelada ou ocorre erro de I/O.

`TerminalInput` implementa esse contrato usando a entrada e saída padrão:

1. Exibe a pergunta.
2. Mostra o prompt `> `.
3. Limpa o buffer de saída para garantir que o prompt apareça imediatamente.
4. Lê uma linha do terminal.
5. Retorna erro se a entrada for fechada.
6. Trata `/cancel` como cancelamento.
7. Ignora respostas vazias ou compostas apenas por espaços.
8. Retorna a resposta preservando seu conteúdo, exceto `\r` e `\n`.

### Componentes principais

- `UserInput`: trait pública que abstrai a origem das respostas do usuário.
- `TerminalInput`: struct pública, sem campos, usada para interação via terminal.
- `TerminalInput::ask`: implementação concreta da leitura interativa.
- `std::io::{self, Write}`: fornece acesso à entrada/saída padrão e ao método `flush`.

### Dependências e integrações

O arquivo depende apenas da biblioteca padrão Rust, especialmente de `std::io`.

A trait `UserInput` permite que outras partes do projeto utilizem entrada de usuário sem depender diretamente do terminal, facilitando implementações alternativas, como entradas simuladas em testes.

### Observações

- O método usa `Result<String, String>` e o operador `?` para propagar erros de I/O convertidos em texto.
- A leitura é síncrona e bloqueia até o usuário responder.
- `/cancel` e o fechamento de `stdin` são tratados como erros distintos.
- O código não valida o conteúdo da resposta além de rejeitar entradas vazias.
