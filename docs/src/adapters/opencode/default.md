## Resumo
Fornece a implementação padrão do trait `Default` para `OpenCodeAdapter`, construindo-o com o identificador "opencode".

## Funcionamento
O método `default()` delega para `Self::new`, passando a string "opencode", de modo que o adapter passa a ser criado sem argumentos explícitos pelo consumidor. Não há validações, tratamento de erros (`Result`/`Option`) nem efeitos colaterais: é apenas um atalho para o construtor com os valores fixos esperados.

## Importações
- `super::OpenCodeAdapter`: Tipo alvo da implementação do trait `Default`.
