## Resumo
Fornece um serviço compartilhado que executa comandos via `bash -lc`, renderizando cada argumento com escape POSIX de aspas simples.

## Funcionamento
`BashService` guarda apenas o executável e expõe operações derivadas de um `Invocation` (programa, argumentos, diretório de trabalho e ambiente): `render` monta a linha de comando com cada item entre aspas simples, escaping aspas internas; variáveis de ambiente marcadas (`$NOME`) são passadas sem interpolação de valor, evitando que conteúdo do usuário vire código shell. A execução (`execute_to`, `execute_streaming`, `execute_bytes_to`) escreve a saída em um `Write` fornecido pelo chamador e retorna `ProcessOutput` com `ExitStatus` e `stdout` capturado — o `Result`/`io::Error` propaga falhas de spawn e de escrita no destino.

## Importações
- `std::process::ExitStatus`: código de saída do processo filho.
