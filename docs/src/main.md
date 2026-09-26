## Resumo
Ponto de entrada do CLI: delega os argumentos ao módulo `run` e converte o resultado em código de saída do processo.

## Funcionamento
`main` coleta os argumentos da linha de comando ignorando o nome do binário e chama `run::run`. Em caso de sucesso, devolve `ExitCode::SUCCESS`; em caso de erro, imprime a mensagem em `stderr` prefixada com `error:` e retorna o código 2. Os submódulos são declarados com `#[path]` por estarem fora da pastaconventional do arquivo, e o bloco `#[cfg(test)]` só é compilado em testes.

## Importações
- `std::env`: Lê os argumentos da linha de comando e os entrega ao fluxo de execução.
- `std::process::ExitCode`: Traduz `Ok`/`Err` em código de saída para o sistema operacional.
- `main/create_agent.rs`: Módulo auxiliar montado por `#[path]`, sem uso direto em `main`.
- `main/print_usage.rs`: Módulo auxiliar montado por `#[path]`, sem uso direto em `main`.
- `main/resume.rs`: Módulo auxiliar montado por `#[path]`, sem uso direto em `main`.
- `main/run.rs`: Executa a lógica principal do CLI a partir dos argumentos recebidos.
- `main/run_workflow.rs`: Módulo auxiliar montado por `#[path]`, sem uso direto em `main`.
- `main/usage.rs`: Módulo auxiliar montado por `#[path]`, sem uso direto em `main`.
- `main/workflow_path.rs`: Módulo auxiliar montado por `#[path]`, sem uso direto em `main`.
