## Resumo
Define os tipos de dados usados para solicitar e registrar edições de arquivos.

## Funcionamento
`Operation` enumera as operações disponíveis; `Request` reúne o caminho, a operação, os parâmetros e o conteúdo da edição. `Pending` armazena a solicitação e o estado anterior do arquivo, incluindo se ele não existia.

## Importações
- `serde`: Serializa e desserializa os tipos definidos.
