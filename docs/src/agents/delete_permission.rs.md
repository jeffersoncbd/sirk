## Resumo
Converte o valor serializado `"allow"` em `true`.

## Funcionamento
Desserializa uma string e retorna `Ok(true)` somente se ela for `"allow"`; qualquer outro valor gera um erro descritivo.

## Importações
- `serde`: desserialização da string e criação do erro customizado
