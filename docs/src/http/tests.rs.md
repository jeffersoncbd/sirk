## Resumo
Organiza os módulos de teste relacionados ao comportamento HTTP.

## Funcionamento
Declara cinco módulos de teste e associa cada um ao arquivo correspondente em `tests/`; não define uma função principal.

## Importações
- `tests/enforces_agent_delete_permissions.rs`: Testes de permissões para excluir agentes.
- `tests/handles_agent_edits.rs`: Testes de edição de agentes.
- `tests/handles_agent_requests.rs`: Testes de requisições de agentes.
- `tests/preserves_literal_template_input.rs`: Testes de preservação de entradas literais.
- `tests/rejects_invalid_requests.rs`: Testes de rejeição de requisições inválidas.
