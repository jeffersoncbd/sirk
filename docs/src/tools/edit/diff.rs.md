## Resumo
Gera um diff unificado entre o conteúdo original e o conteúdo editado.

## Funcionamento
Retorna erro se a edição ainda não estiver preparada ou se a aplicação falhar. Caso contrário, cria um diff com três linhas de contexto; usa `/dev/null` como origem para arquivos inexistentes e escapa caracteres de controle no caminho.

## Importações
- `super::Pending`: Tipo cuja edição está pendente.
- `similar`: Gera o diff unificado.
