## Resumo
Verifica se o valor corresponde ao padrão de componente fornecido.

## Funcionamento
Converte `pattern` e `value` em vetores de caracteres e delega a comparação à função `super::component_match::matches`, retornando `true` caso haja correspondência.

## Importações
- `super::component_match`: Fornece a lógica de comparação entre padrão e valor.
