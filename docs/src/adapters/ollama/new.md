## Resumo
Cria uma nova instância do adaptador Ollama com o caminho do executável informado.

## Funcionamento
Constrói `OllamaAdapter` convertendo o argumento genérico (`impl Into<String>`) em `String` e armazenando-o no campo `executable`. Não realiza validações, I/O nem retorna `Result`/`Option`, portanto não há tratamento de erros; a responsabilidade de verificar se o executável existe fica a cargo de quem chama.

## Importações
- `super::OllamaAdapter`: Tipo do próprio adaptador, usado como retorno do construtor.
