## Resumo

`src/runner/history_validation.rs` valida estruturalmente transcripts persistidos de workflows. Ele verifica se os blocos registrados correspondem à sequência de steps, loops, condicionais, ferramentas, agentes e interações esperadas, identificando históricos incompletos ou inválidos.

## Funcionamento

A validação começa em `validate_blocks`, que percorre recursivamente os steps do workflow e mantém um cursor sobre os registros de `history.steps`.

Para cada step, o código:

- Confere se o label persistido corresponde ao step esperado.
- Valida recursivamente corpos de `LOOP` e branches de `IF`.
- Aplica regras específicas para `EDIT` e `ASK`.
- Valida conversas de agentes, incluindo ferramentas externas e permissões.
- Valida steps comuns como sequências de blocos `Ask`, `Input`, `Output` e resultados de ferramentas.

O retorno `Ok(false)` representa um histórico ainda incompleto, enquanto erros (`Err`) indicam inconsistências estruturais ou dados inválidos que exigem remoção do resultado editado e dos registros posteriores.

A validação também verifica:

- JSON serializado de operações pendentes de edição ou exclusão.
- Correspondência entre requisições de ferramentas e seus resultados.
- Permissões para `EDIT_TOOL`, `DELETE_TOOL` e exclusão forçada.
- Resultados duplicados ou falhos de operações externas.
- Perguntas não vazias e respostas de usuário.
- Ausência de blocos inesperados, como `Delete` em conversas comuns.

## Componentes principais

- `validate_blocks`: ponto de entrada da validação. Percorre o workflow, valida labels e delega a validação conforme o tipo de step.
- `visit`: função interna recursiva usada para atravessar steps normais, loops e condicionais, mantendo o índice atual do histórico.
- `validate_edit_blocks`: valida o input JSON de uma operação `EDIT` e confirma que o diff persistido corresponde ao diff calculado.
- `validate_ask_blocks`: valida os formatos permitidos para um step `ASK`, incluindo pergunta pendente ou já respondida.
- `validate_step_blocks`: verifica a alternância entre perguntas, inputs, outputs e resultados de ferramentas internas como `TREE`, `READ` e `EDIT`.
- `validate_agent_blocks`: valida uma conversa completa com um agente, incluindo:
  - requisições e resultados de `TREE` e `READ`;
  - operações `EDIT_TOOL`;
  - operações `DELETE_TOOL`;
  - perguntas feitas pelo agente;
  - permissões configuradas para edição e exclusão.
- `Block` e `History`: estruturas importadas de `crate::history`, usadas para representar blocos persistidos e o transcript completo.
- `Step`, `condition` e `loop_items`: componentes do workflow usados para identificar steps, selecionar branches e obter iterações de loops.
- `external::*`: tipos, constantes e funções auxiliares para requisições e resultados de edição/exclusão externas.
- `question`: auxiliar que identifica perguntas produzidas por uma resposta.
- `serde_json`: usado para desserializar e validar operações pendentes armazenadas como JSON.

## integrações

O arquivo expõe `validate_blocks` com visibilidade `pub(super)`, portanto disponível ao módulo pai dentro da crate. As demais funções são privadas ao módulo.

A implementação depende de `History`, `Block`, da definição de `Step`, dos validadores de ferramentas em `crate::tools` e das estruturas auxiliares importadas de `external`. O comportamento exato desses componentes externos não pode ser determinado apenas por este arquivo.
