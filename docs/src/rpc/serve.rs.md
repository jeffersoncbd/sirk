## Resumo
Processa requisições JSON-RPC recebidas pela entrada padrão e responde pela saída padrão.

## Funcionamento
Obtém o diretório atual e lê a entrada linha a linha. Converte cada linha em uma requisição: erros de análise geram código `-32700`, requisições inválidas geram `-32600` e as válidas são encaminhadas a `handle`. Serializa cada resposta, escreve uma quebra de linha e descarrega a saída; erros de diretório, leitura ou escrita são retornados como `String`.

## Importações
- `super::handle::handle`: Processa requisições válidas.
- `super::types::{Request, Response, RpcError}`: Tipos RPC usados.
- `serde_json::Value`: Representa valores JSON e identificadores.
- `std::io`: Lê a entrada e escreve as respostas.
