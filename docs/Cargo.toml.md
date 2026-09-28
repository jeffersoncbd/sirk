## Resumo
Configura o pacote `sirk`, suas bibliotecas, dependências e executável.

## Funcionamento
Define metadados do pacote, aponta a biblioteca para `src/lib.rs`, o executável para `src/main.rs` e declara as dependências do projeto.

## Importações
- `serde`: serialização e desserialização
- `serde_yaml`: suporte a YAML
- `serde_json`: suporte a JSON
- `dotenvy`: leitura de variáveis de ambiente
- `sha2`: funções de hash SHA-2
- `sirk-sdk`: SDK local do projeto
- `similar`: comparação de textos
- `tiny_http`: servidor HTTP leve
