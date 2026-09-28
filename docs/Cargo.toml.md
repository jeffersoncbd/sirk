## Resumo
Configura o pacote `sirk`, suas bibliotecas e dependências.

## Funcionamento
Define a biblioteca em `src/lib.rs`, o executável em `src/main.rs` e as dependências usadas pelo projeto; não contém uma função principal.

## Importações
- `serde`: serialização e desserialização
- `serde_yaml`: suporte a YAML
- `serde_json`: suporte a JSON
- `dotenvy`: leitura de variáveis de ambiente
- `sha2`: funções de hash SHA-2
- `sirk-sdk`: SDK local do projeto
- `similar`: comparação de textos
- `tiny_http`: servidor HTTP leve
