### Resumo

Este arquivo implementa o recurso `CUSTOM-TOOL`: execução segura de scripts Bash locais em `tools/<nome>.sh`, recebendo argumentos posicionais e retornando a saída padrão do script.

### Funcionamento

O fluxo principal de `execute` é:

1. Valida o nome da ferramenta, permitindo apenas letras, números, `_` e `-`.
2. Resolve o diretório de execução e o subdiretório `tools`.
3. Verifica que o diretório `tools` permanece dentro da raiz do projeto.
4. Localiza e resolve `tools/<nome>.sh`, garantindo que o script permaneça dentro de `tools`.
5. Confirma que o caminho aponta para um arquivo regular.
6. Executa o script usando `bash`, passando os argumentos literalmente, sem interpolá-los em comandos shell.
7. Captura a saída padrão (`stdout`).
8. Retorna erro caso o processo falhe ou termine com status diferente de sucesso.

### Componentes principais

- `valid_name`: verifica se o nome da ferramenta é não vazio e contém apenas caracteres permitidos.
- `arguments`: converte uma string JSON em `Vec<String>`, exigindo uma lista de argumentos textuais.
- `execute`: localiza, valida e executa o script Bash, retornando `Result<String, String>`.
- `temporary_project`: função de teste que cria um projeto temporário com um diretório `tools`.
- `passes_arguments_literally_and_captures_stdout`: confirma que argumentos com espaços, aspas e sintaxe shell são transmitidos literalmente.
- `reports_missing_scripts_and_unsuccessful_exits`: verifica erros para scripts inexistentes e encerramentos com falha.

### Dependências e integrações

- `crate::services::BashService`: executa o processo externo.
- `crate::services::Invocation`: representa o programa, argumentos e diretório de trabalho da execução.
- `serde_json`: interpreta os argumentos fornecidos em JSON.
- `std::path::Path`: manipula caminhos de arquivos e diretórios.

O arquivo depende da existência de um diretório `tools` dentro do diretório de execução e de scripts Bash com extensão `.sh`.

### Observações

- O tratamento de erros usa `Result` e mensagens textuais detalhadas; não há `panic!` no fluxo normal.
- A canonicalização dos caminhos e as verificações com `starts_with` impedem que diretórios ou scripts fora da área permitida sejam usados, inclusive por meio de caminhos indiretos.
- A execução ocorre externamente através do `bash`, portanto o arquivo produz efeitos colaterais no sistema conforme o script executado.
- Em caso de falha, a saída parcial não é retornada como sucesso.
