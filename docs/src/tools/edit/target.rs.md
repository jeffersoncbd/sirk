## Resumo
Resolve e valida o caminho de destino de uma edição dentro do diretório de execução.

## Funcionamento
Canonicaliza o diretório raiz e verifica o caminho solicitado. Aceita arquivos regulares existentes, rejeita links simbólicos e caminhos fora da raiz. Se o arquivo não existir e `allow_missing` for verdadeiro, exige que o diretório pai exista e esteja dentro da raiz. Retorna o caminho resolvido ou uma mensagem de erro.

## Importações
- `std::fs`: consulta metadados do caminho.
- `std::path`: manipula e canonicaliza caminhos.
