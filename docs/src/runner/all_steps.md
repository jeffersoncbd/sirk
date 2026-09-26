## Resumo
Coleta recursivamente todos os `Step` aninhados a partir de uma lista, incluindo os ramos `iter`, `is_true` e `is_false`.

## Funcionamento
A função percorre a lista recebida empurrando cada `Step` no vetor de resultado e, em seguida, chama a si mesma sobre os três vetores de filhos (`iter`, `is_true`, `is_false`), aplainando a árvore de forma exaustiva. A recursão termina porque os vetores folha são percorridos sem chamadas adicionais; não há validações, tratamento de `Result`/`Option` nem efeitos colaterais — apenas alocação de um novo `Vec<&Step>` com referências aos passos da entrada.

## Importações
- `crate::workflow::Step`: Tipo do passo do workflow, usado para tipar a entrada e o vetor retornado.
