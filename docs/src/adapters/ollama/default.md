## Resumo
Fornece a implementação padrão de `Default` para `OllamaAdapter`.

## Funcionamento
O `impl Default` delega a construção para `OllamaAdapter::new("ollama")`, garantindo que uma instância criada via `Default` sempre aponte para o host `ollama`, sem campos customizados ou lógica adicional.

## Importações
- `super::OllamaAdapter`: Tipo base cuja construção padrão é definida aqui.
