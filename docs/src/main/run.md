## Resumo
Interpreta os argumentos de linha de comando e delega a execução ao módulo correspondente (criar agente, rodar, retomar ou exibir ajuda).

## Funcionamento
`run` faz um `match` sobre o vetor de argumentos: sem argumentos exibe a mensagem de uso; `--newAgent`/`--new-agent` cria um agente; `run <nome>` executa o workflow indicado; `resume <caminho>` retoma a execução a partir de um `Path`; `help`/`--help`/`-h` mostra o uso. Qualquer outra combinação retorna `Err` com o texto de comando inválido seguido da string de uso.

## Importações
- `super::{create_agent, print_usage, resume, run_workflow, usage}`: Funções dos módulos irmãos que executam cada subcomando.
- `std::path::Path`: Converte o argumento textual de `resume` em caminho do sistema de arquivos.
