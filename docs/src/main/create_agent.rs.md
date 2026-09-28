## Resumo
Cria um agente no diretório atual e informa o caminho criado.

## Funcionamento
Obtém o diretório atual, convertendo falhas em `String`, e chama a criação do agente com entrada pelo terminal. Erros são propagados; em caso de sucesso, exibe o caminho e retorna `Ok(())`.

## Importações
- `std::env`: Obtém o diretório atual.
