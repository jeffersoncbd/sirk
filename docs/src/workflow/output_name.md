## Resumo
Valida se um nome de saída de workflow é composto apenas por caracteres alfanuméricos ASCII, `_` ou `-`.

## Funcionamento
A função retorna `true` apenas se o nome não for vazio e todos os seus caracteres forem alfanuméricos ASCII ou pertencerem ao conjunto `_` e `-`; qualquer outro caractere (espaços, acentos, símbolos) faz a validação falhar via `chars().all(...)`. Por ser apenas uma checagem de predicado, não produz `Result` nem `Option` e não tem efeitos colaterais.

## Importações
Nenhuma: o arquivo usa apenas a biblioteca padrão (`core`/`std`) sem `use` explícito.
