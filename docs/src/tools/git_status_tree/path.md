## Resumo
Converte bytes crus de um caminho do Git em `PathBuf`, validando a codificação em plataformas não-Unix.

## Funcionamento
Em sistemas Unix, os bytes são reinterpretados diretamente via `OsStrExt` como `OsStr`, preservando qualquer sequência arbitrária sem falha. Em outras plataformas (Windows), os bytes devem ser UTF-8 válidos: a conversão é feita com `str::from_utf8` e, em caso de erro, retorna `Err` com uma mensagem descritiva indicando que o caminho não é suportado. Em ambos os casos o retorno é `Result<PathBuf, String>`; a função é `pub(super)`, restrita ao módulo pai `git_status_tree`, e não possui efeitos colaterais.

## Importações
- `std::path::PathBuf`: Tipo de retorno que representa o caminho do sistema de arquivos.
- `std::os::unix::ffi::OsStrExt`: Trait Unix que converte `&[u8]` em `&OsStr` sem decodificação (somente Unix).
- `std::ffi::OsStr`: Base do caminho; recebe os bytes via `from_bytes` em Unix.
- `std::str::from_utf8`: Valida UTF-8 antes de criar o `PathBuf` em plataformas não-Unix.
