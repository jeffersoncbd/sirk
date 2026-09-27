## Resumo
Atende requisições RPC recebidas pela entrada padrão e envia respostas pela saída padrão.

## Funcionamento
Obtém o diretório atual e processa cada linha como JSON. Linhas inválidas geram erro de parsing; JSON que não corresponde a uma requisição gera erro de requisição; as demais são encaminhadas a `handle`. Cada resposta é serializada, seguida de uma quebra de linha e enviada imediatamente. Erros de diretório ou de entrada e saída são retornados como `String`; ao chegar ao fim da entrada, retorna `Ok(())`.

## Importações
- `super::handle`: Processa requisições RPC válidas.
- `super::types`: Fornece os tipos de requisição, resposta e erro.
- `serde_json`: Faz o parsing e a serialização JSON.
- `std::io`: Lê linhas da entrada e escreve respostas na saída.
