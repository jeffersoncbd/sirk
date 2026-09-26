## Resumo
Verifica se um arquivo sob `root` é ignorado pela configuração `.readignore`.

## Funcionamento
Lê `root/.readignore`; se ausente, retorna `Ok(false)` (nada é ignorado), e qualquer outro erro de leitura vira `Err("AccessDenied")`. O caminho do arquivo é convertido em caminho relativo à raiz (normalizando `\` para `/`) e comparado, linha a linha, com o matcher de `.gitignore`, ignorando linhas vazias e comentários (`#`).

## Importações
- `std::fs`: lê o conteúdo do arquivo `.readignore`
- `std::io`: identifica `ErrorKind::NotFound` para tratar ausência como falso
- `std::path::Path`: tipos de entrada e relativização do caminho
