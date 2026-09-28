## Resumo
Cria um `OllamaWebAdapter` com os valores fornecidos e normaliza os campos opcionais.

## Funcionamento
Converte `executable` e `base_url` em `String`, aplica `nonempty` à URL e descarta a chave de API se estiver vazia ou contiver apenas espaços. Retorna a instância configurada.

## Importações
- `super::OllamaWebAdapter`: Tipo de adaptador instanciado.
- `nonempty::nonempty`: Converte a URL em valor opcional.
