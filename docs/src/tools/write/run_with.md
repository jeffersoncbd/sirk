## Resumo
Escreve um arquivo de texto dentro de um diretório de execução, com controle de sobreposição (`force`) e de existence (`skip`), recusando caminhos fora da raiz.

## Funcionamento
Valida as flags (`force` e `skip` são mutuamente exclusivas) e que o caminho não seja vazio. Canonicaliza a raiz, junta o caminho relativo e usa `strip_prefix` para garantir que o alvo fique dentro dela. Percorre os componentes um a um, criando os diretórios intermediários via `ensure_directory` e rejeitando qualquer componente não "normal" (ex.: `..`). Se o alvo já existir, exige arquivo regular, revalida que continua sob a raiz após canonicalizar e então: com `skip` retorna sucesso, sem `force` retorna erro, com `force` prossegue para sobrescrita. Finally abre com `OpenOptions` (`create_new` ou `create`/`truncate`), grava os bytes, sincroniza com `sync_all` e converte qualquer erro de I/O em `String` prefixada com `WRITE`.

## Importações
- `super::directory::ensure_directory`: Cria diretórios pai necessários ao caminho-alvo
- `std::fs`: Inspeção de metadados e verificação de arquivos regulares via `symlink_metadata`
- `std::fs::OpenOptions`: Abertura do arquivo com políticas de criação/truncagem distintas
- `std::io::Write`: Gravação do conteúdo e sincronização do descritor em disco
- `std::path::Component`: Validação de que cada trecho do caminho é um nome normal
- `std::path::Path`: Manipulação de caminhos absolutos, relativos e canonicalizados
