## Resumo
Disponibiliza internamente `run` como ponto de entrada do serviço de execução de agentes.

## Funcionamento
Declara os módulos `execute` e `run` e reexporta `run::run` para uso dentro do crate.

## Importações
- `run`: Fornece a função reexportada.
- `execute`: Declara o módulo de execução.
