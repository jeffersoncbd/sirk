## Resumo

`src/runner/external.rs` reúne funções auxiliares para interpretar solicitações de ferramentas externas feitas por um agente, validar seus resultados e preparar operações de edição ou exclusão de arquivos antes da execução.

## Funcionamento

O arquivo analisa blocos de histórico (`Block`) para identificar:

- solicitações de ferramentas `READ` e `TREE`;
- resultados correspondentes a essas solicitações;
- pedidos `EDIT:` e `DELETE:` em formato JSON;
- operações já concluídas, evitando repetições.

Solicitações de edição são desserializadas em `crate::tools::edit::Request`. O código rejeita o campo `version` vindo externamente, lê o arquivo alvo, valida limites de linha, calcula uma versão do conteúdo atual e cria um `Pending` para a operação.

Solicitações de exclusão usam uma estrutura própria, `ExternalDeleteRequest`, com caminho e opção `force`. O histórico pendente é comparado com a solicitação original para garantir consistência.

Os erros são propagados por `Result<String>` ou convertidos em mensagens textuais. Não há `panic!`, `unsafe`, concorrência ou execução direta de processos neste arquivo.

## Componentes principais

- `user_tool_request`: identifica, no histórico, uma resposta do usuário que corresponde a uma solicitação `READ` ou `TREE`.
- `tool_result`: converte o resultado textual de `TREE` ou `READ` no respectivo `Block`.
- `validate_user_tool_result`: verifica se o próximo bloco do histórico corresponde ao tipo de ferramenta solicitado.
- `DUPLICATE_EDIT_RESULT` e `DUPLICATE_DELETE_RESULT`: mensagens para operações repetidas.
- `EDIT_FAILURE_PREFIX` e `DELETE_FAILURE_PREFIX`: prefixos padronizados para erros.
- `ExternalDeleteRequest`: representa uma solicitação externa de exclusão, com `path` e `force`.
- `external_edit_request`: reconhece o prefixo `EDIT:`, desserializa o JSON e rejeita versões fornecidas externamente.
- `prepare_external_edit`: lê o arquivo, valida as linhas solicitadas, calcula sua versão e cria uma edição pendente.
- `external_edit_pending`: recupera e desserializa uma edição pendente armazenada no histórico.
- `completed_external_edit`: verifica se uma edição já aparece como concluída no histórico.
- `external_delete_request`: reconhece e desserializa solicitações com prefixo `DELETE:`.
- `external_delete_pending`: valida e compara o pedido de exclusão com seu registro pendente.
- `completed_external_delete`: verifica se uma exclusão já foi registrada como concluída.

## integrações

O arquivo importa `Block` do módulo de histórico, `question` do bootstrap e utiliza `serde` para serialização e desserialização JSON.

Os símbolos públicos para o restante da crate são as constantes `DUPLICATE_EDIT_RESULT` e `EDIT_FAILURE_PREFIX`. As demais estruturas e funções são `pub(super)`, ficando acessíveis apenas ao módulo pai e seus módulos relacionados.
