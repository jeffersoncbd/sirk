## Resumo
Fornece valores padrão para `OpenRouterAdapter` via `Default::default()`.

## Funcionamento
Define `executable` como `"curl"` e deixa `base_url` e `api_key` como `None`, delegateando ao chamador a configuração de endpoint e credencial em tempo de execução.

## Importações
- `super::OpenRouterAdapter`: Tipo local receptor da trait `Default`.
