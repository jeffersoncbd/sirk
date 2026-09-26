## Resumo
Executa a ferramenta `name` no diretório indicado, sem entrada via stdin.

## Funcionamento
Fachada sobre `execute_with_input`: delega a execução passando entrada vazia (`""`) e o diretório recebido, propagando o `Result<String, String>` (sucesso com a saída ou erro em `String`) sem tratamento adicional.

## Importações
- `std::path::Path`: Representa o diretório de trabalho da execução.
- `super::execute_with_input::execute_with_input`: Executa a ferramenta com a entrada fornecida.
