## Resumo
Verifica que a invocação do OpenCode não habilita aprovação automática de permissões.

## Funcionamento
Cria uma solicitação de execução e obtém a invocação padrão do adaptador. Confirma que a lista de argumentos não contém `--auto`; a criação da invocação falha com `unwrap` se retornar erro.

## Importações
- `super::*`: Disponibiliza os tipos e módulos usados no teste.
- `#[test]`: Marca a função como teste.
