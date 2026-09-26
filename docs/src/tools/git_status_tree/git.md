## Resumo
Executa um comando `git` no diretório raiz informado e devolve seu status de saída e a saída padrão capturada.

## Funcionamento
Monta uma `Invocation` com o programa `git`, os argumentos recebidos, o `working_directory` definido como `root` e ambiente padrão (`Default::default()`). A execução é feita via `BashService::default().execute_bytes_to`, descartando a stderr (`io::sink()`). Qualquer falha de execução vira `Err(String)` com a mensagem prefixada por `GIT-STATUS-TREE could not execute Git:`, enquanto sucesso resulta em `Ok(GitOutput)` carregando o `ExitStatus` e os bytes de stdout — o chamador é responsável por interpretar o status (código diferente de zero não é tratado como erro aqui).

## Importações
- `crate::services::BashService`: executa o processo externo e captura stdout em bytes.
- `crate::services::Invocation`: struct de configuração do programa, argumentos, diretório e ambiente.
- `std::io`: fornece `io::sink()` para descartar a saída de erro.
- `std::path::Path`: representa o diretório raiz do repositório na assinatura.
- `std::process::ExitStatus`: tipo do status de saída retornado por `GitOutput`.
