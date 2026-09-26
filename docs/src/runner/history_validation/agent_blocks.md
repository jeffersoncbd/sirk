## Resumo
Valida a sequência de blocos de um turno do agente, garantindo que pedidos de ferramentas e suas respostas (READ/TREE, EDIT_TOOL, DELETE_TOOL, ASK) estejam coerentes e completos.

## Funcionamento
A função percorre `blocks` com um cursor `position`. O primeiro bloco deve ser `Ask` não vazio (turno iniciado por pergunta) ou `Input`; um segundo `Ask` seguido é inválido. A partir daí, todo bloco do agente precisa ser `Output`. Se a saída contém um pedido de leitura, exige o bloco `Tree`/`Read` correspondente; se é edição ou deleção externa, verifica permissões (`edit_tool`, `delete_tool`, `delete_without_confirm` para deleção forçada), aceita resultados duplicados (validando que a operação já foi concluída antes) ou falhas, e nos casos pendentes exige um `Input` com o payload desserializado que deve bater com o diff/pedido antes do resultado. Perguntas do agente exigem `Input` de resposta não vazia. `Ok(false)` indica histórico incompleto (aguardando input do usuário), enquanto `Err` sinaliza inconsistência; `Ok(true)` apenas quando a conversa começou com `Ask` e terminou de forma válida.

## Importações
- `Block`: Variante do histórico (Ask, Input, Output, Tree, Read, Edit, Delete) analisada.
- `external::*`: Detecta e valida pedidos/completude de edição e deleção de arquivos externos.
- `question`: Extrai a pergunta ASK da saída do agente.
- `serde_json`: Desserializa payloads pendentes de EDIT_TOOL/DELETE_TOOL.
- `crate::tools::request`: Identifica o tipo de ferramenta de leitura solicitada.
- `crate::tools::edit::Pending`: Representa a edição pendente e valida o diff esperado.
