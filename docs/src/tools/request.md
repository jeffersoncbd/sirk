## Resumo
Interpreta uma mensagem de texto como comando de controle (`TREE` ou `READ: <caminho>`), retornando `None` se não for um comando isolado.

## Funcionamento
A função normaliza o texto com `trim()` e só aceita o comando se a linha inteira for o comando: `TREE` retorna `("TREE", "")`; para leitura, remove o prefixo `"READ: "` (aceitando `"READ:"` sem espaço, com caminho vazio) e descarta qualquer entrada que ainda contenha quebras de linha, o que impede comandos embutidos em blocos maiores (código, cercas Markdown, mensagens com texto adicional). Como todo borrow vem do `&str` de entrada, não há alocação; a ausência de correspondência é sinalizada por `Option`, e nenhuma conversão ou escrita de arquivo ocorre aqui — apenas o reconhecimento da intenção.

## Importações
- (nenhuma): a função depende apenas do crate `core`/`std` e de `str` para as rotinas de manipulação.
