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
- `validate_steps`: executa a validação recursiva de etapas, incluindo branches de `IF` e corpo de `LOOP`.
- `Step::render_input` e `Step::render_path`: renderizam templates presentes na entrada ou no caminho.
- `Step::edit_request`: cria uma requisição para `EDIT`, validando coordenadas, operação, versão e conteúdo.
- `loop_items`: interpreta a entrada de `LOOP` como um array JSON de strings.
- `condition`: aceita somente os valores booleanos textuais `true` ou `false`.
- `loop_target`: identifica outputs destinados a variáveis locais de loop.
- `render_scoped` e `render_input`: substituem referências a outputs e variáveis de loop.
- `valid_output_name`: restringe nomes de outputs a letras, números, `_` e `-`.

Os testes cobrem validação de branches, escopos de loop, operações de edição, outputs, referências, ferramentas, escrita de arquivos e ferramentas customizadas.

### Dependências e integrações

- `serde` e `serde_yaml`: desserialização e serialização dos workflows.
- `std::fs` e `std::path::Path`: leitura de workflows do sistema de arquivos.
- `BTreeMap` e `BTreeSet`: armazenamento determinístico de outputs, variáveis locais e nomes declarados.
- `crate::agents::valid_id`: valida agentes conhecidos.
- `crate::tools::supports`: valida ferramentas internas suportadas.
- `crate::tools::custom::valid_name` e `arguments`: validam e processam ferramentas customizadas.
- `crate::tools::edit::{Operation, Request}`: representam operações e requisições de edição.

O arquivo fornece os tipos e métodos usados pelo restante do projeto para interpretar, validar e preparar workflows para execução.

### Observações

A execução efetiva dos agentes e ferramentas não está implementada neste arquivo; ele apenas modela, valida e renderiza os dados necessários. A resolução de templates depende de outputs já disponíveis e falha com `Result<String, String>` quando uma referência é inválida, está fora de escopo ou não foi declarada.
