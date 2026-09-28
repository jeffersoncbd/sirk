## Resumo
Verifica que `run` rejeita comandos de workflow removidos.

## Funcionamento
Executa `run` com os comandos `run documentation` e `resume history/run-old.log`; em ambos os casos, espera um erro contendo “invalid command”.

## Importações
- `super::*`: Importa itens do módulo pai, incluindo `run`.
