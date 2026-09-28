## Resumo
Escolhe e retorna um adaptador disponível informado pelo usuário.

## Funcionamento
Apresenta as opções disponíveis e solicita uma resposta. Remove espaços e converte a entrada para minúsculas; se corresponder a um adaptador disponível, retorna `Ok` com seu nome. Caso contrário, solicita novamente com uma mensagem de erro. Propaga falhas de `input.ask` como `Err`.

## Importações
- `crate::adapters`: Fornece a lista de adaptadores disponíveis.
- `crate::input::UserInput`: Permite solicitar a escolha ao usuário.
