## Resumo
Remove um arquivo regular localizado dentro do diretório de execução.

## Funcionamento
Rejeita caminhos vazios, externos ao diretório ou que atravessem links simbólicos e componentes que não sejam diretórios. Confirma que o destino é um arquivo regular, remove-o e sincroniza o diretório pai; falhas são retornadas como `Err` com uma mensagem descritiva.

## Importações
- `std::fs::{self, File}`: inspeciona, remove e sincroniza arquivos e diretórios.
- `std::path::{Component, Path}`: valida e resolve os componentes do caminho.
