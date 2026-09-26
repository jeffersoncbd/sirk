## Resumo
Detecta se o último bloco de um histórico é uma resposta a uma pergunta de tool, devolvendo o par (nome_da_tool, argumentos).

## Funcionamento
A função aplica um pattern match de slice sobre `blocks`, exigindo que o último elemento seja `Block::Input(answer)` e capturando o bloco imediatamente anterior como `previous`; historiais vazios, terminados em output ou com menos de dois blocos retornam `None`. Em seguida, valida a regra de negócio: só há request de tool quando o bloco anterior é `Block::Ask` ou um `Block::Output` cujo texto retorna algum valor em `question(...)`. Se válido, delega a `crate::tools::request(answer)`, repassando a resposta como `Option<(&str, &str)>` com nome e argumentos da tool; caso contrário retorna `None`. Não há efeitos colaterais — apenas leitura e pattern matching sobre o slice borrowado.

## Importações
- `super::super::question::question`: Extrai a pergunta de um output, definindo se a tool é acionada.
