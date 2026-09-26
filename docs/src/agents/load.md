## Resumo
Carrega um agente a partir de um arquivo Markdown nomeado por seu ID dentro de um diretório.

## Funcionamento
Valida o `id` com `valid_id`, rejecting-o com mensagem orientativa caso use caracteres fora de letras, dígitos, `_` ou `-`. Monta o caminho `<diretório>/<id>.md`, lê seu conteúdo e converte falhas de I/O em `Err` descrevendo o agente e o caminho. Por fim, delega a análise do texto a `Self::parse`,(prefixando eventuais erros com o caminho do arquivo, de forma que toda falha de leitura ou de parse vire um `String` descritivo em vez de propagar o erro original.

## Importações
- `Agent`: tipo归来 devolvido, alvo do método `impl`.
- `valid_id`: valida o formato do identificador antes de acessar o disco.
- `fs`: lê o conteúdo bruto do arquivo `.md` em memória.
- `Path`: representa o diretório base e o caminho final montado.
