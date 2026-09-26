## Resumo
Extrai o texto de uma pergunta a partir de uma linha de comando que começa com o prefixo `ASK:`.

## Funcionamento
A função remove espaços iniciais de `text` e verifica se o restante inicia com `ASK:`; se o prefixo não existir, retorna `None`. Quando existe, o prefixo é removido e os espaços circundantes do conteúdo são aparados, devolvendo `Option<&str>` com a pergunta limpa. Não possui efeito colateral: apenas fatia a string original e opera sobre a memória, sem alocação ou cópia.

## Importações
- Nenhuma: a função usa apenas métodos da biblioteca padrão (`str`).
