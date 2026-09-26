## Resumo
Gera o hash SHA-256 hexadecimal de um conteúdo textual.

## Funcionamento
A função recebe um `&str`, converte para bytes UTF-8 e devolve uma `String` com o digest SHA-256 formatado em hexadecimal minúsculo. Não há validações nem erros: sempre retorna um valor de tamanho fixo (64 caracteres), sem efeitos colaterais.

## Importações
- `sha2::{Digest, Sha256}`:-trait `Digest` e struct `Sha256` para calcular o resumo criptográfico.
