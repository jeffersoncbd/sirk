## Resumo

O arquivo `src/runner/tests/conversations.rs` contém testes de integração do executor de workflows, verificando conversas com agentes, retomada de execuções interrompidas, persistência de histórico e interação com as ferramentas `READ` e `TREE`.

## Funcionamento

Os testes criam projetos temporários, arquivos de agentes e workflows YAML. Em seguida, executam workflows por meio de helpers como `run_with`, `run_interactive_with` e `continue_with`, usando callbacks falsos para simular respostas ou falhas do harness.

O arquivo verifica principalmente:

- Montagem correta da invocação do Codex, incluindo `call_prefix`.
- Restrição de `TREE_TOOL` quando o agente não possui permissão.
- Salvamento e recuperação do histórico após falhas.
- Reexecução de etapas cujo output foi removido ou editado.
- Retomada de perguntas interativas sem repetir chamadas desnecessárias.
- Processamento de respostas do usuário contendo `READ` ou `TREE`.
- Persistência dos resultados das ferramentas no histórico.
- Uso de outputs nomeados entre etapas do workflow.
- Preservação literal de templates inseridos como texto.
- Execução de etapas somente com ferramentas, sem agentes.
- Leitura de arquivos com conteúdo preservado, inclusive quebras de linha.
- Validação de que workflows em estados inválidos não sejam executados novamente.

As falhas são tratadas principalmente com `Result`, usando `unwrap`, `unwrap_err`, `assert!` e `assert_eq!` para validar os resultados esperados. Alguns callbacks usam `panic!` para garantir que uma chamada não deveria ocorrer.

## Componentes principais

- `runs_an_agent_through_its_call_prefix`: verifica o prefixo de chamada configurado no agente e os argumentos enviados ao Codex.
- `tree_is_not_advertised_or_executed_without_permission`: confirma que `TREE` não é oferecido nem executado sem permissão.
- `resumes_failed_turn_with_context_and_replays_edited_output`: testa recuperação de falhas, persistência de contexto e substituição de outputs editados.
- `resumes_questions_without_repeating_model_calls`: verifica a retomada de perguntas pendentes sem chamadas redundantes ao agente.
- `user_can_answer_a_question_with_a_read_request_and_resume_after_it`: testa uma resposta do usuário com `READ` e a restauração do resultado lido.
- `user_can_answer_a_question_with_tree_without_agent_permission`: verifica o uso interativo de `TREE` solicitado pelo usuário.
- `rejects_downstream_steps_after_deleted_response`: valida a rejeição de históricos inconsistentes.
- `runs_without_input_and_keeps_inserted_templates_literal`: verifica etapas sem entrada explícita e preservação literal de templates.
- `tool_step_passes_named_output_and_resumes_without_rerunning_tree`: testa outputs produzidos por ferramentas e sua recuperação sem repetir `TREE`.
- `agent_tree_request_and_result_survive_interruptions`: verifica a persistência de solicitações e resultados de `TREE` durante interrupções.
- `tree_only_workflow_needs_no_agents_and_embedded_tree_is_plain_text`: testa workflows compostos apenas por ferramentas e respostas textuais que não devem ser interpretadas automaticamente como comandos.
- `read_step_passes_verbatim_contents_and_restores_saved_result`: garante que `READ` preserve integralmente o conteúdo do arquivo e o resultado salvo.

O arquivo importa `super::*`, reutilizando os tipos, funções auxiliares e módulos definidos no módulo de testes pai, incluindo `Project`, `Workflow`, `History`, `Block`, `run_with`, `run_interactive_with` e `continue_with`.

## integrações

O arquivo não define novas APIs públicas. Ele interage diretamente com:

- A representação de workflows e suas etapas `agent` e `tool`.
- O histórico de execução e seus blocos de entrada, saída, leitura e árvore.
- Os adaptadores de harness, simulados por callbacks de invocação.
- O sistema de ferramentas `READ` e `TREE`.
- Arquivos de configuração em `.agents/*.md`.
- Arquivos YAML usados para desserializar workflows.

A implementação exata das estruturas e helpers importados não aparece no conteúdo fornecido; portanto, seus detalhes internos não podem ser determinados apenas por este arquivo.
