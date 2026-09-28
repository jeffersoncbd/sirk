## Resumo
Executa uma conversa com um agente carregado para o diretório indicado.

## Funcionamento
Converte o diretório em caminho canônico, carrega o agente de `.agents` e verifica se seu adaptador é conhecido. Cria o histórico da conversa e delega a execução; erros de caminho, agente, adaptador ou conversa são retornados como `String`.

## Importações
- `adapters`: valida o adaptador do agente.
- `agents::Agent`: carrega a definição do agente.
- `history::{History, Snapshot}`: cria o histórico da conversa.
- `std::path::Path`: representa o diretório recebido.
