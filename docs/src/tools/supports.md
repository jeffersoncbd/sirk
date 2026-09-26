## Resumo
Verifica se um nome de ferramenta é suportado pela lista de operações permitidas.

## Funcionamento
A função recebe um `&str` e retorna `true` somente se o texto for idêntico a um dos oito valores aceitos: `TREE`, `GIT-STATUS-TREE`, `READ`, `WRITE`, `DELETE`, `EDIT`, `ASK` ou `AWAIT`. A comparação é feita via macro `matches!`, que faz pattern matching literal de `&str` — logo, a checagem é exata e sensível a maiúsculas/minúsculas, sem tolerância a espaços ou variações. Qualquer outro nome resulta em `false`. Não há `Result`, `Option`, alocação de memória ou qualquer efeito colateral: a função é pura.

## Importações
- `std::matches!`: Macro da biblioteca padrão usada para validar o nome contra os valores aceitos.
