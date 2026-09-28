## Resumo
Prepara uma edição verificando o arquivo e aplicando a operação ao conteúdo atual.

## Funcionamento
Permite arquivo inexistente apenas para anexar ou inserir no início; caso contrário, retorna erro. Lê o conteúdo, valida a operação e registra o estado anterior e se o arquivo faltava.

## Importações
- `Operation`: identifica o tipo de edição.
- `Pending`: tipo ao qual `prepare` pertence.
- `read_optional`: lê o conteúdo se o arquivo existir.
- `target`: resolve o caminho do arquivo alvo.
- `Path`: representa o diretório de trabalho.
