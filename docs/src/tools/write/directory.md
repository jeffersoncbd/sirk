## Resumo
Garante que um diretório pai exista, criando-o se necessário, ou falha com mensagem contextualizada à operação WRITE.

## Funcionamento
Consulta `fs::symlink_metadata` no diretório alvo: se já existir e for diretório, retorna `Ok(())`; se existir como outro tipo de arquivo, retorna `Err` informando que o caminho não é um diretório. Se os metadados falharem com `NotFound`, tenta `create_dir`; um `AlreadyExists` (corrida com outro processo) dispara nova verificação recursiva, e qualquer outro erro de criação vira `Err` com o erro original. Falhas de inspeção também são convertidas em `Err`, sempre prefixadas por "WRITE" e pelo `path` afetado. Não há escrita parcial: ou o diretório existe ao final, ou um erro é devolvido.

## Importações
- `std::fs`: Consulta metadados e criação do diretório no sistema de arquivos.
- `std::path::Path`: Representa o caminho do diretório pai a ser verificado/criado.
