## Resumo
Construtor da struct `OpenCodeAdapter`, armazenando o caminho do executável do OpenCode.

## Funcionamento
Recebe qualquer valor implementável como `String` (como `&str` ou `String`) e o converte via `Into::String` para o campo `executable` da struct, retornando uma instância pronta para uso. Não há validações nem efeitos colaterais; a conversão pode falhar apenas em tempo de compilação caso o tipo não implemente o trait.

## Importações
- `super::OpenCodeAdapter`: Struct do adapter开放式 ao qual o método é associado.
