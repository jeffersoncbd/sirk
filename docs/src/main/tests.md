## Resumo
Suite de testes que valida a CLI: rejeição de comandos inválidos, texto de uso, execução sem argumentos e resolução de fluxos em `flows/`.

## Funcionamento
Verifica que `run` propaga erro contendo "invalid command" para argumentos desconhecidos e aceita lista vazia sem prompt interativo. Confere que `usage()` documenta a forma explícita `new-harness run <flow-name>` e não expõe comandos interativos. Valida `workflow_path` convertendo nomes simples em `flows/<nome>.yml` e rejeitando entradas vazias, com extensão ou contendo separadores de caminho.

## Importações
- `super::run::run`: ponto de entrada da CLI testado para erros e execução vazia.
- `super::usage::usage`: fornece o texto de ajuda conferido nos asserts.
- `super::workflow_path::workflow_path`: resolve nomes de fluxo em caminhos sob `flows/`.
- `std::path::Path`: comparação dos caminhos gerados contra o esperado.
