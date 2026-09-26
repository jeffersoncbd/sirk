## Resumo
Lê o conteúdo de um arquivo opcional, devolvendo `None` se ele não existir.

## Funcionamento
A função chama `fs::read_to_string` no caminho recebido e envolve o conteúdo em `Some`. Se a leitura falhar com `ErrorKind::NotFound`, retorna `Ok(None)` tratando a ausência como caso válido. Qualquer outro erro de I/O (permissão, diretório, conteúdo não UTF-8) é convertido em `Err` com mensagem formatada, sinalizando falha de leitura UTF-8. Não há escrita nem outras efeitos colaterais no disco.

## Importações
- `std::fs`: fornece a leitura do arquivo inteiro como texto UTF-8.
- `std::path::Path`: representa o caminho do arquivo a ser lido.
