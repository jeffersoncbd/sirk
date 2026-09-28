## Resumo
Converte bytes em um caminho `PathBuf`, respeitando as capacidades da plataforma.

## Funcionamento
No Unix, preserva os bytes do caminho diretamente. Em outras plataformas, exige UTF-8 e retorna uma mensagem de erro se a conversão falhar.

## Importações
- `std::path::PathBuf`: Representa o caminho convertido.
- `std::os::unix::ffi::OsStrExt`: Converte bytes em `OsStr` no Unix.
- `std::str`: Valida UTF-8 em plataformas não Unix.
- `std::ffi::OsStr`: Cria `OsStr` a partir dos bytes no Unix.
- `format!`: Monta a mensagem de erro.
