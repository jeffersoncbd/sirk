### Resumo

Este arquivo define o modelo de dados e as regras de validação de workflows YAML da aplicação. Ele representa workflows compostos por etapas executadas por agentes, ferramentas, ferramentas personalizadas ou loops, além de renderizar referências a resultados anteriores.

### Funcionamento

`Workflow::from_file` lê um arquivo YAML, desserializa seu conteúdo com `serde_yaml` e valida a estrutura antes de retorná-la.

A validação verifica:

- versão suportada (`version: 1`);
- existência de pelo menos uma etapa;
- uso de exatamente um agente, ferramenta válida ou `custom-tool` por etapa;
- regras específicas para `TREE`, `READ`, `WRITE`, `LOOP` e ferramentas personalizadas;
- uso correto de `path`, `force`, `iter` e `output`;
- referências válidas como `{{ outputs.nome }}` e `{{ loop.nome }}`;
- isolamento das variáveis de loop e proteção de `loop.item`, que é somente leitura;
- ausência de outputs duplicados no escopo global.

As referências de template são substituídas por `render_scoped`. Valores inseridos não são processados recursivamente. Entradas podem ser texto ou listas de strings; listas são renderizadas individualmente e convertidas novamente para JSON.

Loops recebem uma lista JSON de strings. Cada iteração possui um escopo local próprio, contendo `loop.item`, enquanto os outputs externos permanecem disponíveis para leitura.

### Componentes principais

- `Workflow`: representa o workflow completo, com versão e lista de `Step`.
- `Step`: descreve uma etapa, que pode executar um agente, uma ferramenta, uma ferramenta personalizada ou um loop.
- `StepInput`: enumeração que aceita texto ou vetor de strings.
- `StepInput::render`: renderiza templates presentes na entrada.
- `Workflow::from_file`: carrega e valida um workflow YAML.
- `Workflow::validate`: valida a versão, a existência de etapas e todas as regras recursivas.
- `validate_steps`: valida etapas, outputs, referências e escopos de loops.
- `Step::name`: retorna o nome do agente ou ferramenta configurado.
- `Step::is_tool_step`: identifica etapas de ferramenta.
- `Step::render_input` e `Step::render_path`: renderizam entrada e caminho de uma etapa.
- `Step::force`: indica se uma operação `WRITE` pode substituir um arquivo existente.
- `loop_items`: converte a entrada de `LOOP` em `Vec<String>`.
- `loop_target`: identifica outputs direcionados a `loop.nome`.
- `render_input` e `render_scoped`: realizam a substituição de referências a outputs.
- `valid_output_name`: restringe nomes a caracteres ASCII alfanuméricos, `_` e `-`.

Os testes verificam validação de loops, isolamento de escopos, referências de output, regras das ferramentas, caminhos de `WRITE` e argumentos de `custom-tool`.

### Dependências e integrações

- `serde`: serialização e desserialização das estruturas e atributos YAML.
- `serde_yaml`: leitura da definição do workflow em YAML.
- `serde_json`: validação de listas de loop e serialização de entradas de `custom-tool`.
- `std::fs` e `std::path::Path`: leitura do arquivo de workflow.
- `BTreeMap`: armazenamento determinístico de outputs globais e variáveis locais.
- `crate::agents::valid_id`: valida nomes de agentes.
- `crate::tools::supports`: verifica ferramentas internas suportadas.
- `crate::tools::custom::valid_name` e `arguments`: validam e processam ferramentas personalizadas.

### Observações

A execução efetiva das etapas não ocorre neste arquivo; ele apenas define, carrega, renderiza e valida workflows. O comportamento concreto de agentes e ferramentas depende dos módulos importados de `crate::agents` e `crate::tools`.
