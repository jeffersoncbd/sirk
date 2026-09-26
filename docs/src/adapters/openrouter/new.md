## Resumo
Constrói um `OpenRouterAdapter` normalizando URL base e chave de API.

## Funcionamento
Converte `executable` e `base_url` para `String` e garante que a URL não fique vazia via `nonempty`. A `api_key` opcional é descartada (`None`) quando ausente, só com espaços ou vazia.

## Importações
- `OpenRouterAdapter`: Tipo ao qual o construtor pertence.
- `nonempty`: Valida a URL base, rejeitando string vazia.
