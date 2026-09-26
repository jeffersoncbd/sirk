## Resumo
Agrega os submódulos de parsing e recuperação das ferramentas externas (EDIT/DELETE) e define a estrutura `ExternalDeleteRequest` com as mensagens de erro padronizadas.

## Funcionamento
O arquivo funciona apenas como ponto de entrada do diretório `external/`: declara cada submódulo com `#[path]` e reexporta suas funções com visibilidade `pub(super)`, além de expor as constantes `DUPLICATE_EDIT_RESULT`, `EDIT_FAILURE_PREFIX`, `DUPLICATE_DELETE_RESULT` e `DELETE_FAILURE_PREFIX` usadas para sinalizar requisições duplicadas e falhas. `ExternalDeleteRequest` representa o payload de deleção com `deny_unknown_fields` (campos extras são rejeitados) e `force` com valor padrão `false` via `#[serde(default)]`, garantindo erro na desserialização de JSON inesperado.

## Importações
- `completed_delete`: Detecta e normaliza resultados de deleções já concluídas.
- `completed_edit`: Processa respostas de editions finalizadas anteriormente.
- `delete_pending`: Trata estados de deleção em andamento ou recuperáveis.
- `delete_request`: Converte requisições brute de deleção em `ExternalDeleteRequest`.
- `edit_pending`: Recupera edições pendentes após falhas de execução.
- `edit_request`: Interpreta e valida o payload de edição do agente.
- `prepare_edit`: Prepara o arquivo alvo antes de aplicar a edição.
- `tool_result`: Converte saída de ferramenta em resposta estruturada ao agente.
- `user_tool_request`: Lê a requisição de ferramenta enviada pelo usuário.
- `validate_user_tool_result`: Valida o resultado devolvido pela ferramenta do usuário.
- `serde`: Deriva `Serialize`/`Deserialize` e configura regras de desserialização.
