## Resumo
Lê o arquivo `.env` de um diretório e retorna o valor de uma variável específica, se presente e não vazia.

## Funcionamento
Monta o caminho `<directory>/.env` e usa `dotenvy::from_path_iter` para iterar sobre os pares chave/valor. Se o arquivo não existir (`ErrorKind::NotFound`), retorna `Ok(None)` em vez de propagar erro — ausência de `.env` é um caso normal. Quaisquer outros erros de leitura viram `HarnessError::InvalidConfiguration` identificando o adapter e o caminho; o mesmo vale para pares malformados durante o parsing. Percorre as variáveis e, ao encontrar `key`, devolve `Some` apenas se o valor não for vazio (via `nonempty`), retornando `Ok(None)` caso contrário. Não há escrita em disco: o efeito colateral é apenas a leitura do arquivo.

## Importações
- `super::OpenRouterAdapter`: Tipo dono do método, fornece `id()` para contexto de erro.
- `super::nonempty::nonempty`: Converte string possivelmente vazia em `Option<String>`.
- `crate::harness::HarnessAdapter`: Trait que expõe `id()` usado nos erros.
- `crate::harness::HarnessError`: Variante `InvalidConfiguration` para falhas de leitura/parsing.
- `std::io`: Usado para comparar `io::ErrorKind::NotFound` e tratar `.env` ausente.
- `std::path::Path`: Recebe o diretório base e monta o caminho do arquivo `.env`.
- `dotenvy`: Lê e faz o parse do arquivo `.env` em pares chave/valor.
