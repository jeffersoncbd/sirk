## Resumo
Implementa `Display` para `HarnessError`, formatando cada variante como mensagem legível.

## Funcionamento
O `fmt` faz `match` sobre as variantes de `HarnessError` e escreve no `formatter` textos que incluem o adapter afetado: `UnsupportedOption` e `InvalidResponse` interpolando campos via `write!`, `MissingModel` e `InvalidConfiguration` montando frases fixas com os detalhes da variante. O retorno é `fmt::Result`, propagando erros do escritor sem tratamento adicional.

## Importações
- `super::HarnessError`: Enum de erros do harness, cujas variantes são formatadas.
- `std::fmt`: Fornece `Display`, `Formatter` e `Result` usados na assinatura.
