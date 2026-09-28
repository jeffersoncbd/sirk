## Resumo
Verifica se um arquivo deve ser ignorado conforme as regras de `.readignore` na raiz.

## Funcionamento
Se `.readignore` não existir, retorna `Ok(false)`; se não puder ser lido, retorna `Err("AccessDenied")`. Normaliza o caminho relativo e retorna `Ok(true)` quando encontra um padrão não vazio, não comentado e correspondente.

## Importações
- `std::fs`: Lê o arquivo `.readignore`.
- `std::io`: Identifica quando o arquivo não existe.
- `std::path::Path`: Representa os caminhos recebidos.
- `super::ignore`: Compara o caminho com os padrões de exclusão.
