## Resumo
Converte um valor arbitrário em uma string segura para interpolação em comandos shell POSIX.

## Funcionamento
Se `value` começa com `$` e o nome resultante existe no mapa `environment` **e** é composto apenas por letras ASCII maiúsculas, dígitos ou `_`, a função retorna a referência entre aspas duplas (`"$VAR"`), preservando a expansão pelo shell. Em qualquer outro caso — inclusive quando a variável não existe ou o nome é inválido — o valor é envolvido em aspas simples, com cada aspa simples interna substituída pela sequência `'\''`, evitando quebra de quoting. Não há retorno de `Result`/`Option` e nenhum efeito colateral: a função é pura e sempre devolve uma `String` válida para shell, com literais passados por completo (sem expansão) e referências expandidas no momento da execução.

## Importações
- `std::collections::BTreeMap`: Mapa de ambiente consultado por nome, usado via caminho completo (sem `use`).
