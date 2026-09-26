## Resumo
Módulo que reexporta os contratos de integração do harness a partir de `interfaces`.

## Funcionamento
Não há lógica: o arquivo serve apenas como fachada pública, reexportando `HarnessAdapter`, `HarnessError`, `Invocation` e `RunRequest` de `crate::interfaces` para preservar compatibilidade de imports em caminhos antigos (`crate::harness::*`). Nenhum erro ou `Option` é tratado aqui, pois o módulo não executa operações.

## Importações
- `crate::interfaces`: Origem dos quatro itens reexportados (adaptador, erro, requisição e execução).
