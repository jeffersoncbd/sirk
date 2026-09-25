### Resumo

Este arquivo implementa o mecanismo de edição de arquivos usado exclusivamente por workflows. Ele valida operações, verifica a versão atual do arquivo por SHA-256, prepara alterações persistentes, gera diffs e aplica mudanças de forma controlada e parcialmente atômica.

### Funcionamento

As edições podem inserir, excluir, substituir, acrescentar ou antepor texto. Operações baseadas em linhas exigem um digest SHA-256 obtido anteriormente por `READ`; isso evita editar uma versão desatualizada do arquivo.

O fluxo principal é:

1. Validar a requisição e seus parâmetros.
2. Ler o arquivo-alvo e confirmar sua versão.
3. Armazenar o conteúdo original em um registro `Pending`.
4. Calcular o conteúdo resultante e gerar um diff unificado.
5. Antes da gravação, verificar novamente se o arquivo não mudou.
6. Escrever em um arquivo temporário e substituir o original, preservando permissões quando aplicável.

`Append` e `Prepend` também podem criar um arquivo ausente. A criação usa `hard_link` para evitar substituir um arquivo criado concorrentemente. Caminhos são limitados ao diretório de execução e symlinks não são aceitos como arquivos-alvo.

A exibição do diff pode usar cores quando a saída for um terminal e `NO_COLOR` não estiver definido. Caracteres de controle vindos do conteúdo são escapados para não executar sequências de terminal.

### Componentes principais

- `Operation`: enumeração serializável das operações de edição: `insert`, `delete`, `replace`, `prepend` e `append`.

- `Request`: descreve uma edição, incluindo caminho, operação, coordenadas de linhas, versão esperada e texto de entrada.
  - `validate`: verifica coordenadas, versão e regras específicas de cada operação.
  - `apply_to`: aplica a edição a uma string, preservando exatamente os bytes do conteúdo textual.

- `version`: calcula o digest SHA-256 do conteúdo e o retorna como hexadecimal.

- `Pending`: representa uma edição preparada, contendo a requisição, o conteúdo anterior e a informação sobre eventual ausência do arquivo.
  - `validate`: valida o registro persistido.
  - `prepare`: lê o arquivo e salva o estado original antes da mutação.
  - `diff`: produz um diff unificado usando `similar::TextDiff`.
  - `commit`: confirma a edição, verifica conflitos e grava o resultado.

- `sync_parent`: sincroniza o diretório pai no sistema de arquivos após a alteração.

- `read_optional`: lê um arquivo UTF-8 ou informa que ele não existe.

- `target`: resolve e valida o caminho do arquivo dentro do diretório permitido.

- `display` e `render`: exibem ou formatam o diff, opcionalmente com cores e escape de caracteres de controle.

- Módulo de testes: cobre edições por linha, operações no início e fim do arquivo, conflitos de versão, diffs e renderização colorida.

### Dependências e integrações

- `serde`: serialização e desserialização de `Operation`, `Request` e `Pending`, incluindo rejeição de campos desconhecidos.
- `sha2`: cálculo do SHA-256 usado no controle de versão.
- `similar`: geração de diffs unificados.
- `std::fs` e `std::io`: leitura, escrita, sincronização, permissões e manipulação de arquivos.
- `AtomicU64`: criação de nomes exclusivos para arquivos temporários.
- O arquivo integra-se ao sistema de histórico/workflow por meio dos registros `Pending` e dos blocos `INPUT`, embora a implementação desse histórico esteja em outro módulo.

### Observações

O tratamento de erros usa `Result<(), String>` ou `Result<T, String>`, com validações explícitas e propagação pelo operador `?`. Não há uso de `unsafe`, `async` ou threads.

A explicação do fluxo completo depende dos módulos que criam, persistem e retomam os registros `Pending`; esse contexto não está presente no arquivo analisado.
