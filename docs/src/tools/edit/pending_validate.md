## Resumo
Valida um registro de edição pendente, garantindo que a operação de criação só exista para arquivos ausentes e aplicável a String/Array.

## Funcionamento
`validate` delega a validação estrutural ao `request` e então impõe a regra de criação: se `was_missing` estiver ativo, o conteúdo anterior precisa ser vazio e a operação precisa ser `Append` ou `Prepend`; caso contrário retorna `Err`. Se houver `before`, reaplica a operação sobre ele para confirmar que a transformação é executável. Sem `before`, apenas confirma a validade do pedido. Não altera estado; o erro é um `String` descritivo.

## Importações
- `Operation`: enum das operações de edição usado para filtrar Append/Prepend.
- `Pending`: struct do registro pendente que expõe o método `validate`.
