## Resumo
Implementa a entrada de usuário via terminal, expondo `ask` (leitura de resposta em loop) e `await_confirmation` (confirmação via `/dev/tty`).

## Funcionamento
`ask` imprime a pergunta no stdout e, em laço, exibe o prompt `> `, faz flush da saída e lê uma linha de `stdin`. Linhas vazias são rejeitadas com mensagemorientando uso de `/cancel`; esse comando encerra com `Err("input cancelled")`, assim como EOF, que retorna `Err("input closed")`. Respostas válidas são devolvidas sem quebras de linha finais. `await_confirmation` ignora o stdin (indisponível em hooks de git) e reabre o terminal real via `/dev/tty`, escrevendo o prompt diretamente no dispositivo e aplicando as mesmas regras de EOF, cancelamento e erro — falhas de I/O viram `Err(String)` descritivo.

## Importações
- `std::fs::OpenOptions`: Abre `/dev/tty` para leitura/escrita no modo interativo.
- `std::io`: Fornece `stdin`/`stdout` e o trait `Write` para flush e escrita do prompt.
- `std::io::BufRead`: Habilita `read_line` para ler a resposta do usuário.
- `std::io::BufReader`: Bufferiza o terminal clonado para leituras de linha eficientes.
- `std::io::Write`: Necessário para `write!` e `flush` no descritor do terminal.
- `crate::interfaces::UserInput`: Trait que define o contrato de `ask` e `await_confirmation`.
