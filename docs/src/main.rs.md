## Resumo
Executa o comando recebido e define o código de saída do programa.

## Funcionamento
Repassa os argumentos da linha de comando, exceto o nome do executável, para `run::run`. Em caso de sucesso, retorna `ExitCode::SUCCESS`; se houver erro, exibe a mensagem em stderr e retorna o código 2.

## Importações
- `std::env`: Obtém os argumentos da linha de comando.
- `std::process::ExitCode`: Representa o código de saída do processo.
- `run`: Executa o comando com os argumentos recebidos.
