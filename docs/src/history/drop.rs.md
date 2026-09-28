## Resumo
Libera o bloqueio de `History` ao descartar o valor.

## Funcionamento
Ao ser descartado, tenta desbloquear `_lock` e ignora qualquer erro.

## Importações
- `super::History`: Tipo cujo descarte libera o bloqueio.
