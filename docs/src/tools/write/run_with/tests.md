## Resumo
Escreve um arquivo dentro do diretório do projeto, criando pastas intermediárias conforme necessário.

## Funcionamento
Recebe o diretório base, caminho relativo, conteúdo e uma flag `force`. Valida que o caminho não sai do projeto, não é um diretório nem um symlink. Se o arquivo já existe, exige `force: true` para sobrescrevê-lo, caso contrário retorna `Result` com erro. Possui efeito colateral ao criar diretórios e gravar o arquivo no sistema de arquivos.

## Importações
- `super::*`: Fornece a função principal de escrita.
- `crate::tools::write::write`: Implementa a versão simplificada da função.
- `std::time::{SystemTime, UNIX_EPOCH}`: Gera nome único para diretórios temporários.
