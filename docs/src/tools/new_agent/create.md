## Resumo
Cria um novo agente em um diretório, delegando a geração ao comando de shell e devolvendo o `PathBuf` do agente salvo.

## Funcionamento
A função apenas encapsula `super::create_with`, informando como executar a geração: o `invocation` recebido é rodado via `BashService::execute_streaming`. Falhas de execução viram `Err` com o prefixo "agent generation failed"; se o processo terminar com status de erro, retorna `Err` informando que nenhum agente foi salvo. Só em caso de sucesso o `stdout` é repassado a `create_with`, que resolve o `PathBuf` final.

## Importações
- `crate::input::UserInput`: Trait que abstrai a coleta de entrada do usuário
- `crate::services::BashService`: Executa o comando de geração do agente
- `std::path::Path`: Diretório base onde o agente será criado
- `std::path::PathBuf`: Caminho do agente gerado, retornado em sucesso
