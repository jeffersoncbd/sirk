## Resumo
`with_prefix` antepõe um comando à invocação, preservando suas configurações de execução.

## Funcionamento
Se o prefixo estiver vazio, retorna a invocação sem alterações. Caso contrário, usa o primeiro item como programa e insere os demais argumentos antes do programa e dos argumentos originais. Mantém o diretório de trabalho e o ambiente; não retorna erros.

## Importações
- `std::mem`: Substitui o programa e recupera seu valor original.
- `std::collections::BTreeMap`: Armazena variáveis de ambiente da invocação.
