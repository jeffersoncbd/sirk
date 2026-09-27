## Resumo
Verifica se caminhos de workflow inválidos são rejeitados.

## Funcionamento
Para cada caminho vazio, sem extensão, com travessia de diretório ou aninhado, confirma que `workflow_path` retorna erro.

## Importações
- `super::*`: Importa `workflow_path` do módulo pai.
