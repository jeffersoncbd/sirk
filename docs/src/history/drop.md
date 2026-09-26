## Resumo
Libera explicitamente o bloqueio associado ao `History` quando ele é descartado.

## Funcionamento
No `Drop`, o método `drop(&mut self)` chama `self._lock.unlock()` e ignora seu resultado (`let _ = ...`), garantindo a liberação do bloqueio mesmo que um processo filho tenha herdado brevemente o descritor de arquivo.

## Importações
- `super::History`: Tipo cujos recursos são liberados no `Drop`.
