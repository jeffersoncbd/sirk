## Resumo
Estrutura que implementa `UserInput` sempre falhando, bloqueando execuções que exigem interação do usuário.

## Funcionamento
`NoInput` é um marcador sem estado. Seu `ask` ignora a pergunta recebida e retorna `Err` imediatamente com a mensagem "this execution requires user input", servindo de guard para fluxos que não aceitam entrada via terminal.

## Importações
- `crate::input::UserInput`: Trait de abstração de entrada que `NoInput` implementa.
