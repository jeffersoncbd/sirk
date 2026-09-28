## Resumo
Expõe o transporte HTTP para SDKs remotos e executados em contêineres.

## Funcionamento
Declara os módulos `handle`, `serve` e `types`, e reexporta `serve::serve` como parte da interface pública.

## Importações
- `handle`: Módulo de tratamento HTTP.
- `serve`: Implementação do servidor HTTP.
- `types`: Tipos usados pelo transporte HTTP.
