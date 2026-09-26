## Resumo
Fornece a implementação padrão do `CodexAdapter`, delegando ao construtor com o nome "codex".

## Funcionamento
O `Default` chama `CodexAdapter::new("codex")`, garantindo que uma instância sem argumentos use o identificador padrão. Não há validações, tratamento de erros nem efeitos colaterais explícitos.

## Importações
- `super::CodexAdapter`: Tipo ao qual a implementação `Default` pertence.
