## Resumo
Cria um adaptador OpenRouter com executável, URL base e chave de API opcionais.

## Funcionamento
Converte executável e URL base para `String`, aplica `nonempty` à URL e descarta a chave de API se estiver vazia ou contiver apenas espaços.

## Importações
- `super::OpenRouterAdapter`: Tipo do adaptador criado.
- `super::nonempty::nonempty`: Trata a URL base.
