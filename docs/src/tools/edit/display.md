## Resumo
Exibe o diff renderizado no stdout, com cores apenas quando a saída é um terminal real.

## Funcionamento
Verifica se `stdout` é um TTY e se a variável `NO_COLOR` não está definida para decidir o uso de cores; em seguida delega a formatação para `super::render::render` e imprime o resultado com `print!`. Não retorna `Result` nem `Option` — falhas de escrita são ignoradas silenciosamente.

## Importações
- `std::io::IsTerminal`: Trait que permite detectar se stdout está associado a um terminal.
