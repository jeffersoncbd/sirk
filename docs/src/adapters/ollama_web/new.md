## Resumo
Constrói um `OllamaWebAdapter` normalizando os campos de executable, base URL e API key.

## Funcionamento
Converte `executable` e `base_url` para `String` e passa a URL base por `nonempty`, que Presumably valida que não é vazia. A `api_key` é opcional e só é mantida se não for `None` nem contiver apenas espaços em branco (`trim().is_empty()`), sendo descartada caso contrário. Não há I/O nem panic: falhas de URL inválida ficam a cargo de `nonempty`.

## Importações
- `super::OllamaWebAdapter`: Tipo do adapter ao qual o construtor pertence.
- `super::nonempty::nonempty`: Valida e garante uma URL base não vazia.
