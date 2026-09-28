## Resumo
O arquivo declara submódulos de testes do adaptador OpenCode.

## Funcionamento
Não há função principal; o arquivo importa símbolos do módulo pai e tipos de suporte, e declara dois módulos de testes.

## Importações
- `super::*`: Símbolos do módulo pai.
- `crate::harness`: Tipos usados pelos testes do adaptador.
- `std::path::PathBuf`: Tipo para representar caminhos.
