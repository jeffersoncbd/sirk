## Resumo
Carrega um `Workflow` a partir de um arquivo YAML, desserializando e validando seu conteúdo.

## Funcionamento
Lê o arquivo indicado por `path` como texto e converte a string em `Workflow` via desserialização YAML. Erros de leitura e de parse são mapeados para `String` com mensagens contextualizadas pelo caminho do arquivo. Após a desserialização, chama `workflow.validate()?`, propagando seu erro; só então retorna o workflow válido como `Ok`.

## Importações
- `super::Workflow`: Tipo alvo do `impl`, tratado como `Self` na construção.
- `std::fs`: Lê o conteúdo bruto do arquivo em String.
- `std::path::Path`: Recebe o caminho de entrada e formata-o nos erros.
- `serde_yaml::from_str`: Desserializa o texto YAML em um `Workflow`.
