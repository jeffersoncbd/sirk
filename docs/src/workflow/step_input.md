## Resumo
Expõe publicamente a renderização do input de um `Step`, delegando ao campo `input`.

## Funcionamento
`render_input` é um delegador fino: valida apenas o mapeamento de `outputs` (BTreeMap ordenada por chave) e `locals` opcional, encaminhando ambos ao método `render` do campo `input`. Não há lógica de negócio nem estado próprio; o `Result<String, String>` e o erro em `String` vêm diretamente do método chamado, portanto falhas de renderização são propagadas sem transformation.

## Importações
- `super::Step`: Tipo owners, permite acessar o campo `input` dentro do `impl`.
- `std::collections::BTreeMap`: Tipa os mapas de outputs e locals, garantindo ordenação estável das chaves.
