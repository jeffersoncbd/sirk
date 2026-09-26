## Resumo
Serializa o histórico de uma workflow em um arquivo YAML de texto legível, de forma atômica.

## Funcionamento
Converte o snapshot para YAML e monta o documento prefixado por `TITLE`, iterando os passos: cada um recebe um cabeçalho com `SEPARATOR` e um rótulo (personalizado em `labels` ou gerado como `Step N — <nome do passo>`). Os blocos de cada passo são escritos com seu marcador, e linhas reservadas são escapadas com `\`. A gravação é atômica: escreve em `<arquivo>.log.tmp`, faz `sync_all` e renomeia para o destino, sincronizando o diretório pai; falhas de serialização, I/O ou rename viram `Err(String)`.

## Importações
- `super::{History, SEPARATOR, TITLE, reserved::reserved}`: Tipos e constantes do histórico; `reserved` detecta linhas a escapar.
- `std::fs::{self, File}`: Cria o arquivo temporário e faz o rename final.
- `std::io::Write`: Permite `write_all` no arquivo temporário.
- `serde_yaml::to_string`: Serializa o snapshot da workflow em YAML.
