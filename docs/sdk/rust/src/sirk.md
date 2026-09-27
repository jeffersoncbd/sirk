## Resumo
`Sirk` reúne o estado de um processo filho e seus canais de comunicação.

## Funcionamento
Armazena o processo, a entrada padrão opcional, a saída padrão com buffer e o próximo identificador. Como o arquivo não define funções, não há fluxo de execução nem tratamento de erros.

## Importações
- `std::io::BufReader`: Armazena a saída padrão com buffer.
- `std::process::Child`: Representa o processo filho.
- `std::process::ChildStdin`: Representa a entrada padrão do processo.
- `std::process::ChildStdout`: Representa a saída padrão do processo.
