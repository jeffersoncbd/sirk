## Resumo
Inicia o CLI em modo RPC e configura uma instância de `Sirk` para comunicação.

## Funcionamento
Executa o programa no diretório informado, conecta stdin e stdout por pipes e herda stderr. Retorna `Error` se o processo não iniciar ou se algum pipe esperado estiver indisponível; caso contrário, guarda o processo, prepara o leitor de saída e inicia a sequência de IDs em 1.

## Importações
- `crate::Error`: Representa erros de inicialização e protocolo.
- `crate::Sirk`: Tipo configurado e retornado pela função.
- `std::ffi::OsStr`: Tipo aceito para o executável.
- `std::io::BufReader`: Bufferiza a leitura da saída do CLI.
- `std::path::Path`: Tipo aceito para o diretório de execução.
- `std::process::{Command, Stdio}`: Inicia o CLI e configura seus fluxos.
