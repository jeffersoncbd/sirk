## Resumo
Executa a ferramenta solicitada no diretório indicado e retorna seu resultado.

## Funcionamento
Para `TREE`, lista os arquivos do diretório e formata os caminhos; para `READ`, lê o conteúdo solicitado. Erros dessas operações são propagados como `String`; nomes desconhecidos retornam erro.

## Importações
- `std::path::Path`: Representa o diretório de execução.
- `format_paths`: Formata a lista de caminhos.
- `read`: Lê o conteúdo solicitado.
- `Tree`: Lista arquivos do diretório.
