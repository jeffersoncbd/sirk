## Resumo
Substitui marcadores `{{ outputs.nome }}` e `{{ loop.nome }}` de um template pelos respectivos valores.

## Funcionamento
Percorre o template procurando `{{`, exige fechamento `}}` e escolhe a fonte de dados conforme o prefixo: `loop.` usa `locals` (erro se ausente) e `outputs.` usa o mapa `outputs`. A chave é validada por `valid` e deve existir no mapa, caso contrário retorna `Err` com mensagem descritiva. O texto restante é copiado ao final e o resultado devolvido em `Ok`, sem modificar as entradas.

## Importações
- `super::output_name::valid`: Confere se o nome da chave é válido.
- `std::collections::BTreeMap`: Maps ordenados de chaves/valores de saída e loop.
