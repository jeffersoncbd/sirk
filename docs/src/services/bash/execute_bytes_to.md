## Resumo
Executa um comando Bash renderizado a partir de uma `Invocation` e captura o stdout em bytes brutos, espelhando-os em um `Write` fornecido.

## Funcionamento
O método monta um `Command` com `-lc` e o script gerado por `self.render`, aplica o diretório de trabalho e as variáveis de ambiente da invocação, e configura `stdin` nulo, `stdout` canalizado e `stderr` herdado. O pipe é lido em blocos de 8 KiB: cada pedaço é escrito e flushed no writer recebido e também acumulado em um vetor. Erros `Interrupted` apenas reiniciam o laço, enquanto EOF encerra a leitura. Se a captura ou a escrita falhar, o processo filho é morto e aguardado para não ficar órfão, e o erro é propagado. Em sucesso, retorna `BinaryProcessOutput` com o status de saída e os bytes coletados.

## Importações
- `BashService`: Tipo dono do método, guarda executável e contexto do shell.
- `BinaryProcessOutput`: Estrutura de retorno com status e stdout bruto capturado.
- `Invocation`: Entrada com script, diretório de trabalho e ambiente.
- `io`: Tipo de erro e `ErrorKind` para tratar `Interrupted`.
- `Read`: Permite a leitura incremental do pipe de stdout.
- `Write`: Usado para espelhar os bytes no destino fornecido.
- `Command`: Cria e configura o processo do shell.
- `Stdio`: Define `null`, `piped` e `inherit` para cada stream.
