## Resumo
`Agent::parse` valida e converte uma definição textual em um agente.

## Funcionamento
Exige metadados YAML entre delimitadores `---`, valida campos como adaptador e permissões e rejeita instruções vazias. Em caso de erro, retorna uma mensagem; se tudo for válido, retorna `Agent` com os metadados e as instruções Markdown normalizadas.

## Importações
- `serde`: serialização e desserialização dos metadados.
- `serde_yaml`: conversão dos metadados YAML.
- `model`: desserialização do modelo.
- `call_prefix`: desserialização do prefixo de chamada.
- `tree_default`: valor padrão da permissão de árvore.
- `tree_permission`: desserialização da permissão de árvore.
- `edit_permission`: desserialização da permissão de edição.
- `delete_permission`: desserialização das permissões de exclusão.
