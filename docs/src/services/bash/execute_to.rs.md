## Resumo
Executa uma invocação, captura stdout e o envia a um destino.

## Funcionamento
Delegа a execução e propaga erros; converte stdout para UTF-8, retornando erro se inválido, e preserva o status.

## Importações
- `super`: tipos `BashService` e `ProcessOutput`.
- `Invocation`: dados da execução.
- `std::io`: erros e escrita do stdout.
