## Resumo
Construtor de `BashService`, armazenando o executável que será usado para invocar comandos shell.

## Funcionamento
Recebe o caminho do executável como `impl Into<String>`, converte para `String` via `.into()` (aceitando tanto `&str` quanto `String`) e devolve o serviço com o campo `executable` preenchido. Não realiza validações, I/O nem pode falhar: sempre retorna um `Self` válido, delegando erros de execução para as chamadas subsequentes.

## Importações
- `super::BashService`: Tipo-alvo do construtor, definido no módulo pai `bash`.
