## Resumo
Lê um arquivo UTF-8 dentro do diretório de execução, respeitando as regras de acesso.

## Funcionamento
Rejeita caminhos vazios, diretórios e caminhos que escapem da raiz; retorna erros se não conseguir resolver, validar ou ler o arquivo. Antes da leitura, verifica se o caminho está ignorado e, nesse caso, retorna `AccessDenied`.

## Importações
- `std::fs`: Leitura do conteúdo do arquivo.
- `std::path::Path`: Representação e validação dos caminhos.
- `super::ignored`: Verifica se o arquivo está bloqueado.
