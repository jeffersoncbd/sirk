## Resumo
Valida um nome de fluxo e monta o caminho `flows/<nome>.yml`.

## Funcionamento
A função recusa nomes vazios ou que contenham caracteres fora de `[A-Za-z0-9_-]`, retornando `Err` com mensagem descritiva; caso contrário, combina o diretório `flows` com o nome mais a extensão `.yml` e devolve `Ok(PathBuf)`. Sem efeitos colaterais.

## Importações
- `Path`: montagem do caminho base do arquivo de fluxo
- `PathBuf`: retorno do caminho validado
