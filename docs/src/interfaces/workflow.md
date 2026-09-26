## Resumo
Define as estruturas de dados (serializáveis) que descrevem um workflow e seus passos.

## Funcionamento
O arquivo apenas declara tipos: `Workflow` (versão e lista de `Step`), `Step` (configuração opcional de agent, tool, custom-tool, input, edição com coordenadas, flags, e listas aninhadas `iter`/`is_true`/`is_false`) e os enums `EditCoordinate` (número ou template) e `StepInput` (texto, array ou booleano). Não há lógica de execução: o papel é purely estrutural, com anotações `serde` (`deny_unknown_fields`, `default`, `rename`, `skip_serializing_if`) que definem tolerância a campos ausentes, rejeição de campos extras e o formato `kebab-case` na serialização. Um módulo `default` é declarado para valores padrão.

## Importações
- `serde::Serialize`: Habilita a serialização das structs e enums para JSON/YAML.
- `serde::Deserialize`: Habilita a desserialização validando campos desconhecidos.
