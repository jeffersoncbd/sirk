### Resumo

Este arquivo implementa a operação `WRITE`: cria um arquivo UTF-8 dentro de um diretório de execução, podendo criar diretórios-pai e, opcionalmente, substituir arquivos existentes.

Também valida caminhos para impedir gravações fora do diretório permitido, rejeita diretórios e links simbólicos como destino e inclui testes unitários para esses comportamentos.

### Funcionamento

A função pública `write` é uma forma simplificada de chamar `write_with_options` sem o modo `skip`.

`write_with_options`:

1. Valida combinações inválidas de opções e garante que o caminho não esteja vazio.
2. Resolve o diretório raiz com `canonicalize`.
3. Converte o caminho recebido em um caminho absoluto dentro dessa raiz.
4. Verifica se todos os componentes do caminho permanecem dentro do diretório de execução.
5. Cria os diretórios-pai ausentes usando `ensure_directory`.
6. Analisa o destino existente:
   - rejeita diretórios e outros tipos que não sejam arquivos regulares;
   - rejeita links simbólicos;
   - recusa sobrescrever arquivos, a menos que `force` seja `true`;
   - retorna sucesso sem alterar o arquivo quando `skip` é `true`.
7. Abre o arquivo com `OpenOptions`:
   - `create_new(true)` para criação exclusiva;
   - `create(true).truncate(true)` quando `force` está habilitado.
8. Escreve o conteúdo e chama `sync_all` para sincronizar os dados com o armazenamento.

O tratamento de erros usa `Result<(), String>`, convertendo erros de filesystem em mensagens textuais específicas.

### Componentes principais

- `write(directory, path, content, force)`: API pública principal para criar ou substituir um arquivo.
- `write_with_options(directory, path, content, force, skip)`: implementação completa, incluindo os modos de sobrescrita e ignorar escrita.
- `ensure_directory(directory, path)`: garante que um diretório-pai exista. Cria apenas um nível por chamada e lida com possíveis condições de corrida durante a criação.
- `Component`, `Path`: usados para validar e montar caminhos com segurança.
- `OpenOptions`: configura a criação exclusiva ou a substituição do arquivo.
- `#[cfg(test)] mod tests`: testes unitários que cobrem:
  - criação de arquivos e diretórios aninhados;
  - preservação de arquivos no modo `skip`;
  - substituição com `force`;
  - rejeição de caminhos externos;
  - rejeição de diretórios e links simbólicos.

### Dependências e integrações

O arquivo usa apenas a biblioteca padrão do Rust:

- `std::fs` para metadados, criação de diretórios e leitura/escrita de arquivos;
- `std::io::Write` para escrever e sincronizar o conteúdo;
- `std::path` para manipulação e validação de caminhos.

Ele depende de um diretório de execução fornecido pelo chamador e não interage diretamente com outros módulos ou serviços externos no conteúdo apresentado.

### Observações

- O conteúdo é recebido como `&str` e convertido para bytes UTF-8 com `as_bytes()`.
- Não há uso de `unsafe`, concorrência explícita ou comunicação de rede.
- O arquivo evita substituir links simbólicos e impede que o destino escape da raiz definida.
- `force` e `skip` são mutuamente exclusivos.
- O contexto externo — especialmente quem chama essas funções e como interpreta as mensagens de erro — não está presente no trecho fornecido.
