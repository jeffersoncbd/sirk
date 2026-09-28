## Resumo
Executa o comando recebido e define o código de saída do programa.

## Funcionamento
Passa os argumentos, exceto o nome do executável, para `run::run`. Se a execução for bem-sucedida, retorna `ExitCode::SUCCESS`; caso contrário, exibe o erro em stderr e retorna o código 2.

## Importações
- `std::env`: Obtém os argumentos da linha de comando.
- `std::process::ExitCode`: Representa o código de saída do processo.
- `run`: Executa o comando com os argumentos recebidos.
