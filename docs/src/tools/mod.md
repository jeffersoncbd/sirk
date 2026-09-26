### Resumo

O arquivo `src/tools/mod.rs` centraliza o despacho e a interpretação de ferramentas compartilhadas por etapas de workflow e solicitações de agentes. Ele identifica requisições textuais válidas, executa ferramentas de leitura ou listagem e formata seus resultados.

### Funcionamento

O módulo declara submódulos para ferramentas específicas: `custom`, `delete`, `edit`, `git_status_tree`, `new_agent`, `read`, `tree` e `write`.

A função `supports` reconhece os nomes `TREE`, `GIT-STATUS-TREE`, `READ`, `WRITE`, `DELETE`, `EDIT`, `ASK` e `AWAIT`, enquanto `request` interpreta apenas mensagens isoladas nos formatos `TREE`, `READ: <caminho>` ou `READ:`. O texto é aparado antes da análise; requisições com conteúdo adicional ou caminhos contendo quebras de linha são rejeitadas.

A execução ocorre por `execute` e `execute_with_input`. Embora `supports` também reconheça `WRITE`, `DELETE`, `EDIT`, `ASK` e `AWAIT`, o despacho efetivo atualmente trata apenas:
- `TREE`, listando arquivos e serializando os caminhos como JSON.
- `GIT-STATUS-TREE`, listando arquivos relacionados ao estado do Git.
- `READ`, lendo o conteúdo de um caminho no diretório informado.
Nomes desconhecidos produzem um `Result::Err`. Caminhos não representáveis como UTF-8 e falhas de serialização também são convertidos em mensagens de erro. A função `format_paths` garante que a saída seja um array JSON formatado e terminado por nova linha.
Os testes verificam a preservação correta de caracteres especiais em caminhos e a aceitação somente de requisições autônomas.

### Componentes principais

- `pub mod custom`, `delete`, `edit`, `git_status_tree`, `new_agent`, `read`, `tree` e `write`: registram os submódulos de ferramentas.
- `supports(name: &str) -> bool`: informa se o nome da ferramenta está entre os nomes reconhecidos.
- `request(text: &str) -> Option<(&str, &str)>`: analisa requisições textuais autônomas e retorna o nome da ferramenta e sua entrada.
- `execute(...) -> Result<String, String>`: executa uma ferramenta sem entrada explícita.
- `execute_with_input(...) -> Result<String, String>`: realiza o despacho com entrada e diretório de trabalho.
- `format_paths(...)`: converte caminhos para strings UTF-8 e os serializa em JSON.
- Módulo privado `tests`: valida a serialização de caminhos e o parser de requisições.

### integrações

As principais partes públicas expostas são os módulos declarados, `supports`, `request`, `execute` e `execute_with_input`. O arquivo depende de `serde_json` para serialização, de `std::path::Path`/`PathBuf` para manipulação de caminhos e dos módulos internos `read`, `tree` e `git_status_tree` para realizar as operações correspondentes.