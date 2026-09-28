## Resumo
Adiciona ao Git todas as alterações do projeto no diretório informado.

## Funcionamento
Localiza o projeto a partir de `directory`; se essa etapa falhar, retorna o erro. Em seguida executa `git add --all -- .` no diretório do projeto e retorna `Ok(())` se o comando for bem-sucedido.

## Importações
- `super::project::project`: Localiza o projeto associado ao diretório.
- `super::run::run`: Executa o comando Git e propaga erros.
- `std::path::Path`: Representa o diretório recebido.
