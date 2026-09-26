## Resumo
Pede uma resposta ao usuário até que o texto digitado não seja vazio.

## Funcionamento
Repete `input.ask(question)`, propagando erros com `?`; devolve `Ok(value)` quando `value.trim()` não está vazio, descartando a string original sem alteração. Entradas em branco disparam nova pergunta, logo o loop só encerra com sucesso.

## Importações
- `crate::input::UserInput`: Trait que abstrai a leitura da resposta do usuário.
