## Resumo
Cria e salva um novo histórico associado ao snapshot.

## Funcionamento
Cria a pasta `history`, gera um nome de arquivo com o horário e o PID do processo, adquire um bloqueio, inicializa o histórico vazio e o salva. Erros de criação, obtenção do horário, bloqueio ou salvamento são convertidos em `String`.

## Importações
- `super`: Tipos `History` e `Snapshot`
- `std::fs`: Criação da pasta de histórico
- `std::time`: Geração do horário do arquivo
