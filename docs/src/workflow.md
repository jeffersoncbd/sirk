### Resumo

Este arquivo define o modelo e as regras de validação de workflows declarativos em YAML. Ele representa workflows compostos por etapas de agentes, ferramentas, ferramentas customizadas, condicionais `IF`, loops `LOOP` e operações de edição de arquivos.

### Funcionamento

Um `Workflow` possui uma versão e uma lista não vazia de `Step`. O método `from_file` lê o YAML, desserializa com `serde_yaml` e executa a validação completa.

A validação verifica:

- versão suportada do workflow (`1`);
- existência de pelo menos uma etapa;
- uso correto de agentes, ferramentas e `custom-tool`;
- compatibilidade entre campos e ferramentas;
- referências válidas a outputs anteriores;
- escopo de variáveis de loop;
- regras específicas de `IF`, `LOOP`, `READ`, `WRITE` e `EDIT`;
- nomes e declarações duplicadas de outputs;
- operações e coordenadas válidas de edição.

Templates no formato `{{ outputs.nome }}` são resolvidos usando outputs já declarados. Dentro de loops, também são permitidas referências como `{{ loop.item }}` e `{{ loop.nome }}`.

Condicionais validam os dois ramos. Após um `IF`, apenas outputs definidos em ambos os ramos ficam disponíveis para as etapas seguintes. Loops criam um escopo local isolado, impedindo que seus outputs escapem para fora da iteração.

### Componentes principais

- `Workflow`: representa o workflow completo, com `version` e `steps`.
- `Step`: representa uma etapa e seus parâmetros, como agente, ferramenta, entrada, caminho, output, branches e iterações.
- `EditCoordinate`: coordenada de edição que pode ser um número ou um template renderizável.
- `StepInput`: entrada que pode ser texto, lista de strings ou booleano.
- `Workflow::from_file`: carrega e valida um workflow a partir de um arquivo.
- `Workflow::validate`: valida a versão, a estrutura e todas as etapas.
- `Step`: representa uma etapa e seus parâmetros, como agente, ferramenta, ferramenta customizada, entrada, caminho, flags `force`/`skip`, operação de edição, versão esperada, outputs, branches e iterações.
- `validate_steps`: executa a validação recursiva de etapas, incluindo branches de `IF` e corpo de `LOOP`.
- `Step::render_input` e `Step::render_path`: renderizam templates presentes na entrada ou no caminho.
- `Step::edit_request`: renderiza os campos e monta uma requisição para `EDIT`; a validação completa da requisição ocorre durante `Workflow::validate`.
- `loop_items`: interpreta a entrada de `LOOP` como um array JSON de strings.
- `condition`: aceita somente os valores booleanos textuais `true` ou `false`.
- `loop_target`: identifica outputs destinados a variáveis locais de loop.
- `render_scoped` e `render_input`: substituem referências a outputs e variáveis de loop.
- `valid_output_name`: restringe nomes de outputs a letras, números, `_` e `-`.
Os testes cobrem validação de branches, escopos de loop, operações de edição, outputs, referências, ferramentas, escrita de arquivos e ferramentas customizadas.

### Integrações

- `serde` e `serde_yaml`: desserialização e serialização dos workflows, com `serde(deny_unknown_fields)` rejeitando campos YAML desconhecidos.
- `std::fs` e `std::path::Path`: leitura do arquivo de workflow.
- `BTreeMap` e `BTreeSet`: armazenamento determinístico de outputs, variáveis locais e nomes declarados.
- `crate::agents::valid_id`: validação de agentes conhecidos.
- `crate::tools::supports`: validação de ferramentas internas suportadas.
- `crate::tools::custom::{valid_name, arguments}`: validação de ferramentas customizadas e interpretação de seus argumentos.
- `crate::tools::edit::{Operation, Request}`: representação e validação das operações de edição.

O arquivo expõe os modelos `Workflow`, `Step`, `EditCoordinate` e `StepInput`, além das funções de validação e renderização usadas para preparar workflows. `Step::edit_request` renderiza os campos e monta um `Request`; durante a validação do workflow, esse request também é submetido às regras de `Request::validate`.

A execução efetiva dos agentes e ferramentas não está implementada neste arquivo. A resolução de templates depende de outputs já disponíveis e falha com `Result<String, String>` quando uma referência é inválida, está fora de escopo ou não foi declarada. Dentro de loops, `loop.item` é somente leitura, e outputs locais permanecem restritos ao corpo da iteração; após um `IF`, apenas nomes definidos nos dois ramos permanecem disponíveis.

As APIs públicas também incluem métodos de consulta e preparação em `Step`, como `branch`, `name`, `is_tool_step`, `force`, `render_input`, `render_path` e `edit_request`, além das funções de renderização e validação de condições e loops.