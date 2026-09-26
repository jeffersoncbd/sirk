## Resumo
`Agent::parse` converte um arquivo Markdown de agente com front matter YAML em um `Agent` validado.

## Funcionamento
Lê a fonte linha a linha, exigindo cabeçalho `---` inicial e um `---` de fechamento; o bloco entre eles é desserializado como `Metadata` (campos desconhecidos são rejeitados, e flags como `TREE_TOOL`/`EDIT_TOOL`/`DELETE_TOOL` só são verdadeiras com `allow`). Depois valida regras de negócio: `adapter` não vazio, `json: true` bloqueado por ausência de normalização de eventos, `DELETE_WITHOUT_CONFIRM` exige `DELETE_TOOL: allow` e instruções Markdown não vazias. Em sucesso, devolve `Agent` com o `id` fornecido, o modelo normalizado e o `call_prefix` expandido; em falha, retorna `Err(String)` descritivo. A serialização usa `Deserialize` próprio para tolerar tanto o arquivo original quanto snapshots salvos.

## Importações
- `serde`: Deriva `Serialize`/`Deserialize` para converter o agente em YAML.
- `serde_yaml`: Desserializa o front matter e valida campos desconhecidos.
- `model::deserialize`: Normaliza o nome do modelo (trim + minúsculas).
- `call_prefix::deserialize`: Aceita um token único ou lista de argumentos.
- `tree_default::allow`: Valor padrão de `TREE_TOOL` (desabilitado).
- `tree_permission::deserialize`: Só habilita a ferramenta de árvore com `allow`.
- `edit_permission::deserialize`: Só habilita edição externa com `allow`.
- `delete_permission::deserialize`: Só habilita deleção com `allow`.
- `id::valid_id`: Valida o identificador usado para `Agent::load`.
