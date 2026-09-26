## Resumo
Aplica um prefixo de comando a uma `Invocation`, reposicionando programa e argumentos.

## Funcionamento
`with_prefix` consome `self` e o slice `prefix`: se o slice for vazio, devolve a invocação inalterada; caso contrário, o primeiro elemento vira o novo `program` e os restantes são antepostos aos argumentos originais, resultando em `prefix[1..] + [program_anterior] + argumentos`. `working_directory` e `environment` ficam intactos. Não faz I/O nem panics; falha de formato é representada pela ausência de alteração.

## Importações
- `std::collections::BTreeMap`: guarda o ambiente enviado só ao processo filho.
- `std::path::PathBuf`: representa o diretório de trabalho da invocação.
- `std::mem::replace`: troca `program` pelo primeiro item do prefixo.
