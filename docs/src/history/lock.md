## Resumo
Abre (criando se necessário) o arquivo de lock `*.log.lock` do histórico e adquire bloqueio exclusivo, sinalizando erro via `String`.

## Funcionamento
`lock` constrói um `OpenOptions` com `create(true)`, `truncate(false)` e leitura/escrita, evitando apagar um lock já existente, e abre o caminho obtido ao trocar a extensão do arquivo original para `log.lock`. Falhas de abertura viram `Err(String)` com a mensagem do erro de IO. Em seguida, `try_lock` tenta o bloqueio; se falhar (outro processo usando ou permissão insuficiente), retorna mensagem contextualizada. O sucesso devolve o `File` mantido aberto, cuja posse garante exclusividade mútua.

## Importações
- `super::History`: Método associado para localizar o lock do histórico.
- `std::fs::File`: Handle do arquivo de lock retornado em sucesso.
- `std::fs::OpenOptions`: Cria/abre o lock sem truncar conteúdo existente.
- `std::path::Path`: Recebe o caminho base e deriva o caminho `.log.lock`.
