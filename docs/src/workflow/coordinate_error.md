## Resumo
Gera a mensagem de erro usada quando uma coordenada de workflow não renderiza para um inteiro positivo.

## Funcionamento
Recebe o nome do campo (`&str`) e interpola-o em uma string fixa via `format!`, sempre devolvendo uma `String` — não há `Result`/`Option` nem validações dentro da função. A mensagem segue o padrão `"EDIT {name} must render to a positive integer"`, indicando que o valor é gerado por um comando `EDIT` cujo resultado seria usado como coordenada numérica; a verificação do valor em si ocorre onde a função é chamada. Não há efeitos colaterais.

## Importações
- `std::format!`: macro da std para interpolar `name` na mensagem de erro.
