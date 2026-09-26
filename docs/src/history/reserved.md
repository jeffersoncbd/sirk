## Resumo
Verifica se uma linha do histórico corresponde a um marcador de seção reservada do log.

## Funcionamento
A função `reserved` retorna `true` para linhas que coincidem exatamente com marcadores conhecidos (`==> ASK`, `==> INPUT`, `<== OUTPUT`, `==> TREE`, `==> READ`, `==> EDIT`, `==> DELETE` ou a constante `SEPARATOR` importada do módulo pai), via `matches!`. Também considera reservadas as linhas iniciadas por `"Step "` ou pelo caractere `'\'` (escapes). Não consome a string (recebe `&str`), não aloca e não tem efeitos colaterais; qualquer outra linha retorna `false`.

## Importações
- `super::SEPARATOR`: Constante de separador de seções definida no módulo pai do histórico.
