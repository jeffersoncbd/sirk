## Resumo
Atalho que monta um `edit_request` delegando a `edit_request_with` com a flag `false`.

## Funcionamento
Encaminha os `outputs` e os `locals` recebidos (opcionais) para `edit_request_with`, fixando o último parâmetro booleano em `false`. O `Result` e a mensagem de erro (`String`) são propagados sem tratamento, delegando toda validação e efeito colateral ao método chamado.

## Importações
- `super::Step`: tipo receptor dos métodos de etapa do workflow.
- `std::collections::BTreeMap`: dicionário ordenado de saídas e variáveis locais.
- `crate::tools::edit::Request`: estrutura da requisição de edição retornada.
