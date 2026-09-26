## Resumo
Constrói a string de comando shell para uma `Invocation`, aplicando escape via `shell_quote`.

## Funcionamento
Empurra o prefixo `exec --`, depois o programa e cada argumento, quoting cada um com o ambiente da invocação, e une tudo com espaços num único `String`. Não há validação de entradas; a segurança depende inteiramente do quoting.

## Importações
- `super::BashService`: impl de onde o método é associado.
- `crate::services::Invocation`: fornece programa, argumentos e ambiente.
- `crate::services::quote::shell_quote`: escapa cada valor conforme o shell do ambiente.
