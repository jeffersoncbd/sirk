## Resumo
Valida que uma coordenada seja maior que zero, devolvendo o valor ou um erro descritivo.

## Funcionamento
A função recebe o nome da coordenada e seu valor. Se o valor for `0`, retorna `Err` com a mensagem gerada por `coordinate_error(name)`; caso contrário, repassa o valor unchanged em `Ok`. O parâmetro `name` existe apenas para compor a mensagem de erro, enquanto `value` é devolvido sem transformação.

## Importações
- `super::coordinate_error::coordinate_error`: Monta a mensagem de erro usando o nome da coordenada
