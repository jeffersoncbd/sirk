## Resumo
Lê respostas e confirmações do usuário pelo terminal.

## Funcionamento
`ask` exibe a pergunta e repete a leitura até receber uma resposta não vazia; `/cancel`, entrada fechada ou falhas de I/O retornam erro. `await_confirmation` lê diretamente do terminal interativo (`/dev/tty`), aceita qualquer linha como confirmação e retorna erro em caso de cancelamento, entrada fechada ou falha.

## Importações
- `std::fs::OpenOptions`: Abre o terminal interativo para confirmação.
- `std::io`: Lê e escreve no terminal e trata operações de I/O.
- `crate::interfaces::UserInput`: Define a interface implementada.
