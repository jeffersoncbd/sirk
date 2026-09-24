### Resumo

Este arquivo implementa o comando interno `WRITE`, responsável por criar ou substituir arquivos UTF-8 dentro de um diretório de execução específico. Ele valida o caminho, impede operações fora do diretório permitido e controla explicitamente a sobrescrita por meio do parâmetro `force`.

### Funcionamento

A função pública `write`:

1. Rejeita caminhos vazios.
2. Resolve o diretório-raiz com `canonicalize`.
3. Converte o caminho informado em um caminho absoluto dentro da raiz.
4. Verifica se o destino permanece dentro do diretório de execução.
5. Cria os diretórios-pai ausentes.
6. Recusa destinos que não sejam arquivos regulares.
7. Recusa sobrescrever arquivos existentes quando `force` é `false`.
8. Usa `create_new` para criação exclusiva ou `truncate` para substituição forçada.
9. Escreve o conteúdo como bytes UTF-8 e chama `sync_all` para sincronizar o arquivo.

A função retorna `Ok(())` em caso de sucesso ou `Err(String)` com uma mensagem específica em caso de falha.

### Componentes principais

- `write(...) -> Result<(), String>`: API pública do módulo para criação ou substituição de arquivos.
- `ensure_directory(...)`: função privada que cria recursivamente, um nível por vez, os diretórios-pai necessários.
- `Component::Normal`: usado para rejeitar componentes de caminho que não sejam nomes normais, como navegação relativa ou componentes especiais.
- `OpenOptions`: configura o arquivo para criação exclusiva ou substituição forçada.
- Módulo `tests`: contém testes para:
  - criação de arquivos em diretórios aninhados;
  - recusa de sobrescrita sem `force`;
  - substituição com `force: true`;
  - rejeição de caminhos externos e destinos que não sejam arquivos.

### Dependências e integrações

O arquivo usa apenas a biblioteca padrão do Rust:

- `std::fs`: operações de arquivos, diretórios e metadados;
- `std::io::Write`: escrita e sincronização do conteúdo;
- `std::path`: manipulação e validação de caminhos.

A função recebe o diretório de execução como `&Path`, portanto depende do chamador para fornecer corretamente essa raiz. O comportamento está alinhado a uma ferramenta `WRITE` controlada pelo workflow, conforme indicado pelo comentário inicial.

### Observações

- O módulo não usa `unsafe`, concorrência ou comunicação externa.
- O tratamento de erros é baseado em `Result<(), String>`, com mensagens próprias para cada tipo de falha.
- Arquivos existentes só podem ser substituídos quando `force` é `true`.
- Os testes usam diretórios temporários e removem esses diretórios ao final de cada caso.
