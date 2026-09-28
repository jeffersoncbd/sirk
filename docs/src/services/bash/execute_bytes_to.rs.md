## Resumo
Executa uma invocação do Bash e captura sua saída binária.

## Funcionamento
Inicia o processo no diretório e ambiente da invocação, sem entrada padrão e com erros direcionados ao processo pai. Lê a saída em blocos, copia cada bloco para `output` e a acumula; se a leitura ou escrita falhar, encerra e aguarda o processo antes de retornar o erro. Ao terminar, retorna o status e os bytes capturados.

## Importações
- `super::{BashService, BinaryProcessOutput}`: serviço e tipo do resultado.
- `crate::services::Invocation`: parâmetros da execução.
- `std::io::{self, Read, Write}`: leitura, escrita e erros de E/S.
- `std::process::{Command, Stdio}`: criação e configuração do processo.
