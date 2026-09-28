## Resumo
Prepara e aplica uma solicitação de edição, evitando repetir uma edição já concluída.

## Funcionamento
Interpreta o payload como `Request` e rejeita versões informadas pelo solicitante. Se a mesma edição já aparece no histórico, retorna uma mensagem sem aplicá-la. Caso contrário, lê o arquivo, calcula sua versão, prepara a edição, registra a solicitação no histórico e a salva antes de confirmar a alteração. Erros de leitura ou preparação viram mensagens para corrigir a solicitação; erros de parsing, serialização, salvamento ou confirmação são propagados como `Err`.

## Importações
- `Block`, `History`: histórico e registro da solicitação.
- `Pending`, `Request`: representação e preparação da edição.
- `read`: leitura do arquivo antes da edição.
- `serde_json`: parsing e serialização de JSON.
