## Resumo
Define o serviço compartilhado que executa processos por meio do Bash.

## Funcionamento
`BashService` guarda o executável do Bash e organiza a implementação em submódulos. As saídas representam o status do processo e seus dados capturados como texto ou bytes.

## Importações
- `std::process::ExitStatus`: tipo usado para representar o status de saída.
