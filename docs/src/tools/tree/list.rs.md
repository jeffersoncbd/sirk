## Resumo
Lista arquivos existentes e não ignorados sob um diretório de trabalho Git.

## Funcionamento
Resolve e valida o diretório, consulta o Git para obter arquivos rastreados e não rastreados e aplica as regras de `.treeignore`. Converte os caminhos preservando bytes não UTF-8, ignora arquivos removidos e entradas que não são arquivos, inclui links simbólicos sem segui-los e retorna os caminhos ordenados e sem duplicatas. Erros de resolução, execução do Git ou inspeção de arquivos são retornados como `String`.

## Importações
- `crate::services`: executa comandos Git com argumentos estruturados.
- `std::collections::BTreeSet`: armazena caminhos excluídos.
- `std::fs`: inspeciona metadados sem seguir links.
- `std::io`: descarta a saída do processo e identifica arquivos ausentes.
- `std::path::Path`: representa o diretório recebido.
- `super::Tree`: constrói o resultado com raiz e arquivos.
- `super::path::path_from_bytes`: converte caminhos preservando bytes.
