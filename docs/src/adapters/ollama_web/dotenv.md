## Resumo
Lê uma chave específica do arquivo `.env` de um diretório e retorna seu valor não vazio, se existir.

## Funcionamento
Monta o caminho `<directory>/.env` e usa `dotenvy::from_path_iter` para iterar sobre as variáveis. Se o arquivo não existir (`ErrorKind::NotFound`), retorna `Ok(None)` em vez de erro; qualquer outra falha de leitura vira `HarnessError::InvalidConfiguration` com o caminho e a causa. Durante a iteração, erros de parse de uma linha também viram `InvalidConfiguration`. Ao encontrar a chave correspondente, devolve `Ok(Some(valor))` passando o valor por `nonempty` (vazio/nulo vira `None`); se o loop terminar sem match, retorna `Ok(None)`. A busca encerra na primeira ocorrência da chave. Não há efeitos colaterais: apenas leitura do disco.

## Importações
- `super::OllamaWebAdapter`: Tipo dono do método, usado em `self.id()` para identificar o adaptador nos erros.
- `super::nonempty::nonempty`: Converte string vazia em `Option::None`.
- `crate::harness::HarnessAdapter`: Trait que fornece `id()` para composing a mensagem de erro.
- `crate::harness::HarnessError`: Tipo de erro retornado em falhas de leitura ou parse.
- `std::io`: Necessário para comparar `io::ErrorKind::NotFound` na falha de leitura.
- `std::path::Path`: Representa o diretório base usado para montar o caminho do `.env`.
