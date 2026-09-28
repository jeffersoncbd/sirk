## Resumo
Formata erros de `HarnessError` como mensagens legíveis.

## Funcionamento
Escolhe uma mensagem conforme a variante do erro, incluindo o adaptador e os detalhes disponíveis, e a escreve no formatador.

## Importações
- `super::HarnessError`: Tipo de erro formatado.
- `std::fmt`: Fornece a implementação de formatação.
