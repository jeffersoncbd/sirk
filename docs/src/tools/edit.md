### Resumo

Este arquivo implementa um mecanismo de edição de arquivos exclusivo para workflows. Ele valida operações, prepara alterações com base no conteúdo atual, verifica versões usando SHA-256, gera diffs unificados e grava o resultado de forma durável e relativamente segura contra alterações concorrentes.

### Funcionamento

O fluxo principal é:

1. Uma `Request` descreve o arquivo, a operação, as coordenadas, a versão esperada e o texto de entrada.
2. `Request::validate` verifica se:
   - o caminho foi informado;
   - as coordenadas correspondem à operação;
   - linhas começam em 1;
   - deleções não possuem entrada;
   - operações por linha possuem uma versão SHA-256 válida.
3. `Request::apply_to` verifica conflito de versão e calcula os offsets de bytes para inserir, remover, substituir, acrescentar ou preceder conteúdo.
4. `Pending::prepare` lê o arquivo, armazena seu conteúdo original e valida previamente a alteração.
5. `Pending::diff` produz um diff unificado.
6. `Pending::commit` revalida o estado do arquivo e grava o resultado:
   - cria arquivos ausentes sem sobrescrever um arquivo criado concorrentemente;
   - substitui arquivos existentes por meio de arquivo temporário e `rename`;
   - preserva permissões do arquivo original;
   - sincroniza o arquivo e o diretório com `sync_all`.

O caminho do arquivo é resolvido dentro de um diretório-base. Arquivos simbólicos não são aceitos, e o caminho final precisa permanecer dentro desse diretório.

### Componentes principais

- `Operation`: enum público com as operações `Insert`, `Delete`, `Replace`, `Prepend` e `Append`. É serializado em letras minúsculas.
- `Request`: estrutura pública que representa uma solicitação de edição.
  - `validate`: valida a forma da solicitação.
  - `apply_to`: aplica a edição a um conteúdo em memória.
- `version`: calcula o digest SHA-256 do conteúdo e o retorna como hexadecimal.
- `Pending`: representa uma edição preparada, contendo a solicitação, o conteúdo anterior e a indicação de que o arquivo não existia.
  - `validate`: verifica a consistência do registro preparado.
  - `prepare`: captura o conteúdo atual e prepara a edição.
  - `diff`: gera o diff da alteração.
  - `commit`: confirma a alteração no filesystem.
- `sync_parent`: sincroniza o diretório que contém o arquivo.
- `read_optional`: lê um arquivo UTF-8, diferenciando arquivo ausente de erro de leitura.
- `target`: valida e resolve o caminho do arquivo dentro do diretório de execução.
- `display`: imprime um diff com cores quando a saída é um terminal e `NO_COLOR` não está definido.
- `render`: aplica destaque visual a linhas adicionadas e removidas e escapa caracteres de controle.
- Módulo de testes: cobre edições por linha, preservação de bytes, fim de arquivo, conflitos de versão e renderização de diffs.

### Dependências e integrações

- `serde`: serialização e desserialização de `Operation`, `Request` e `Pending`.
- `sha2`: cálculo do SHA-256 usado no controle otimista de versão.
- `similar`: geração de diffs unificados.
- `std::fs` e `std::io`: leitura, escrita, arquivos temporários, sincronização e detecção de terminal.
- `std::sync::atomic`: geração de nomes únicos para arquivos temporários.
- Interage com o filesystem por meio do diretório-base recebido em `prepare` e `commit`.

### Observações

- O arquivo trabalha apenas com texto UTF-8, mas preserva exatamente os bytes do conteúdo lido, incluindo finais de linha `\r\n` e ausência de newline no fim.
- Operações `Append` e `Prepend` podem criar um arquivo ausente; operações baseadas em linhas exigem que ele exista.
- O tratamento de erros usa `Result<_, String>` e o operador `?`; não há `panic!` nem código `unsafe` no arquivo.
- A validação de versão evita aplicar alterações sobre conteúdo diferente daquele lido durante a preparação.
- O comportamento completo depende do protocolo externo que fornece as versões, armazena `Pending` e decide quando chamar `prepare`, `diff` e `commit`.
