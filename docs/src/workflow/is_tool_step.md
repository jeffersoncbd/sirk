## Resumo
Verifica se um `Step` do workflow executa uma ferramenta.

## Funcionamento
O método `is_tool_step` retorna `true` quando o step possui qualquer uma das duas referências opcionais preenchidas — `tool` (ferramenta nativa) ou `custom_tool` (ferramenta customizada) — e `false` caso ambas sejam `None`. Não há validação, alocação ou efeitos colaterais: é apenas uma checagem de discriminante sobre campos já existentes na estrutura.

## Importações
- `super::Step`: Trait_tipo onde o método é implementado, fornecendo os campos `tool` e `custom_tool`.
