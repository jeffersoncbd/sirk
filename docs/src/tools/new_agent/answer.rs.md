## Resumo
Solicita uma resposta até receber um valor não vazio.

## Funcionamento
Faz a pergunta repetidamente e retorna a resposta original quando ela contém algo além de espaços em branco. Se `input.ask` falhar, propaga o erro.

## Importações
- `crate::input::UserInput`: Interface usada para solicitar respostas.
