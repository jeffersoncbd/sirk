## Resumo
Executa um agente com a entrada fornecida e retorna seu resultado textual.

## Funcionamento
Monta um fluxo com uma etapa para o agente, executa-o silenciosamente no diretório indicado e extrai a saída `result`; retorna erro se a execução falhar ou não produzir essa saída.

## Importações
- `runner::run_silent_with`: Executa o fluxo e coleta as saídas.
- `workflow`: Define o fluxo e a etapa do agente.
- `std::path::Path`: Representa o diretório de execução.
