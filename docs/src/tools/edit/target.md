## Resumo
Resolve o caminho de um arquivo a ser editado, garantindo que ele exista e fique dentro do diretório de execução.

## Funcionamento
Canonicaliza o diretório base e junta o caminho solicitado. Se o alvo não existir e `allow_missing` for verdadeiro, valida o diretório pai (deve estar dentro da raiz) e devolve o caminho do arquivo ainda não criado. Se existir, exige arquivo regular (sem symlink) e confirma, após canonicalizar, que o resultado está contido na raiz — caso contrário retorna `Err(String)`. Erros do sistema de arquivos são convertidos em mensagens com o prefixo `EDIT`.

## Importações
- `std::fs`: Inspeciona o alvo via `symlink_metadata` sem seguir symlinks.
- `std::path::Path`: Recebe o diretório base e monta/canonicaliza o caminho.
- `std::path::PathBuf`: Tipo de retorno com o caminho final validado.
