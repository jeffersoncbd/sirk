## Resumo
Verifica que `run` rejeita comandos inválidos ou removidos.

## Funcionamento
Executa `run` com os argumentos `run documentation`, `resume history/run-old.log` e `rpc`; em cada caso, espera um erro contendo “invalid command”.

## Importações
- `super::*`: Importa `run` do módulo pai.
