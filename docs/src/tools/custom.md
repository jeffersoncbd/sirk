### Resumo

O arquivo implementa o suporte a **CUSTOM-TOOL**, permitindo executar scripts Bash localizados no diretório `tools` do projeto, com argumentos posicionais fornecidos como strings JSON.

### Funcionamento

O nome da ferramenta é validado para aceitar apenas letras, dígitos, `_` e `-`. Os argumentos são desserializados com `serde_json` como `Vec<String>`.

Na execução, o código:

1. Resolve e canonicaliza o diretório de execução.
2. Localiza o subdiretório `tools`.
3. Garante que tanto o diretório quanto o script permaneçam dentro dos limites esperados.
4. Procura um arquivo chamado `<nome>.sh` e verifica se ele é um arquivo regular.
5. Monta uma `Invocation` para executar `bash`, passando o caminho do script e os argumentos separadamente.
6. Usa `BashService` para executar o processo e capturar a saída padrão.
7. Retorna o `stdout` apenas se o processo terminar com sucesso.

Erros de validação, resolução de caminhos, execução ou término com status diferente de sucesso são convertidos em mensagens `String`. Quando o script falha, sua saída parcial não é retornada como resultado válido.

Os testes verificam que os argumentos são passados literalmente, sem interpretação de shell, que o `stdout` é capturado e que scripts ausentes ou com falha produzem erro.

### Componentes principais

- `valid_name`: valida o nome da ferramenta.
- `arguments`: converte uma representação JSON em `Vec<String>`.
- `execute`: valida caminhos, prepara a invocação do Bash, executa o script e retorna seu `stdout`.
- `BashService` e `Invocation`: abstrações internas usadas para executar o processo externo.
- Módulo `tests`: cria projetos temporários e testa passagem de argumentos, captura de saída e tratamento de falhas.

### integrações

- `pub fn valid_name`, `pub fn arguments` e `pub fn execute` são funções públicas disponíveis para outras partes da crate.
- Integra-se ao módulo interno `crate::services`, utilizando `BashService` e `Invocation`.
- Utiliza a crate externa `serde_json` para desserializar os argumentos.
- Depende do executável externo `bash` e do sistema de arquivos para localizar os scripts em `tools/`.
