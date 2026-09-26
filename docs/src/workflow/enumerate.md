## Resumo
Fornece um atalho booleano que indica se a etapa está habilitada para enumeração.

## Funcionamento
O método `enumerate` é um getter simples sobre o campo opcional `enumerate` do `Step`, sem efeitos colaterais. Se o campo for `None` (não definido na configuração), retorna `false` por meio de `unwrap_or`, garantindo um booleano em vez de `Option<bool>` e evitando tratamento de erro pelo chamador.

## Importações
- `super::Step`: Tipo próprio do módulo pai, receptor do método implementado.
