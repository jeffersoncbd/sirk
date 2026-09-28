## Resumo
Verifica que a invocação do adaptador OpenCode não ativa aprovação automática de permissões.

## Funcionamento
Cria uma solicitação de execução, obtém a invocação do adaptador e confirma que seus argumentos não incluem `--auto`; a criação da invocação propaga erros com `unwrap`.

## Importações
- `super::*`: Acessa os tipos e o adaptador do módulo pai.
