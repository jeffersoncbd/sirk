## Resumo
Converte uma lista de caminhos de arquivos em um array JSON formatado, pronto para exibição em ferramentas.

## Funcionamento
Valida cada `PathBuf` via `to_str()`, interrompendo no primeiro caminho não-UTF-8 com mensagem de erro que identifica a ferramenta (`Err(String)`). Se todos forem válidos, serializa os `&str` com `serde_json::to_string_pretty` e adiciona uma quebra de linha final; falha de serialização também vira `Err(String)` com o erro original encadeado. Entrada vazia resulta em `"[]\n"`.

## Importações
- `std::path::PathBuf`: tipos dos caminhos recebidos em `files`.
- `serde_json::to_string_pretty`: gera o array JSON indentado com um caminho por linha.
