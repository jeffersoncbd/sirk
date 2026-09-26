## Resumo
Define o contrato de interação com o usuário para perguntas iniciais e esclarecimentos do agente.

## Funcionamento
A trait `UserInput` exige a implementação de `ask`, que recebe uma pergunta e retorna a resposta como `Result<String, String>` — `Err` carrega a mensagem de falha. `await_confirmation` é um método padrão derivado de `ask`: descarta a resposta e converte o resultado em `Result<(), ()>`, permitindo confirmar ações sem que o implementador precise de lógica própria. Recebe `&mut self`, permitindo que implementações armazenem estado (histórico, cache, io).

## Importações
- `std::result::Result`: tipo de retorno das operações de entrada.
