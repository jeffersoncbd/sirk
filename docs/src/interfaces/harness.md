## Resumo
Converte uma `RunRequest` neutra em uma `Invocation` específica do adaptador de harness.

## Funcionamento
Recebe o prompt, diretório de trabalho, modelo opcional e flag de stream de eventos, validando opções suportadas pelo adaptador e construindo uma `Invocation` para o CLI do agente de codificação; em caso de configuração inválida ou opção não suportada, retorna `HarnessError` correspondente.

## Importações
- `crate::interfaces::Invocation`: Tipo retornado representando a invocação preparada para o CLI.
- `std::path::PathBuf`: Armazena o diretório de trabalho especificado na requisição.
