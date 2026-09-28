## Resumo
Verifica que edições por linha preservam bytes e tratam corretamente o fim do arquivo.

## Funcionamento
Testa inserções, exclusões e substituições com quebras de linha variadas, incluindo CRLF, texto UTF-8 e conteúdo sem quebra final. Também confirma que uma linha fora do intervalo retorna erro.

## Importações
- `super::*`: Importa os itens do módulo pai usados pelo teste.
