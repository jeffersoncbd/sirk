## Resumo
Organiza o transporte JSON-RPC delimitado por linhas e reexporta `serve`.

## Funcionamento
Declara os módulos `handle`, `serve` e `types`, e torna `serve::serve` disponível pelo módulo `rpc`. A implementação e o tratamento de erros de `serve` não aparecem neste arquivo.

## Importações
- `handle`: Declara o módulo de tratamento.
- `serve`: Declara o módulo que fornece `serve`.
- `types`: Declara o módulo de tipos.
