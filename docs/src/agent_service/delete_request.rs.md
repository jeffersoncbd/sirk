## Resumo
Processa uma solicitação de exclusão de arquivo, respeitando a permissão para execução forçada e evitando repetições.

## Funcionamento
Interpreta o payload como JSON e rejeita campos desconhecidos. Se `force` não estiver autorizado, retorna uma mensagem de falha; se a mesma solicitação já foi concluída, informa que não houve alterações. Caso contrário, registra e salva a solicitação no histórico. Sem `force`, retorna erro por falta de entrada do usuário; com `force`, tenta excluir o arquivo e retorna sucesso vazio ou uma mensagem de falha.

## Importações
- `crate::history::{Block, History}`: Acessa e atualiza o histórico.
- `serde::Deserialize`: Permite interpretar a solicitação JSON.
