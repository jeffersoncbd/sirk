## Resumo
Retoma a execução de um harness a partir de um caminho de estado.

## Funcionamento
Encaminha `path` para `new_harness::runner::resume`, convertendo o resultado com `?` (propagando erro como `String`) e devolvendo `Ok(())` em caso de sucesso. Sem estado próprio ou validações.

## Importações
- `std::path::Path`: caminho do estado de retomada, recebido por referência.
