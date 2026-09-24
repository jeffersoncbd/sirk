### Resumo

Este arquivo implementa a execução de **custom tools**: scripts Bash armazenados em `tools/<nome>.sh` dentro do diretório de execução do projeto. Ele valida o nome da ferramenta, resolve o script com segurança, passa argumentos posicionais e retorna exatamente o conteúdo produzido em `stdout`.

### Funcionamento

O fluxo principal de `execute` é:

1. Valida o nome recebido, permitindo apenas letras, números, `_` e `-`.
2. Resolve o diretório de execução e o subdiretório `tools`.
3. Verifica se ambos permanecem dentro do diretório permitido.
4. Localiza `<nome>.sh`, resolve seu caminho e confirma que é um arquivo regular dentro de `tools`.
5. Cria uma `Invocation` para executar o script usando `bash`.
6. Passa os argumentos como argumentos posicionais separados, sem interpolá-los como código shell.
7. Executa o processo por meio de `BashService`.
8. Retorna o `stdout` apenas se o processo terminar com sucesso.
9. Em caso de erro de resolução, execução ou status de saída diferente de zero, retorna uma mensagem `Err`.

### Componentes principais

- `valid_name`: verifica se o nome da custom tool não está vazio e contém apenas caracteres ASCII alfanuméricos, `_` ou `-`.
- `arguments`: converte uma string JSON em `Vec<String>`, exigindo uma lista de argumentos textuais.
- `execute`: resolve, valida e executa o script Bash correspondente ao nome informado.
- `Invocation`: estrutura importada de `crate::services`, usada para descrever o programa, seus argumentos e o diretório de trabalho.
- `BashService`: serviço interno responsável pela execução do processo e captura da saída.
- Módulo de testes:
  - verifica que argumentos com espaços, aspas e sintaxe shell são passados literalmente;
  - verifica erros para scripts inexistentes;
  - verifica que uma saída parcial não é retornada quando o script termina com falha.

### Dependências e integrações

- `crate::services::{BashService, Invocation}`: abstrações internas para executar comandos Bash.
- `serde_json`: desserializa a lista de argumentos fornecida em JSON.
- `std::path::Path`: representa o diretório de execução e os caminhos dos scripts.
- Scripts locais em `tools/*.sh`: são os executáveis efetivamente chamados pelo módulo.

### Observações

- O arquivo não usa `unsafe`, concorrência ou `async/await`.
- Os erros são propagados com `Result<String, String>` e mensagens descritivas.
- O script é executado com `bash`; portanto, precisa ser compatível com Bash.
- O conteúdo retornado é apenas o `stdout` de uma execução bem-sucedida. O código não expõe diretamente o tratamento de `stderr`.
- A validação de caminhos baseada em `canonicalize` impede que diretórios ou scripts escapem das áreas permitidas, inclusive por meio de caminhos resolvidos.
