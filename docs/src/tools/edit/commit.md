## Resumo
Aplica uma edição preparada em `Pending::commit`, gravando o resultado no alvo de forma atômica e reportando conflito se o arquivo mudar antes da publicação.

## Funcionamento
Valida a requisição, reaplica a transformação sobre o conteúdo original e compara com o disco: se o conteúdo atual já for o esperado, apenas sincroniza arquivo e diretório e retorna o diff. Se divergir de ambos os estados (original e resultado), aborta com erro de conflito. A gravação ocorre num arquivo temporário único (PID + contador atômico) no mesmo diretório, copiando permissões, escrevendo, sincronizando e revalidando o alvo imediatamente antes da publicação. Arquivos antes inexistentes são publicados via hard link (sem sobrescrever alvo concorrente); existentes, via rename. Qualquer falha remove o temporário, e ao final retorna o diff.

## Importações
- `Pending`: struct que carrega requisição, conteúdo anterior e estado de preparação.
- `read_optional`: lê o alvo tratando ausência como `None`.
- `sync_parent`: força sincronização do diretório pai após a publicação.
- `target`: resolve o caminho alvo conforme o diretório e a política de arquivo ausente.
- `fs`: operações de permissões, hard link, rename e remoção.
- `File`: reabre o alvo para `sync_all` no caminho idempotente.
- `OpenOptions`: cria o temporário com `create_new` evitando colisão.
- `Write`: `write_all` do novo conteúdo no temporário.
- `Path`: tipo do diretório base da operação.
- `AtomicU64`: contador de nomes temporários únicos por processo.
- `Ordering`: modo `Relaxed` no incremento do contador.
- `super::target::target`: resolução repetida do alvo para detectar mudanças.
