## Resumo
Exibe na saída padrão o texto de uso gerado por `super::usage::usage()`.

## Funcionamento
Chama `usage()` no módulo pai e imprime o texto retornado via `print!`, sem quebra de linha nem argumentos adicionais. Não há validações, retornos ou efeitos colaterais além da escrita na stdout.

## Importações
- `print!`: Macro padrão do Rust usada para escrever o texto na stdout.
- `super::usage`: Módulo pai que fornece a função `usage()` com a mensagem.
