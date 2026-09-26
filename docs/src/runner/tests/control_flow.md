## Resumo

O arquivo `src/runner/tests/control_flow.rs` contém testes de integração para o controle de fluxo de workflows Rust, especialmente os steps `IF`, `LOOP` e `WRITE`, além da recuperação de execuções interrompidas por meio do histórico.

## Funcionamento

Os testes criam projetos temporários, workflows YAML e arquivos locais para verificar o comportamento do runner. Eles validam:

- Reutilização de arquivos existentes ou geração de documentação por agentes.
- Seleção correta dos ramos `is_true` e `is_false` de `IF`.
- Validação antecipada de agentes inválidos, inclusive em ramos que não serão executados.
- Persistência e retomada de execução usando `History`.
- Restauração de escopos locais, como `loop.item`, `loop.content` e resultados produzidos dentro de iterações.
- Execução de loops aninhados e loops vazios.
- Validação de que a entrada de `LOOP` seja um array de strings.
- Detecção de históricos inconsistentes ou modificados manualmente.
- Criação de arquivos por `WRITE` e prevenção da repetição desse step após a retomada.
- Preservação de resultados externos durante a continuação de uma execução.

Os testes usam callbacks falsos para simular chamadas de agentes. Isso permite verificar prompts, quantidade de chamadas, interrupções e retomadas sem depender de um modelo real. Erros são tratados com `Result`, usando `unwrap`, `unwrap_err` e asserções para confirmar falhas esperadas.

## Componentes principais

- `if_reuses_docs_or_generates_them_and_restores_loop_outputs`: verifica leitura de documentação existente, geração condicional por agente, escrita de novos arquivos e restauração dos resultados de um loop.
- `if_recovers_pending_nested_branch_and_rejects_edited_conditions`: testa recuperação de uma execução pendente dentro de um ramo aninhado e rejeita alterações na condição registrada no histórico.
- `if_validates_unselected_agents_and_handles_empty_selection`: garante a validação de todos os agentes, mesmo os não selecionados, e o comportamento de um ramo vazio.
- `write_step_creates_a_file_and_is_not_repeated_after_resume`: valida a criação de arquivos por `WRITE` e que o arquivo não seja recriado durante uma retomada concluída.
- `loop_reads_items_with_local_outputs_and_resumes_at_pending_iteration`: testa leitura de itens, outputs locais por iteração e retomada a partir da iteração pendente.
- `nested_and_empty_loops_restore_outer_scope`: verifica loops aninhados, loops vazios e restauração das variáveis do loop externo.
- `loop_is_not_an_agent_tool_and_checks_nested_agents_early`: confirma que `LOOP` não é tratado como ferramenta de agente e que agentes internos são validados previamente.
- `loop_rejects_non_array_output_before_running_body`: valida a rejeição de resultados que não sejam arrays de strings antes da execução do corpo do loop.
- `loop_refuses_later_records_after_an_edited_pending_child`: detecta registros posteriores inválidos quando um resultado pendente foi removido do histórico.
- `plain_loop_outputs_are_local_and_restored_on_resume`: testa a separação entre outputs globais e locais de cada iteração.
- `validates_all_agents_before_input_or_execution`: confirma que agentes inválidos impedem a execução antes de processar entradas ou criar histórico.

O arquivo importa `super::*`, reutilizando os tipos, funções e utilitários definidos no módulo de testes pai, como `Project`, `Workflow`, `History`, `Block`, `run_with`, `continue_with`, `run_interactive_with` e `answers`. Também usa operações de filesystem e estruturas como `BTreeMap`.

## integrações

Não há funções, structs, enums ou traits públicas definidos diretamente neste arquivo. Ele utiliza APIs internas do runner e os mecanismos de persistência de histórico, execução de workflows, filesystem e callbacks de harness.

A análise do comportamento completo depende das implementações importadas de `super::*`, especialmente do runner, de `History`, dos tipos de workflow e dos utilitários de teste.
