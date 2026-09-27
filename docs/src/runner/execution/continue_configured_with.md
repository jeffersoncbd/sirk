## Resumo
Executa as etapas do workflow salvo após validar o histórico.

## Funcionamento
Valida o snapshot e os blocos do histórico; em seguida, executa as etapas com o motor, acumulando as saídas em um mapa ordenado. Retorna as saídas ou propaga o erro como `String`.

## Importações
- `validate_blocks`, `validate_snapshot`: Validam o histórico antes da execução.
- `Engine`: Executa as etapas do workflow.
- `History`: Fornece o histórico e o snapshot.
- `UserInput`: Fornece a entrada durante a execução.
- `Invocation`: Define a chamada passada à função de execução.
- `BTreeMap`: Armazena as saídas em ordem pelas chaves.
