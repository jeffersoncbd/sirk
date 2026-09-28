## Resumo
Salva o histórico em disco, serializando seus metadados e blocos.

## Funcionamento
Serializa o snapshot em YAML e monta o conteúdo do arquivo, escapando linhas reservadas nos blocos. Grava em um arquivo temporário, sincroniza os dados, renomeia-o para o destino e sincroniza o diretório. Erros de serialização ou de operações de arquivo são convertidos em `String` e retornados.

## Importações
- `super`: Acessa tipos e constantes do histórico.
- `reserved`: Identifica linhas que precisam ser escapadas.
- `std::fs`: Cria, renomeia e sincroniza arquivos.
- `std::io::Write`: Grava os dados no arquivo temporário.
- `serde_yaml`: Serializa o snapshot em YAML.
