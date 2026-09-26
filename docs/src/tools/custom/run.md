## Resumo
Executa um script Bash personalizado (`tools/<nome>.sh`) dentro de um diretório de projeto e retorna sua saída padrão.

## Funcionamento
Valida o nome do tool (apenas letras, dígitos, `_` e `-`), canonicaliza o diretório de execução e a pasta `tools`, garantindo que ambos existam e que `tools` permaneça dentro da raiz (impedindo symlink escape). Em seguida localiza `<nome>.sh`, exigindo que continue dentro de `tools` e seja um arquivo regular, com caminho UTF-8 válido. Monta uma `Invocation` (`bash` + caminho do script + argumentos repassados literalmente, sem shell) executada via `BashService` com ambiente padrão. Qualquer falha de resolução ou execução retorna `Err` com mensagem prefixada por `CUSTOM-TOOL`; status de saída diferente de zero também é erro, garantindo que saídas parciais não sejam commitadas. Em sucesso, devolve `stdout`.

## Importações
- `super::valid_name`: Valida o formato do nome do tool antes de usá-lo no caminho.
- `crate::services::BashService`: Executa o script e captura status e saída padrão.
- `crate::services::Invocation`: Descreve programa, argumentos, diretório de trabalho e ambiente.
- `std::path::Path`: Representa o diretório base e permite canonicalizar caminhos.
